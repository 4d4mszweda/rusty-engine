use std::rc::Rc;

use cgmath::{InnerSpace, Matrix4, Rad, Vector3};

use crate::mesh::Mesh;
use crate::shader::Program;
use crate::textures::Texture;

pub struct SceneObject {
    pub mesh: Rc<Mesh>,
    pub base_model: Matrix4<f32>,
    pub base_color1: Vector3<f32>,
    pub base_color2: Vector3<f32>,

    // ANIMACJE
    pub animate_rotation: bool,
    pub animate_color: bool,
    pub rotation_axis: Vector3<f32>,
    pub rotation_speed: f32,
    pub color_speed: f32,
    pub is_ground: bool,

    // TEKSTURY
    pub texture: Option<Rc<Texture>>,
    pub use_texture: bool,
    pub alpha_cutout: bool,
}

impl SceneObject {
    pub fn new(
        mesh: Rc<Mesh>,
        base_model: Matrix4<f32>,
        base_color1: Vector3<f32>,
        base_color2: Vector3<f32>,
    ) -> Self {
        SceneObject {
            mesh,
            base_model,
            base_color1,
            base_color2,
            animate_rotation: false,
            animate_color: false,
            rotation_axis: Vector3::new(0.0, 1.0, 0.0),
            rotation_speed: 0.0,
            color_speed: 0.0,
            is_ground: false,
            texture: None,
            use_texture: false,
            alpha_cutout: false,
        }
    }

    pub fn with_rotation(mut self, axis: Vector3<f32>, speed: f32) -> Self {
        self.animate_rotation = true;
        self.rotation_axis = axis;
        self.rotation_speed = speed;
        self
    }

    pub fn with_color_animation(mut self, speed: f32) -> Self {
        self.animate_color = true;
        self.color_speed = speed;
        self
    }

    pub fn with_ground(mut self, is_ground: bool) -> Self {
        self.is_ground = is_ground;
        self
    }

    pub fn with_texture(mut self, texture: Rc<Texture>, alpha_cutout: bool) -> Self {
        self.use_texture = true;
        self.alpha_cutout = alpha_cutout;
        self.texture = Some(texture);
        self
    }

    pub fn draw(&self, program: &Program, time: f32, view: &Matrix4<f32>, proj: &Matrix4<f32>) {
        program.use_program();

        let mut model = self.base_model;
        if self.animate_rotation {
            let angle = time * self.rotation_speed;
            let rot = Matrix4::from_axis_angle(self.rotation_axis.normalize(), Rad(angle));
            model = model * rot;
        }

        let mut c1 = self.base_color1;
        let mut c2 = self.base_color2;

        if self.animate_color {
            let t = (time * self.color_speed).sin() * 0.5 + 0.5;
            c1 = self.base_color1 * (1.0 - t) + self.base_color2 * t;
            c2 = self.base_color2 * (1.0 - t) + self.base_color1 * t;
        }

        program.set_mat4("u_model", &model);
        program.set_mat4("u_view", view);
        program.set_mat4("u_proj", proj);
        program.set_vec3("u_color1", &c1);
        program.set_vec3("u_color2", &c2);
        program.set_int("u_is_ground", if self.is_ground { 1 } else { 0 });

        if let Some(tex) = &self.texture {
            tex.bind(0);
            program.set_int("u_use_texture", 1);
            program.set_int("u_alpha_cutout", if self.alpha_cutout { 1 } else { 0 });
        } else {
            program.set_int("u_use_texture", 0);
            program.set_int("u_alpha_cutout", 0);
        }

        self.mesh.draw();
    }
}
