use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use tobj;

pub struct Mesh {
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: i32,
}

impl Mesh {
    pub fn from_obj<P: AsRef<Path>>(path: P) -> Mesh {
        let path_ref = path.as_ref();
        println!("Loading OBJ: {:?}", path_ref);
        println!("CWD: {:?}", std::env::current_dir().unwrap());
        println!("Trying to load: {:?}", path_ref);
        println!("Exists? {}", path_ref.exists());

        let (models, _materials) = tobj::load_obj(path_ref)
            .unwrap_or_else(|e| panic!("Failed to load OBJ {:?}: {:?}", path_ref, e));

        if models.is_empty() {
            panic!("OBJ file has no models!");
        }

        let mesh = &models[0].mesh;

        let mut vertices: Vec<f32> = Vec::new();
        for &i in &mesh.indices {
            let i = i as usize;

            let px = mesh.positions[3 * i];
            let py = mesh.positions[3 * i + 1];
            let pz = mesh.positions[3 * i + 2];

            let (nx, ny, nz) = if !mesh.normals.is_empty() {
                (
                    mesh.normals[3 * i],
                    mesh.normals[3 * i + 1],
                    mesh.normals[3 * i + 2],
                )
            } else {
                (0.0, 1.0, 0.0)
            };

            let (tx, ty) = if !mesh.texcoords.is_empty() {
                (mesh.texcoords[2 * i], mesh.texcoords[2 * i + 1])
            } else {
                (0.0, 0.0)
            };

            vertices.extend_from_slice(&[px, py, pz, nx, ny, nz, tx, ty]);
        }

        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

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

            let stride = (8 * mem::size_of::<f32>()) as i32;

            // position
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());

            // normal
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<f32>()) as *const c_void,
            );

            // texcoord
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<f32>()) as *const c_void,
            );

            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            vertex_count: mesh.indices.len() as i32,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            gl::BindVertexArray(0);
        }
    }
}
