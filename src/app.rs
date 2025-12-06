use cgmath::{Matrix4, Vector3};

use glfw::{Action, Context, Key};

use std::rc::Rc;

use std::sync::mpsc::Receiver;

use crate::camera::Camera;
use crate::glcontext;
use crate::mesh::Mesh;
use crate::scene_object::SceneObject;
use crate::shader::Program;

pub struct App {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    program: Program,
    objects: Vec<SceneObject>,
    camera: Camera,
    last_time: f32,
}

impl App {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glcontext::init_glfw();

        let (mut window, events) = glcontext::create_window(&mut glfw, width, height, title);
        window.make_current();

        glcontext::init_gl(&mut window);

        // let program = Program::new(shaders::basic::VERT, shaders::basic::FRAG);
        let program = Program::from_files("assets/shaders/basic.vert", "assets/shaders/basic.frag");

        // Ładowanie siatek
        let ground_mesh = Rc::new(Mesh::from_obj("assets/models/ground-large.obj"));
        let tree_mesh = Rc::new(Mesh::from_obj("assets/models/palm.obj"));
        let house_mesh = Rc::new(Mesh::from_obj("assets/models/kaktus.obj"));
        let rock_mesh = Rc::new(Mesh::from_obj("assets/models/rock.obj"));

        // Tworzenie obiektów sceny
        let mut objects = Vec::new();

        // Podłoże
        let ground_model = Matrix4::from_scale(1.0);
        objects.push(
            SceneObject::new(
                ground_mesh.clone(),
                ground_model,
                Vector3::new(0.1, 0.4, 0.1),
                Vector3::new(0.4, 0.25, 0.1),
            )
            .with_ground(true),
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
        objects.push(SceneObject::new(
            house_mesh.clone(),
            house_model,
            Vector3::new(0.5, 0.3, 0.2),
            Vector3::new(0.8, 0.6, 0.4),
        ));

        // Skała 1 – obrót
        let rock_model1 = Matrix4::from_translation(cgmath::Vector3::new(-1.0, 0.0, 2.0))
            * Matrix4::from_scale(0.8);
        objects.push(
            SceneObject::new(
                rock_mesh.clone(),
                rock_model1,
                Vector3::new(0.4, 0.4, 0.4),
                Vector3::new(0.7, 0.7, 0.7),
            )
            .with_rotation(cgmath::Vector3::new(0.0, 1.0, 0.0), 1.0),
        );

        // Skała 2 – obrót + kolor
        let rock_model2 = Matrix4::from_translation(cgmath::Vector3::new(3.0, 0.0, 1.0))
            * Matrix4::from_scale(0.5);
        objects.push(
            SceneObject::new(
                rock_mesh.clone(),
                rock_model2,
                Vector3::new(0.2, 0.2, 0.5),
                Vector3::new(0.6, 0.6, 1.0),
            )
            .with_rotation(cgmath::Vector3::new(0.0, 1.0, 0.0), 2.0)
            .with_color_animation(2.0),
        );

        let camera = Camera::new(12.0, 0.5, 0.8);

        let last_time = glfw.get_time() as f32;

        App {
            glfw,
            window,
            events,
            program,
            objects,
            camera,
            last_time,
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            let current_time = self.glfw.get_time() as f32;
            let dt = current_time - self.last_time;
            self.last_time = current_time;

            self.glfw.poll_events();

            self.handle_input(dt);
            self.render(current_time);

            self.window.swap_buffers();

            // obsługa zdarzeń (np. ESC)
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_input(&mut self, dt: f32) {
        crate::input::process_input(&mut self.window, dt, &mut self.camera);
    }

    fn render(&mut self, time: f32) {
        unsafe {
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
}
