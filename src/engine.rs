use cgmath::{Matrix4, Point3, Vector3};
use egui_glow::glow;
use glfw::{Action, Context, Key};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::Receiver;

use crate::camera::{Camera, CameraMode};
use crate::debug;
use crate::glcontext;
use crate::gui::Gui;
use crate::input;
use crate::mesh::Mesh;
use crate::scene_object::SceneObject;
use crate::shader::{Program, SpecModel};
use crate::textures::Texture;

// TODO gui

pub struct Engine {
    glfw: glfw::Glfw,                           // CONTEXT
    window: glfw::Window,                       // WINDOW CONTEXT
    events: Receiver<(f64, glfw::WindowEvent)>, // KEYMAPS EVENTS
    input: input::Input,                        // INPUT
    program: Program,                           // SHADERS
    objects: Vec<SceneObject>,                  // OBJECTS ON THE SCENE
    camera: Camera,                             // CAMERA
    last_time: f32,                             // JUST TIME
    gui: Gui,                                   // GUI CONTEXT
}

impl Engine {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        // INIT GLFW
        let mut glfw = glcontext::init_glfw();

        // INIT WINDOW
        let (mut window, events) = glcontext::create_window(&mut glfw, width, height, title);
        window.make_current();
        glcontext::init_gl(&mut window);

        // SHADERS
        let program = Program::from_files("assets/shaders/basic.vert", "assets/shaders/basic.frag");
        program.use_program();
        program.set_int("u_diffuse", 0);

        let glow_ctx = unsafe {
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)
        };
        let glow_ctx = Arc::new(glow_ctx);

        // GUI
        let gui = Gui::new(glow_ctx.clone());
        let last_time = glfw.get_time() as f32;

        // CAMERA
        let camera = Camera::new_orbit(
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            12.0,
            0.5,
            0.8,
        );

        // INPUT
        let input = input::Input::new();

        unsafe {
            debug::gl_check_errors(file!(), line!(), "after init");
        }

        Engine {
            glfw,
            window,
            events,
            program,
            objects: Vec::new(),
            camera,
            input,
            last_time,
            gui,
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            let current_time = self.glfw.get_time() as f32;
            let dt = current_time - self.last_time;
            self.last_time = current_time;

            self.glfw.poll_events();

            self.gui.begin_frame();

            let now = current_time;

            for (_, event) in glfw::flush_messages(&self.events) {
                self.gui.on_glfw_event(&self.window, &event);
                self.input.on_event(&event, now);
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    self.window.set_should_close(true);
                }
            }

            let wants_pointer = self.gui.context().wants_pointer_input();

            if self.input.check_rmb_down() {
                if self.input.rmb_down() {
                    self.window.set_cursor_mode(glfw::CursorMode::Disabled);
                } else {
                    self.window.set_cursor_mode(glfw::CursorMode::Normal);
                }
                self.input.lock_rmb();
            }

            if !wants_pointer {
                self.input.setup_input(dt, now, &mut self.camera);
            } else {
                self.input.take_mouse_delta();
                self.input.take_scroll();
            }

            let objects_count = self.objects.len();
            let camera = &mut self.camera;
            let prog = &mut self.program;

            let full_output = self.gui.run(&self.window, current_time as f64, move |ctx| {
                Engine::print_ui(ctx, camera, prog, objects_count);
            });

            self.render(current_time);

            self.gui.paint(&self.window, full_output);

            self.window.swap_buffers();
        }
    }

    fn render(&mut self, time: f32) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthMask(gl::TRUE);
            gl::Disable(gl::BLEND);
            gl::DepthFunc(gl::LESS);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.program.use_program();
        // --- CAMERA POS (MUSI być co klatkę) ---
        let cam_pos = self.camera.eye_position();
        self.program.set_vec3(
            "u_cameraPos",
            &cgmath::Vector3::new(cam_pos.x, cam_pos.y, cam_pos.z),
        );

        // --- lighting toggles (z egui) ---
        self.program
            .set_int("u_useLighting", self.program.light.is_on as i32);
        self.program.set_int(
            "u_specModel",
            match self.program.light.model {
                SpecModel::Phong => 0,
                SpecModel::BlinnPhong => 1,
            },
        );
        // --- światło (też logicznie per frame) ---
        self.program
            .set_vec3("u_light.Position", &cgmath::Vector3::new(2.0, 3.0, 1.0));
        self.program
            .set_vec3("u_light.Ambient", &cgmath::Vector3::new(0.1, 0.1, 0.1));
        self.program
            .set_vec3("u_light.Diffuse", &cgmath::Vector3::new(1.0, 1.0, 1.0));
        self.program
            .set_vec3("u_light.Specular", &cgmath::Vector3::new(1.0, 1.0, 1.0));
        self.program
            .set_vec3("u_light.Attenuation", &cgmath::Vector3::new(1.0, 0.0, 0.0));

        let (width, height) = self.window.get_size();
        let aspect = width as f32 / height as f32;

        let view = self.camera.view_matrix();
        let proj = self.camera.proj_matrix(aspect);

        for obj in &self.objects {
            obj.draw(&self.program, time, &view, &proj);
        }

        unsafe {
            debug::gl_check_errors(file!(), line!(), "end of frame");
        }
    }

    fn print_ui(
        ctx: &egui::Context,
        camera: &mut Camera,
        program: &mut Program,
        objects_count: usize,
    ) {
        egui::Window::new("Debug").show(ctx, |ui| {
            ui.label(format!("Objects: {}", objects_count));
        });

        egui::Window::new("Camera").show(ctx, |ui| {
            ui.label(format!("Mode: {:?}", camera.mode));

            ui.separator();
            ui.label("Set mode:");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(matches!(camera.mode, CameraMode::Orbit), "Orbit")
                    .clicked()
                {
                    camera.mode = CameraMode::Orbit;
                }
                if ui
                    .selectable_label(matches!(camera.mode, CameraMode::Free), "Free")
                    .clicked()
                {
                    camera.mode = CameraMode::Free;
                }
            });
        });

        egui::Window::new("Światło").show(ctx, |ui| {
            ui.checkbox(&mut program.light.is_on, "Włącz oświetlenie");

            ui.separator();
            ui.label("Model specular:");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(matches!(program.light.model, SpecModel::Phong), "Phong")
                    .clicked()
                {
                    program.light.model = SpecModel::Phong;
                }
                if ui
                    .selectable_label(
                        matches!(program.light.model, SpecModel::BlinnPhong),
                        "Blinn-Phong",
                    )
                    .clicked()
                {
                    program.light.model = SpecModel::BlinnPhong;
                }
            });
        });
    }
    pub fn hello_world(&mut self) -> &mut Self {
        let ground_mesh = Rc::new(Mesh::from_obj("assets/models/ground-large.obj"));
        let tree_mesh = Rc::new(Mesh::from_obj("assets/models/palm.obj"));
        let house_mesh = Rc::new(Mesh::from_obj("assets/models/kaktus.obj"));

        let mut objects = Vec::new();

        let ground_tex =
            Rc::new(Texture::from_file("assets/textures/ground.jpg").set_mirrored_repeat());
        let cactus_tex = Rc::new(Texture::from_file("assets/textures/cactus.jpg"));

        // Podłoże
        let ground_model = Matrix4::from_scale(1.0);
        objects.push(
            SceneObject::new(
                ground_mesh.clone(),
                ground_model,
                Vector3::new(0.6, 0.6, 0.6),
                Vector3::new(0.8, 0.8, 0.8),
            )
            .with_ground(true)
            .with_texture(ground_tex.clone(), false),
        );

        // Drzewo – animacja koloru
        let tree_model = Matrix4::from_translation(cgmath::Vector3::new(-3.0, 0.0, -2.0));
        objects.push(
            SceneObject::new(
                tree_mesh.clone(),
                tree_model,
                Vector3::new(0.1, 0.5, 0.1),
                Vector3::new(0.6, 0.8, 0.3),
            )
            .with_color_animation(1.0),
        );

        // Kaktus – statyczny
        let house_model = Matrix4::from_translation(cgmath::Vector3::new(2.0, 0.0, -4.0));
        objects.push(
            SceneObject::new(
                house_mesh.clone(),
                house_model,
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(1.0, 1.0, 1.0),
            )
            .with_texture(cactus_tex.clone(), false),
        );

        self.objects = objects;
        self
    }
}
