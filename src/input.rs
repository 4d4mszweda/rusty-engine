use glfw::{Action, Key};

use crate::camera::Camera;

pub fn process_input(window: &mut glfw::Window, dt: f32, camera: &mut Camera) {
    let rot_speed = 1.5;
    let zoom_speed = 10.0;

    // W/S – góra/dół
    if window.get_key(Key::W) == Action::Press {
        camera.theta -= rot_speed * dt;
    }
    if window.get_key(Key::S) == Action::Press {
        camera.theta += rot_speed * dt;
    }

    // A/D – obrót wokół sceny
    if window.get_key(Key::A) == Action::Press {
        camera.phi -= rot_speed * dt;
    }
    if window.get_key(Key::D) == Action::Press {
        camera.phi += rot_speed * dt;
    }

    // + / - – zoom
    if window.get_key(Key::Equal) == Action::Press {
        camera.radius -= zoom_speed * dt;
    }
    if window.get_key(Key::Minus) == Action::Press {
        camera.radius += zoom_speed * dt;
    }

    // ograniczenia kamery
    if camera.radius < 3.0 {
        camera.radius = 3.0;
    }
    if camera.radius > 50.0 {
        camera.radius = 50.0;
    }

    let eps = 0.1;
    if camera.theta < eps {
        camera.theta = eps;
    }
    if camera.theta > std::f32::consts::PI - eps {
        camera.theta = std::f32::consts::PI - eps;
    }
}
