use cgmath::{Matrix4, Rad, Vector3};

use egui_glow::glow;
use glfw::{Action, Context, Key};
use rand::Rng;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::Receiver;

use crate::camera::Camera;
use crate::glcontext;
use crate::gui::Gui;
use crate::mesh::Mesh;
use crate::scene_object::SceneObject;
use crate::shader::Program;
use crate::textures::Texture;

pub struct Engine {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    program: Program,
    objects: Vec<SceneObject>,
    camera: Camera,
    last_time: f32,

    gui: Gui,
}

impl Engine {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glcontext::init_glfw();

        let (mut window, events) = glcontext::create_window(&mut glfw, width, height, title);
        window.make_current();

        glcontext::init_gl(&mut window);

        // let program = Program::new(shaders::basic::VERT, shaders::basic::FRAG);
        let program = Program::from_files("assets/shaders/basic.vert", "assets/shaders/basic.frag");
        program.use_program();
        program.set_int("u_diffuse", 0);

        let glow_ctx = unsafe {
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)
        };
        let glow_ctx = Arc::new(glow_ctx);

        let gui = Gui::new(glow_ctx.clone());

        // Ładowanie siatek
        let ground_mesh = Rc::new(Mesh::from_obj("assets/models/ground-large.obj"));
        let tree_mesh = Rc::new(Mesh::from_obj("assets/models/palm.obj"));
        let house_mesh = Rc::new(Mesh::from_obj("assets/models/kaktus.obj"));
        let rock_mesh = Rc::new(Mesh::from_obj("assets/models/rock.obj"));
        let flower_mesh = Rc::new(Mesh::quad());

        let mut objects = Vec::new();
        // let flower_positions = [
        //     Vector3::new(-2.0, 0.0, 1.0),
        //     Vector3::new(-1.0, 0.0, 3.0),
        //     Vector3::new(0.0, 0.0, 2.5),
        //     Vector3::new(1.0, 0.0, 1.5),
        //     Vector3::new(2.0, 0.0, 3.0),
        // ];

        // --- Meshe ---
        let flower_tex = Rc::new(Texture::from_file("assets/textures/flower32bit.png"));
        let ground_tex = Rc::new(Texture::from_file("assets/textures/ground.jpg"));
        ground_tex.set_mirrored_repeat();
        let cactus_tex = Rc::new(Texture::from_file("assets/textures/cactus.jpg"));
        let rock_tex = Rc::new(Texture::from_file("assets/textures/rock.jpg"));

        let mut rng = rand::thread_rng();
        let flower_count = 120;

        for _ in 0..flower_count {
            // ZAKRES X,Z dopasuj do rozmiaru swojego ground-large.obj
            // Załóżmy np. ziemia ~ od -8 do 8 w obu osiach:
            let x = rng.gen_range(-8.0, 8.0);
            let z = rng.gen_range(-8.0, 8.0);

            // losowa skala kwiatka
            let scale = rng.gen_range(0.4, 1.0);

            // losowy obrót wokół Y
            let rotation = rng.gen_range(0.0, std::f32::consts::TAU);

            // pierwszy quad
            let model1 = Matrix4::from_translation(Vector3::new(x, 0.0, z))
                * Matrix4::from_angle_y(Rad(rotation))
                * Matrix4::from_scale(scale);

            objects.push(
                SceneObject::new(
                    flower_mesh.clone(),
                    model1,
                    Vector3::new(1.0, 1.0, 1.0),
                    Vector3::new(1.0, 1.0, 1.0),
                )
                .with_texture(flower_tex.clone(), true),
            );

            // drugi quad – obrócony o 90° względem pierwszego
            let model2 = Matrix4::from_translation(Vector3::new(x, 0.0, z))
                * Matrix4::from_angle_y(Rad(rotation + std::f32::consts::FRAC_PI_2))
                * Matrix4::from_scale(scale);

            objects.push(
                SceneObject::new(
                    flower_mesh.clone(),
                    model2,
                    Vector3::new(1.0, 1.0, 1.0),
                    Vector3::new(1.0, 1.0, 1.0),
                )
                .with_texture(flower_tex.clone(), true),
            );
        }
        //    for pos in &flower_positions {
        //        // pierwszy quad
        //        let model1 = Matrix4::from_translation(*pos) * Matrix4::from_scale(0.7);
        //        objects.push(
        //            SceneObject::new(
        //                flower_mesh.clone(),
        //                model1,
        //                Vector3::new(1.0, 1.0, 1.0),
        //                Vector3::new(1.0, 1.0, 1.0),
        //            )
        //            .with_texture(flower_tex.clone(), true),
        //        );

        //        // drugi quad – obrócony o 90 stopni wokół Y
        //        let model2 = Matrix4::from_translation(*pos)
        //            * Matrix4::from_angle_y(Rad(std::f32::consts::FRAC_PI_2))
        //            * Matrix4::from_scale(0.7);
        //        objects.push(
        //            SceneObject::new(
        //                flower_mesh.clone(),
        //                model2,
        //                Vector3::new(1.0, 1.0, 1.0),
        //                Vector3::new(1.0, 1.0, 1.0),
        //            )
        //            .with_texture(flower_tex.clone(), true),
        //        );
        //    }
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

        // Dom – statyczny
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

        // Skała 1 – obrót
        let rock_model1 = Matrix4::from_translation(cgmath::Vector3::new(-1.0, 0.0, 2.0))
            * Matrix4::from_scale(0.8);
        objects.push(
            SceneObject::new(
                rock_mesh.clone(),
                rock_model1,
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(1.0, 1.0, 1.0),
            )
            .with_rotation(cgmath::Vector3::new(0.0, 1.0, 0.0), 1.0)
            .with_texture(rock_tex.clone(), false),
        );

        // Skała 2 – obrót + kolor
        let rock_model2 = Matrix4::from_translation(cgmath::Vector3::new(3.0, 0.0, 1.0))
            * Matrix4::from_scale(0.5);
        objects.push(
            SceneObject::new(
                rock_mesh.clone(),
                rock_model2,
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(1.0, 1.0, 1.0),
            )
            .with_rotation(cgmath::Vector3::new(0.0, 1.0, 0.0), 2.0)
            .with_color_animation(2.0)
            .with_texture(rock_tex.clone(), false),
        );

        let camera = Camera::new(12.0, 0.5, 0.8);

        let last_time = glfw.get_time() as f32;

        Engine {
            glfw,
            window,
            events,
            program,
            objects,
            camera,
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

            // 1. Nowa klatka egui
            self.gui.begin_frame();

            // 2. Obsługa eventów z glfw
            for (_, event) in glfw::flush_messages(&self.events) {
                // przekazujemy do egui
                self.gui.on_glfw_event(&self.window, &event);

                // równolegle logika silnika (ESC itd.)
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => {}
                }
            }

            // 3. Input kamery – możesz rozważyć:
            //    jeśli gui.ctx().wants_pointer_input() => nie ruszaj kamery
            self.handle_input(dt);

            let last_time = self.last_time;
            let objects_count = self.objects.len();

            let full_output = self.gui.run(&self.window, current_time as f64, move |ctx| {
                Engine::build_ui(ctx, last_time, objects_count);
            });

            self.render(current_time);

            // 6. Render egui na wierzchu
            self.gui.paint(&self.window, full_output);

            // 7. Swap buffers
            self.window.swap_buffers();
        }
    }

    fn handle_input(&mut self, dt: f32) {
        crate::input::process_input(&mut self.window, dt, &mut self.camera);
    }

    fn render(&mut self, time: f32) {
        unsafe {
            // Przywróć stan dla 3D
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthMask(gl::TRUE);
            gl::Disable(gl::BLEND);
            gl::DepthFunc(gl::LESS);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = self.window.get_size();
        let aspect = width as f32 / height as f32;

        let view = self.camera.view_matrix();
        let proj = self.camera.proj_matrix(aspect);

        for obj in &self.objects {
            obj.draw(&self.program, time, &view, &proj);
        }
    }

    fn build_ui(ctx: &egui::Context, last_time: f32, objects_count: usize) {
        egui::Window::new("Debug").show(ctx, |ui| {
            ui.label(format!("Time: {:.2}", last_time));
            ui.label(format!("Objects: {}", objects_count));
        });

        egui::Window::new("Camera").show(ctx, |ui| {
            ui.label("Tu sobie później dodasz suwaki do kamery");
            ui.label("(np. radius/yaw/pitch, zależnie co masz w Camera)");
        });
    }
}
