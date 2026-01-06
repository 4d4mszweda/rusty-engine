use crate::camera::{Camera, CameraAction, CameraMode};
use glfw::{Action, Key, WindowEvent};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct KeyState {
    is_down: bool,
    pressed_at: f32,
    consumed: bool,
}

impl KeyState {
    fn new() -> Self {
        Self {
            is_down: false,
            pressed_at: 0.0,
            consumed: false,
        }
    }
}

pub struct Input {
    keys: HashMap<Key, KeyState>,
    scroll_y: f32,
    tap_time: f32,
    // mouse look
    prev_rmb_down: bool,
    rmb_down: bool,
    last_cursor: Option<(f32, f32)>,
    mouse_dx: f32,
    mouse_dy: f32,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            scroll_y: 0.0,
            tap_time: 0.2,
            rmb_down: false,
            prev_rmb_down: false,
            last_cursor: None,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
        }
    }

    pub fn on_event(&mut self, event: &WindowEvent, now: f32) {
        match *event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                let ks = self.keys.entry(key).or_insert_with(KeyState::new);
                ks.is_down = true;
                ks.pressed_at = now;
                ks.consumed = false;
            }
            WindowEvent::Key(key, _, Action::Release, _) => {
                if let Some(ks) = self.keys.get_mut(&key) {
                    ks.is_down = false;
                }
            }
            WindowEvent::Scroll(_, y) => {
                self.scroll_y += y as f32;
            }
            WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Press, _) => {
                self.rmb_down = true;
                self.last_cursor = None;
                self.mouse_dx = 0.0;
                self.mouse_dy = 0.0;
            }

            WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Release, _) => {
                self.rmb_down = false;
                self.last_cursor = None;
                self.mouse_dx = 0.0;
                self.mouse_dy = 0.0;
            }
            WindowEvent::CursorPos(x, y) => {
                if !self.rmb_down {
                    self.last_cursor = Some((x as f32, y as f32));
                    return;
                }

                let (x, y) = (x as f32, y as f32);

                if let Some((lx, ly)) = self.last_cursor {
                    self.mouse_dx += x - lx;
                    self.mouse_dy += y - ly;
                }
                self.last_cursor = Some((x, y));
            }
            _ => {}
        }
    }

    pub fn tap(&mut self, key: Key, now: f32) -> bool {
        if let Some(ks) = self.keys.get_mut(&key) {
            if ks.is_down && !ks.consumed {
                let held = now - ks.pressed_at;
                if held <= self.tap_time {
                    ks.consumed = true;
                    return true;
                }
            }
        }
        false
    }

    pub fn hold(&self, key: Key) -> bool {
        self.keys.get(&key).map_or(false, |k| k.is_down)
    }

    pub fn take_scroll(&mut self) -> f32 {
        let y = self.scroll_y;
        self.scroll_y = 0.0;
        y
    }

    pub fn check_rmb_down(&self) -> bool {
        self.rmb_down != self.prev_rmb_down
    }

    pub fn rmb_down(&self) -> bool {
        self.rmb_down
    }

    pub fn lock_rmb(&mut self) {
        self.prev_rmb_down = self.rmb_down;
    }

    pub fn take_mouse_delta(&mut self) -> (f32, f32) {
        let dx = self.mouse_dx;
        let dy = self.mouse_dy;
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
        (dx, dy)
    }

    pub fn setup_input(&mut self, dt: f32, now: f32, camera: &mut Camera) {
        if self.hold(Key::W) {
            camera.input_event(dt, CameraAction::W);
        }
        if self.hold(Key::S) {
            camera.input_event(dt, CameraAction::S);
        }
        if self.hold(Key::A) {
            camera.input_event(dt, CameraAction::A);
        }
        if self.hold(Key::D) {
            camera.input_event(dt, CameraAction::D);
        }

        if self.tap(Key::M, now) {
            camera.input_event(dt, CameraAction::ToggleMode);
        }

        if self.hold(Key::Equal) {
            camera.input_event(dt, CameraAction::ZoomIn);
        }
        if self.hold(Key::Minus) {
            camera.input_event(dt, CameraAction::ZoomOut);
        }

        let scroll = self.take_scroll();

        let sensitivity = 3.0;
        let steps = (scroll.abs() * sensitivity).round() as i32;

        for _ in 0..steps {
            if scroll > 0.0 {
                camera.input_event(dt, CameraAction::ZoomIn);
            } else if scroll < 0.0 {
                camera.input_event(dt, CameraAction::ZoomOut);
            }
        }

        if let CameraMode::Free = camera.mode {
            if self.rmb_down() {
                let (dx, dy) = self.take_mouse_delta();
                let sens = 0.002;
                camera.free_look(dx * sens, -dy * sens);
            } else {
                self.take_mouse_delta();
            }
        }
    }
}
