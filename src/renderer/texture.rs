use crate::truevision::Targa;

pub struct Texture {
    raw: u32,
}

impl Texture {
    pub fn from_targa(targa: &Targa) -> Self {
        unsafe {
            let mut raw = 0;
            gl::GenTextures(1, &mut raw);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, raw);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as _,
                targa.width as _,
                targa.height as _,
                0,
                gl::BGR,
                gl::UNSIGNED_BYTE,
                targa.bytes.as_ptr() as _,
            );

            Self::unbind_slot(0);

            return Self { raw };
        }
    }

    pub fn bind_slot(&self, slot: u32) {
        unsafe {
            let slot = gl::TEXTURE0 + slot; // warn: unsound
            gl::ActiveTexture(slot);
            gl::BindTexture(gl::TEXTURE_2D, self.raw);
        }
    }

    pub fn unbind_slot(slot: u32) {
        unsafe {
            let slot = gl::TEXTURE0 + slot; // warn: unsound
            gl::ActiveTexture(slot);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.raw);
        }
    }
}
