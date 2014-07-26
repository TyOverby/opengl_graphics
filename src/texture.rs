use gl;
use gl::types::GLuint;
use libc::c_void;

use image;
use image::GenericImage;

use graphics::ImageSize;

/// Wraps OpenGL texture data.
/// The texture gets deleted when running out of scope.
pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    /// Creates a new texture.
    #[inline(always)]
    pub fn new(id: GLuint, width: u32, height: u32) -> Texture {
        Texture {
            id: id,
            width: width,
            height: height,
        }
    }

    /// Gets the OpenGL id of the texture.
    #[inline(always)]
    pub fn get_id(&self) -> GLuint {
        self.id
    }

    /// Loads image from memory, the format is 8-bit greyscale.
    pub fn from_memory_alpha(buf: &[u8], width: u32, height: u32) -> Result<Texture, String> {
        let mut pixels = Vec::new();
        for alpha in buf.iter() {
            pixels.push(255);
            pixels.push(255);
            pixels.push(255);
            pixels.push(*alpha);
        }

        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void
            );
        }

        Ok(Texture::new(id, width, height))
    }

    /// Loads image by relative file name to the asset root.
    pub fn from_path(path: &Path) -> Result<Texture, String> {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(e)  => return Err(format!("Could not load '{}': {}",
                path.filename_str().unwrap(), e)),
        };

        match img.color() {
            image::RGBA(8) => {},
            c => fail!("Unsupported color type {} in png", c),
        };

        let (width, height) = img.dimensions();

        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as i32
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32
            );
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.raw_pixels().as_ptr() as *const c_void
            );
        }

        Ok(Texture::new(id, width, height))
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl ImageSize for Texture {
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
