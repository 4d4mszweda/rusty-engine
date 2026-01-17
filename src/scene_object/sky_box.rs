use crate::program;

use std::ffi::c_void;
use std::path::Path;
use std::{mem, ptr};

use cgmath::Matrix4;

pub struct SkyBoxState {
    pub skyboxes: Vec<SkyBox>,
    pub skybox_selected: usize,
}

impl Default for SkyBoxState {
    fn default() -> Self {
        Self {
            skyboxes: vec![
                SkyBox::from_folder("assets/textures/skybox1", "Skybox 1").expect("skybox1"),
                SkyBox::from_folder("assets/textures/skybox2", "Skybox 2").expect("skybox2"),
                SkyBox::from_folder("assets/textures/skybox3", "Skybox 3").expect("skybox3"),
            ],
            skybox_selected: 0,
        }
    }
}

pub struct SkyBox {
    vao: u32,
    vbo: u32,
    cubemap: u32,
    program: program::Program,
    pub name: String,
}

impl SkyBox {
    /// Folder musi zawierać:
    /// posx.jpg, negx.jpg, posy.jpg, negy.jpg, posz.jpg, negz.jpg
    pub fn from_folder<P: AsRef<Path>>(folder: P, name: impl Into<String>) -> Result<Self, String> {
        let folder = folder.as_ref();

        let program = program::Program::from_files(
            "assets/shaders/skybox.vert",
            "assets/shaders/skybox.frag",
        );

        let (vao, vbo) = Self::create_cube_vao();
        let cubemap = Self::load_cubemap(folder)?;

        Ok(Self {
            vao,
            vbo,
            cubemap,
            program,
            name: name.into(),
        })
    }

    pub fn draw(&self, view: &Matrix4<f32>, proj: &Matrix4<f32>) {
        unsafe {
            gl::DepthFunc(gl::LEQUAL);
            gl::DepthMask(gl::FALSE);
        }

        self.program.use_program();

        // Usuń translację z view (skybox zawsze w tle)
        let mut view_no_translation = *view;
        view_no_translation.w.x = 0.0;
        view_no_translation.w.y = 0.0;
        view_no_translation.w.z = 0.0;

        self.program.set_mat4("u_view", &view_no_translation);
        self.program.set_mat4("u_proj", proj);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.cubemap);
        }
        self.program.set_int("u_skybox", 0);

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
            gl::DepthMask(gl::TRUE);
            gl::DepthFunc(gl::LESS);
        }
    }

    fn create_cube_vao() -> (u32, u32) {
        // 36 wierzchołków (pos only)
        let vertices: [f32; 36 * 3] = [
            // back (-Z)
            -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, // left (-X)
            -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, 1.0, // right (+X)
            1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
            -1.0, -1.0, // front (+Z)
            -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, // top (+Y)
            -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, -1.0, // bottom (-Y)
            -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0,
            1.0, -1.0, 1.0,
        ];

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * mem::size_of::<f32>()) as i32,
                ptr::null(),
            );

            gl::BindVertexArray(0);
        }

        (vao, vbo)
    }

    fn load_cubemap(folder: &Path) -> Result<u32, String> {
        // OpenGL expects: +X, -X, +Y, -Y, +Z, -Z
        // U Ciebie pliki: posx/negx/posy/negy/posz/negz
        let faces = [
            ("posx.jpg", gl::TEXTURE_CUBE_MAP_POSITIVE_X),
            ("negx.jpg", gl::TEXTURE_CUBE_MAP_NEGATIVE_X),
            ("posy.jpg", gl::TEXTURE_CUBE_MAP_POSITIVE_Y),
            ("negy.jpg", gl::TEXTURE_CUBE_MAP_NEGATIVE_Y),
            ("posz.jpg", gl::TEXTURE_CUBE_MAP_POSITIVE_Z),
            ("negz.jpg", gl::TEXTURE_CUBE_MAP_NEGATIVE_Z),
        ];

        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, tex);
        }

        for (file, target) in faces {
            let path = folder.join(file);

            let img = image::open(&path)
                .map_err(|e| format!("Skybox load error {:?}: {}", path, e))?
                .to_rgba();

            let (w, h) = img.dimensions();
            let data = img.into_raw();

            unsafe {
                gl::TexImage2D(
                    target,
                    0,
                    gl::RGBA as i32,
                    w as i32,
                    h as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const c_void,
                );
            }
        }

        unsafe {
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }

        Ok(tex)
    }
}
