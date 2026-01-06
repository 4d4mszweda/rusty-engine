use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

#[derive(Clone, Copy, Debug)]
pub enum CameraMode {
    Orbit,
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CameraAction {
    ZoomIn,
    ZoomOut,
    W,
    S,
    A,
    D,
    ToggleMode,
}

pub struct Camera {
    pub mode: CameraMode,

    // wspólne
    pub fov_y_deg: f32,
    pub near: f32,
    pub far: f32,

    // ORBIT
    pub target: Point3<f32>,
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,

    // FREE (FPS)
    pub position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub world_up: Vector3<f32>,
}

#[allow(dead_code)]
impl Camera {
    pub fn new_orbit(target: Point3<f32>, radius: f32, theta: f32, phi: f32) -> Self {
        Self {
            mode: CameraMode::Orbit,
            fov_y_deg: 45.0,
            near: 0.1,
            far: 100.0,

            target,
            radius,
            theta,
            phi,

            position: Point3::new(0.0, 0.0, radius),
            yaw: 0.0,
            pitch: 0.0,
            world_up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn input_event(&mut self, dt: f32, action: CameraAction) {
        let rot_speed = 1.5 * dt;
        let zoom_speed = 10.0 * dt;
        let move_speed = 10.0 * dt;

        if action == CameraAction::ToggleMode {
            self.toggle_mode();
            return;
        }

        match self.mode {
            CameraMode::Orbit => match action {
                CameraAction::W => self.orbit_rotate(0.0, -rot_speed),
                CameraAction::S => self.orbit_rotate(0.0, rot_speed),
                CameraAction::A => self.orbit_rotate(-rot_speed, 0.0),
                CameraAction::D => self.orbit_rotate(rot_speed, 0.0),
                CameraAction::ZoomIn => self.orbit_zoom(-zoom_speed),
                CameraAction::ZoomOut => self.orbit_zoom(zoom_speed),
                _ => {}
            },
            CameraMode::Free => {
                let mut f = 0.0;
                let mut r = 0.0;
                let mut u = 0.0;

                match action {
                    CameraAction::ZoomIn => f += move_speed,
                    CameraAction::ZoomOut => f -= move_speed,
                    CameraAction::D => r += move_speed,
                    CameraAction::A => r -= move_speed,
                    CameraAction::W => u += move_speed,
                    CameraAction::S => u -= move_speed,
                    _ => {}
                }

                self.free_move(f, r, u);
            }
        }
    }

    pub fn set_target(&mut self, target: Point3<f32>) {
        self.target = target;
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::Orbit => CameraMode::Free,
            CameraMode::Free => CameraMode::Orbit,
        };
    }

    pub fn proj_matrix(&self, aspect: f32) -> Matrix4<f32> {
        cgmath::perspective(
            Rad(self.fov_y_deg.to_radians()),
            aspect,
            self.near,
            self.far,
        )
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        match self.mode {
            CameraMode::Orbit => {
                let eye = orbit_eye(self.target, self.radius, self.theta, self.phi);
                Matrix4::look_at(eye, self.target, self.world_up)
            }
            CameraMode::Free => {
                let forward = free_forward(self.yaw, self.pitch);
                let center = self.position + forward;
                Matrix4::look_at(self.position, center, self.world_up)
            }
        }
    }

    // --- ORBIT controls ---
    pub fn orbit_rotate(&mut self, d_phi: f32, d_theta: f32) {
        self.phi += d_phi;
        self.theta += d_theta;

        let eps = 0.001;
        self.theta = self.theta.clamp(eps, std::f32::consts::PI - eps);
    }

    pub fn orbit_zoom(&mut self, dr: f32) {
        self.radius = (self.radius + dr).max(0.05);
    }

    // --- FREE controls ---
    pub fn free_look(&mut self, d_yaw: f32, d_pitch: f32) {
        self.yaw += d_yaw;
        self.pitch += d_pitch;

        let limit = std::f32::consts::FRAC_PI_2 - 0.001;
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn free_move(&mut self, forward: f32, right: f32, up: f32) {
        let f = free_forward(self.yaw, self.pitch);
        let r = f.cross(self.world_up).normalize();
        let u = self.world_up;

        self.position += f * forward;
        self.position += r * right;
        self.position += u * up;
    }
}

fn orbit_eye(target: Point3<f32>, r: f32, theta: f32, phi: f32) -> Point3<f32> {
    let x = r * theta.sin() * phi.cos();
    let y = r * theta.cos();
    let z = r * theta.sin() * phi.sin();
    Point3::new(target.x + x, target.y + y, target.z + z)
}

fn free_forward(yaw: f32, pitch: f32) -> Vector3<f32> {
    let x = yaw.cos() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.sin() * pitch.cos();
    Vector3::new(x, y, z).normalize()
}
