pub mod sky_box;
use std::rc::Rc;

use cgmath::{InnerSpace, Matrix4, Rad, Vector3};

use crate::mesh::Mesh;
use crate::program::Program;
use crate::textures::Texture;

pub struct SceneObject {
    // BASE
    pub mesh: Rc<Mesh>,
    pub base_model: Matrix4<f32>,
    pub base_color: Vector3<f32>,

    // MATERIAŁ
    pub material: Material,

    // ANIMACJE
    pub animate_rotation: bool,
    pub rotation_axis: Vector3<f32>,
    pub rotation_speed: f32,
    pub is_ground: bool,

    // TEKSTURY
    pub texture: Option<Rc<Texture>>,
    pub use_texture: bool,
    pub alpha_cutout: bool,
}

#[allow(dead_code)]
impl SceneObject {
    pub fn new(mesh: Rc<Mesh>, base_model: Matrix4<f32>, base_color: Vector3<f32>) -> Self {
        SceneObject {
            mesh,
            base_model,
            material: Material::default(),
            base_color,
            animate_rotation: false,
            rotation_axis: Vector3::new(0.0, 1.0, 0.0),
            rotation_speed: 0.0,
            is_ground: false,
            texture: None,
            use_texture: false,
            alpha_cutout: false,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_rotation(mut self, axis: Vector3<f32>, speed: f32) -> Self {
        self.animate_rotation = true;
        self.rotation_axis = axis;
        self.rotation_speed = speed;
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

        program.set_mat4("u_model", &model);
        program.set_mat4("u_view", view);
        program.set_mat4("u_proj", proj);
        program.set_vec3("u_color", &self.base_color);
        program.set_int("u_is_ground", if self.is_ground { 1 } else { 0 });
        program.set_vec3("u_material.Ambient", &self.material.ambient);
        program.set_vec3("u_material.Diffuse", &self.material.diffuse);
        program.set_vec3("u_material.Specular", &self.material.specular);
        program.set_float("u_material.Shininess", self.material.shininess);

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

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vector3::new(0.2, 0.2, 0.2),
            diffuse: Vector3::new(1.0, 1.0, 1.0),
            specular: Vector3::new(0.5, 0.5, 0.5),
            shininess: 32.0,
        }
    }
}

impl Material {
    pub fn glossy() -> Self {
        Self {
            ambient: Vector3::new(0.25, 0.25, 0.25),
            diffuse: Vector3::new(1.0, 1.0, 1.0),
            specular: Vector3::new(1.0, 1.0, 1.0),
            shininess: 128.0,
        }
    }

    pub fn matte() -> Self {
        Self {
            ambient: Vector3::new(0.35, 0.35, 0.35),
            diffuse: Vector3::new(1.0, 1.0, 1.0),
            specular: Vector3::new(0.05, 0.05, 0.05),
            shininess: 8.0,
        }
    }
}
