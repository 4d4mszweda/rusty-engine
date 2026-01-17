use std::ffi::c_void;
use std::{mem, ptr};

pub struct InstancedMesh {
    pub vao: u32,
    pub vbo: u32,
    pub instance_vbo: u32,
    pub vertex_count: i32,
    pub instance_count: i32,
}

impl InstancedMesh {
    /// Bazuje na istniejącym VAO/VBO quada (pos/normal/uv w loc 0/1/2)
    /// Dodaje instanced atrybut mat4 w loc 3..6
    pub fn from_quad_with_instances(max_instances: usize) -> Self {
        // Wykorzystaj swój Mesh::quad() jako bazę, ale potrzebujemy jego vao/vbo.
        // Jeśli Mesh::quad() zwraca Mesh z vao/vbo publiczne: użyj tego.
        // Jeśli nie: skopiuj kod z Mesh::quad() tutaj i zwróć vao/vbo.

        let base = crate::mesh::Mesh::quad();

        let mut instance_vbo = 0;
        unsafe {
            gl::BindVertexArray(base.vao);

            gl::GenBuffers(1, &mut instance_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, instance_vbo);

            // max_instances * mat4(16 floats)
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (max_instances * 16 * mem::size_of::<f32>()) as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            let stride = (16 * mem::size_of::<f32>()) as i32;

            for i in 0..4usize {
                let loc: u32 = 3u32 + i as u32;

                gl::EnableVertexAttribArray(loc);
                gl::VertexAttribPointer(
                    loc,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    (i * 4 * mem::size_of::<f32>()) as isize as *const c_void,
                );
                gl::VertexAttribDivisor(loc, 1);
            }

            gl::BindVertexArray(0);
        }

        Self {
            vao: base.vao,
            vbo: base.vbo,
            instance_vbo,
            vertex_count: base.vertex_count,
            instance_count: 0,
        }
    }

    pub fn update_instances(&mut self, matrices: &[cgmath::Matrix4<f32>]) {
        self.instance_count = matrices.len() as i32;

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instance_vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (matrices.len() * 16 * mem::size_of::<f32>()) as isize,
                matrices.as_ptr() as *const c_void,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn draw_instanced(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, self.vertex_count, self.instance_count);
            gl::BindVertexArray(0);
        }
    }
}
