use crate::camera::{Camera, CameraAction};
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
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            scroll_y: 0.0,
            tap_time: 0.2,
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
    }
}
