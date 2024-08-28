use super::ffi::*;

pub struct Framebuffer {
    width: u32,
    height: u32
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }

    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn create_window(&self, title: &str) {
        create_window_rs(title, self.width, self.height);
    }

    pub fn render(&self) {
        render_rs();
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        destroy_window_rs();
    }
}