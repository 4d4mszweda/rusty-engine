use cgmath::{Matrix4, Point3, Rad, Vector3};

pub struct Camera {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
}

impl Camera {
    pub fn new(radius: f32, theta: f32, phi: f32) -> Self {
        Camera { radius, theta, phi }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let eye = spherical_to_cartesian(self.radius, self.theta, self.phi);
        let center = Point3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        Matrix4::look_at(eye, center, up)
    }

    pub fn proj_matrix(&self, aspect: f32) -> Matrix4<f32> {
        cgmath::perspective(Rad(45.0f32.to_radians()), aspect, 0.1, 100.0)
    }
}

fn spherical_to_cartesian(r: f32, theta: f32, phi: f32) -> Point3<f32> {
    let x = r * theta.sin() * phi.cos();
    let y = r * theta.cos();
    let z = r * theta.sin() * phi.sin();
    Point3::new(x, y, z)
}
