use eframe::glow::{self, HasContext};
use std::sync::Arc;

/// Handles the OpenGL texture sharing between the "Wave Engine" (Servo) and the UI.
pub struct OffscreenRenderer {
    pub texture_id: glow::Texture,
    pub fbo_id: glow::Framebuffer,
    pub width: i32,
    pub height: i32,
    gl: Arc<glow::Context>,
}

impl OffscreenRenderer {
    pub fn new(gl: Arc<glow::Context>, width: i32, height: i32) -> Self {
        unsafe {
            // 1. Create a texture to render into
            let texture_id = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture_id));
            
            // Set texture parameters
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            // Allocate storage for the texture
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None, // No data yet
            );

            // 2. Create a Framebuffer Object (FBO)
            let fbo_id = gl.create_framebuffer().expect("Failed to create framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo_id));

            // Attach the texture to the FBO
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture_id),
                0,
            );

            // Check FBO status
            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                log::error!("Framebuffer is not complete!");
            }

            // Unbind
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.bind_texture(glow::TEXTURE_2D, None);

            Self {
                texture_id,
                fbo_id,
                width,
                height,
                gl,
            }
        }
    }

    /// Prepares the OpenGL state for the Engine to paint.
    /// Binds the Framebuffer so Servo writes to our texture.
    pub fn begin_paint(&self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo_id));
            self.gl.viewport(0, 0, self.width, self.height);
            
            // Clear default background to white (typical browser base)
            self.gl.clear_color(1.0, 1.0, 1.0, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    /// Cleans up after the Engine has painted.
    pub fn end_paint(&self) {
        unsafe {
            // Unbind FBO so we don't mess up egui's rendering
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if self.width == width && self.height == height {
            return;
        }
        self.width = width;
        self.height = height;

        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture_id));
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );
            self.gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }
}

impl Drop for OffscreenRenderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture_id);
            self.gl.delete_framebuffer(self.fbo_id);
        }
    }
}
