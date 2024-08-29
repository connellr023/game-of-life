use crate::{framebuffer::prelude::PlatformFramebuffer, Framebuffer};
use std::cell::Cell;

pub struct Game<'a> {
    fb: &'a PlatformFramebuffer,
    is_running: Cell<bool>,
    grid: Box<[u8]>,
    tile_pixel_size: u16,
}

impl<'a> Game<'a> {
    pub fn new(fb: &'a PlatformFramebuffer, width: u16, height: u16, tile_pixel_size: u16) -> Self {
        Self {
            fb,
            is_running: Cell::new(true),
            grid: vec![0; (width * height) as usize].into_boxed_slice(),
            tile_pixel_size,
        }
    }

    #[inline(always)]
    pub fn is_running(&self) -> bool {
        self.is_running.get()
    }

    pub fn update(&mut self) {
        // ...
    }
}
