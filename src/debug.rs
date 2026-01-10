// src/debug.rs
pub fn gl_error_to_str(err: u32) -> &'static str {
    match err {
        gl::NO_ERROR => "NO_ERROR",
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID_VALUE",
        gl::INVALID_OPERATION => "INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
        _ => "UNKNOWN_ERROR",
    }
}

/// Wyczyść kolejkę błędów (przydatne przed sekcją, którą testujesz)
pub unsafe fn gl_clear_errors() {
    while gl::GetError() != gl::NO_ERROR {}
}

/// Sprawdź błędy OpenGL i wypisz wszystkie.
/// Zwraca true jeśli nie było błędów.
pub unsafe fn gl_check_errors(file: &str, line: u32, where_: &str) -> bool {
    let mut ok = true;
    loop {
        let err = gl::GetError();
        if err == gl::NO_ERROR {
            break;
        }
        ok = false;
        eprintln!(
            "[OpenGL ERROR] {} (0x{:X}) at {}:{} [{}]",
            gl_error_to_str(err),
            err,
            file,
            line,
            where_
        );
    }
    ok
}

/// Shader compile error z nazwą pliku
pub unsafe fn check_shader_compile(shader: u32, shader_path: &str) -> Result<(), String> {
    let mut status: i32 = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

    if status == gl::TRUE as i32 {
        return Ok(());
    }

    let mut len: i32 = 0;
    gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0u8; len.max(1) as usize];
    gl::GetShaderInfoLog(
        shader,
        len,
        std::ptr::null_mut(),
        buf.as_mut_ptr() as *mut i8,
    );

    let log = String::from_utf8_lossy(&buf).to_string();
    Err(format!(
        "Shader compile failed in file: {}\n--- compiler log ---\n{}\n--------------------",
        shader_path, log
    ))
}

/// Program link error z listą plików shaderów
pub unsafe fn check_program_link(
    program: u32,
    program_name: &str,
    shader_files: &[&str],
) -> Result<(), String> {
    let mut status: i32 = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

    if status == gl::TRUE as i32 {
        return Ok(());
    }

    let mut len: i32 = 0;
    gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

    let mut buf = vec![0u8; len.max(1) as usize];
    gl::GetProgramInfoLog(
        program,
        len,
        std::ptr::null_mut(),
        buf.as_mut_ptr() as *mut i8,
    );

    let log = String::from_utf8_lossy(&buf).to_string();
    Err(format!(
        "Program link failed: {}\nShaders: {:?}\n--- linker log ---\n{}\n------------------",
        program_name, shader_files, log
    ))
}

/// Makro w stylu CHECK_FOR_ERRORS: owija pojedyncze wywołanie GL i sprawdza błędy.
#[macro_export]
macro_rules! GL_CALL {
    ($expr:expr) => {{
        unsafe {
            $crate::debug::gl_clear_errors();
            let r = $expr;
            $crate::debug::gl_check_errors(file!(), line!(), stringify!($expr));
            r
        }
    }};
}
