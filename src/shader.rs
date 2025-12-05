use std::ffi::CString;
use std::fs;
use std::path::Path;
use std::ptr;

use cgmath::Matrix4;
use cgmath::prelude::*;

pub struct Program {
    pub id: u32,
}

impl Program {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Program {
        unsafe {
            let vertex_shader = Self::compile_shader(vertex_src, gl::VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(fragment_src, gl::FRAGMENT_SHADER);

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            let mut success: i32 = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len: i32 = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
                panic!(
                    "Program linking error: {}",
                    String::from_utf8_lossy(&buffer)
                );
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Program { id: program_id }
        }
    }

    pub fn from_files<V: AsRef<Path>, F: AsRef<Path>>(vert_path: V, frag_path: F) -> Program {
        let vert_source = fs::read_to_string(vert_path).expect("Failed to read vertex shader file");
        let frag_source =
            fs::read_to_string(frag_path).expect("Failed to read fragment shader file");

        Program::new(&vert_source, &frag_source)
    }

    fn compile_shader(src: &str, shader_type: u32) -> u32 {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_str = CString::new(src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success: i32 = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len: i32 = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
                panic!("Shader compile error: {}", String::from_utf8_lossy(&buffer));
            }

            shader
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, c_name.as_ptr()) }
    }

    pub fn set_mat4(&self, name: &str, mat: &Matrix4<f32>) {
        let loc = self.get_uniform_location(name);
        if loc < 0 {
            return;
        }
        unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.as_ptr());
        }
    }

    pub fn set_vec3(&self, name: &str, v: &cgmath::Vector3<f32>) {
        let loc = self.get_uniform_location(name);
        if loc < 0 {
            return;
        }
        unsafe {
            gl::Uniform3f(loc, v.x, v.y, v.z);
        }
    }

    pub fn set_float(&self, name: &str, val: f32) {
        let loc = self.get_uniform_location(name);
        if loc < 0 {
            return;
        }
        unsafe {
            gl::Uniform1f(loc, val);
        }
    }

    pub fn set_int(&self, name: &str, val: i32) {
        let loc = self.get_uniform_location(name);
        if loc < 0 {
            return;
        }
        unsafe {
            gl::Uniform1i(loc, val);
        }
    }
}
