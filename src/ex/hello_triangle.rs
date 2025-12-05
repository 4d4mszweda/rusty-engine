use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ffi::c_void;

pub fn run() {
    // --- inicjalizacja GLFW ---
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(800, 600, "Rust OpenGL", glfw::WindowMode::Windowed)
        .expect("Nie mogę stworzyć okna");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // --- załaduj funkcje OpenGL ---
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const c_void);

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    }

    // --- dane wierzchołków trójkąta ---
    let vertices: [f32; 9] = [
        // position     // color
        0.0, 0.5, 0.0, // góra
        -0.5, -0.5, 0.0, // lewy dół
        0.5, -0.5, 0.0, // prawy dół
    ];

    // --- VAO + VBO ---
    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Typ bufforu
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        ); // tutaj przypisujemy dane z tabliy vartices do buffera

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    // --- shadery jako string literal ---
    const VERT_SRC: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;

        void main() {
            gl_Position = vec4(aPos, 1.0);
        }
    "#;

    const FRAG_SRC: &str = r#"
        #version 330 core
        out vec4 FragColor;

        void main() {
            FragColor = vec4(1.0, 0.5, 0.2, 1.0);
        }
    "#;

    // --- kompilacja shaderów + program ---
    let shader_program = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_vert = CString::new(VERT_SRC).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_vert.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_frag = CString::new(FRAG_SRC).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_frag.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_shader);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    };

    // --- główna pętla ---
    while !window.should_close() {
        // obsługa inputu
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
    }
}
