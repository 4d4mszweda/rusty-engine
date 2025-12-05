use glfw::{WindowHint, WindowMode};
use std::sync::mpsc::Receiver;

pub fn init_glfw() -> glfw::Glfw {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to init GLFW");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw
}

pub fn create_window(
    glfw: &mut glfw::Glfw,
    width: u32,
    height: u32,
    title: &str,
) -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
    let (mut window, events) = glfw
        .create_window(width, height, title, WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.set_key_polling(true);
    (window, events)
}

pub fn init_gl(window: &mut glfw::Window) {
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::ClearColor(0.2, 0.3, 0.4, 1.0);
    }
}
