use crate::program::Program;
use crate::scene_object::instanced_mesh::InstancedMesh;
use crate::textures::Texture;
use cgmath::{Matrix4, Rad, Vector3};
use rand::{Rng, SeedableRng};

pub struct LeafState {
    pub leaf_program: Program,
    pub leaf_mesh: InstancedMesh,
    pub leaf_texture: Texture,
    pub leaf_system: LeafSystem,
    pub last_time: f32,
}

impl Default for LeafState {
    fn default() -> Self {
        Self {
            leaf_program: Program::from_files(
                "assets/shaders/leaf.vert",
                "assets/shaders/leaf.frag",
            ),
            leaf_mesh: InstancedMesh::from_quad_with_instances(5000),
            leaf_texture: Texture::from_file("assets/textures/snowflake.png"),
            leaf_system: LeafSystem::new(2000),
            last_time: 0.0,
        }
    }
}

impl LeafState {
    pub fn update(&mut self, time: f32) {
        let dt = (time - self.last_time).max(0.0);
        self.last_time = time;
        self.leaf_system.update(dt);
    }

    pub fn draw(&mut self, view: &Matrix4<f32>, proj: &Matrix4<f32>) {
        if !self.leaf_system.enabled {
            return;
        }

        // instanced matrices -> VBO
        let mats = self.leaf_system.build_instance_matrices();
        self.leaf_mesh.update_instances(&mats);

        unsafe {
            gl::Disable(gl::CULL_FACE);
            // przy discard nie potrzebujesz BLEND
        }

        self.leaf_program.use_program();
        self.leaf_program.set_mat4("u_view", view);
        self.leaf_program.set_mat4("u_proj", proj);
        self.leaf_program
            .set_vec3("u_tint", &cgmath::Vector3::new(1.0, 1.0, 1.0));
        self.leaf_program.set_float("u_alphaThreshold", 0.5);

        self.leaf_texture.bind(0);
        self.leaf_program.set_int("u_diffuse", 0);

        self.leaf_mesh.draw_instanced();

        unsafe {
            gl::Enable(gl::CULL_FACE);
        }
    }
}

pub struct Leaf {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub rot: f32,
    pub rot_speed: f32,
    pub scale: f32,
}

pub struct LeafSystem {
    pub leaves: Vec<Leaf>,
    pub area_half: Vector3<f32>, // rozmiar “chmury” liści (x,z) i wysokość startu
    pub floor_y: f32,
    pub enabled: bool,
}

impl LeafSystem {
    pub fn new(count: usize) -> Self {
        let mut sys = Self {
            leaves: Vec::new(),
            area_half: Vector3::new(10.0, 6.0, 10.0),
            floor_y: 0.0,
            enabled: true,
        };
        sys.respawn_many(count, 1234);
        sys
    }

    pub fn respawn_many(&mut self, count: usize, seed: u64) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        self.leaves.clear();
        self.leaves.reserve(count);

        for _ in 0..count {
            self.leaves
                .push(Self::random_leaf(&mut rng, self.area_half, self.floor_y));
        }
    }

    fn random_leaf<R: Rng>(rng: &mut R, area: Vector3<f32>, floor_y: f32) -> Leaf {
        let x = rng.gen_range(-area.x..=area.x);
        let z = rng.gen_range(-area.z..=area.z);
        let y = rng.gen_range(1.0..=area.y);

        let fall = rng.gen_range(0.6..=2.0);
        let drift_x = rng.gen_range(-0.2..=0.2);
        let drift_z = rng.gen_range(-0.2..=0.2);

        Leaf {
            pos: Vector3::new(x, y, z),
            vel: Vector3::new(drift_x, -fall, drift_z),
            rot: rng.gen_range(0.0..=std::f32::consts::TAU),
            rot_speed: rng.gen_range(-3.0..=3.0),
            scale: rng.gen_range(0.05..=0.18),
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.enabled {
            return;
        }

        for leaf in &mut self.leaves {
            leaf.pos += leaf.vel * dt;
            leaf.rot += leaf.rot_speed * dt;

            leaf.vel.x += (leaf.rot * 0.5).sin() * 0.01;
            leaf.vel.z += (leaf.rot * 0.7).cos() * 0.01;

            if leaf.pos.y < self.floor_y - 0.2 {
                leaf.pos.y = 6.0;
            }
        }
    }

    pub fn build_instance_matrices(&self) -> Vec<Matrix4<f32>> {
        let mut mats = Vec::with_capacity(self.leaves.len());
        for l in &self.leaves {
            let t = Matrix4::from_translation(l.pos);
            let r = Matrix4::from_angle_y(Rad(l.rot)); // prosta rotacja
            let s = Matrix4::from_scale(l.scale);
            mats.push(t * r * s);
        }
        mats
    }
}
