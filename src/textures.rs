use std::os::raw::c_void;
use std::path::Path;

use image::GenericImage;

pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Texture {
        let path_ref = path.as_ref();
        println!("Loading texture: {:?}", path_ref);

        // wczytanie obrazu i konwersja na RGBA8
        let img = image::open(path_ref).expect("Failed to load texture");
        let img = img.flipv(); // OpenGL ma (0,0) w lewym dolnym rogu
        let (width, height) = img.dimensions();
        let data = img.to_rgba();

        let mut tex_id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);

            // parametry tekstury
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture { id: tex_id }
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}
