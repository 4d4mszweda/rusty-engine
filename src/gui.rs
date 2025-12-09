use egui::{Context as EguiContext, Event as EguiEvent, Pos2, Rect, vec2};
use egui_glow::Painter;
use egui_glow::glow;
use std::sync::Arc;

pub struct Gui {
    ctx: EguiContext,
    painter: Painter,
    events: Vec<EguiEvent>,
    pointer_pos: Option<Pos2>,
}

impl Gui {
    pub fn new(glow_ctx: Arc<glow::Context>) -> Self {
        let ctx = EguiContext::default();

        let painter =
            Painter::new(glow_ctx, "", None, false).expect("Failed to create egui_glow Painter");

        Self {
            ctx,
            painter,
            events: Vec::new(),
            pointer_pos: None,
        }
    }

    pub fn context(&self) -> &EguiContext {
        &self.ctx
    }

    /// Zaczynamy nową klatkę – czyścimy listę eventów
    pub fn begin_frame(&mut self) {
        self.events.clear();
    }

    /// Wołasz to dla każdego WindowEvent z glfw
    pub fn on_glfw_event(&mut self, window: &glfw::Window, event: &glfw::WindowEvent) {
        use egui::{Event, PointerButton};
        use glfw::WindowEvent::*;
        use glfw::{Action, Modifiers, MouseButton};

        match *event {
            CursorPos(x, y) => {
                let pos = Pos2::new(x as f32, y as f32);
                self.pointer_pos = Some(pos);
                self.events.push(Event::PointerMoved(pos));
            }
            MouseButton(btn, action, mods) => {
                if let Some(pos) = self.pointer_pos {
                    let button = match btn {
                        MouseButton::Button1 => PointerButton::Primary,
                        MouseButton::Button2 => PointerButton::Secondary,
                        MouseButton::Button3 => PointerButton::Middle,
                        _ => return,
                    };

                    let pressed = action == Action::Press;

                    let modifiers = egui::Modifiers {
                        alt: mods.contains(Modifiers::Alt),
                        ctrl: mods.contains(Modifiers::Control),
                        shift: mods.contains(Modifiers::Shift),
                        mac_cmd: false,
                        command: mods.contains(Modifiers::Control),
                    };

                    self.events.push(Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    });
                }
            }
            Scroll(x, y) => {
                let modifiers = egui::Modifiers::default(); // lub z `mods`, jeśli przekazujesz
                self.events.push(Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Line,
                    delta: vec2(x as f32, y as f32),
                    modifiers,
                });
            }
            Char(c) => {
                self.events.push(Event::Text(c.to_string()));
            }
            Key(key, _scancode, action, mods) => {
                // Tu można zmapować na Event::Key, jeśli chcesz,
                // ale minimum do klikania myszą nie jest wymagane.
                // Warto dodać, jeśli chcesz obsługiwać shortcuty egui.
            }
            _ => {}
        }
    }

    pub fn run<F>(&mut self, window: &glfw::Window, time: f64, build_ui: F) -> egui::FullOutput
    where
        F: FnMut(&EguiContext),
    {
        let (width, height) = window.get_size();
        let width = width.max(1) as f32;
        let height = height.max(1) as f32;

        let raw_input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0.0, 0.0),
                vec2(width, height),
            )),
            time: Some(time),
            events: std::mem::take(&mut self.events),
            ..Default::default()
        };

        self.ctx.run(raw_input, build_ui)
    }

    pub fn paint(&mut self, window: &glfw::Window, full_output: egui::FullOutput) {
        let (width, height) = window.get_size();
        let width = width.max(1) as f32;
        let height = height.max(1) as f32;

        let egui::FullOutput {
            platform_output: _,
            textures_delta,
            shapes,
            ..
        } = full_output;

        let pixels_per_point = self.ctx.pixels_per_point();
        let clipped_primitives = self.ctx.tessellate(shapes, pixels_per_point);

        // tekstury
        for (id, delta) in textures_delta.set {
            self.painter.set_texture(id, &delta);
        }
        for id in textures_delta.free {
            self.painter.free_texture(id);
        }

        // rysowanie
        self.painter.paint_primitives(
            [width as u32, height as u32],
            pixels_per_point,
            &clipped_primitives,
        );
    }
}
