use std::rc::Rc;

use crate::{Framebuffer, PlatformFramebuffer};

const BG_COLOR: u32 = 0x000000;
const FRACTAL_COLOR: u32 = 0xFFFFFF;

pub struct Fractal {
    fb: Rc<PlatformFramebuffer>,
    square_size: u8,
}

impl Fractal {
    pub fn new(fb: Rc<PlatformFramebuffer>, square_size: u8) -> Self {
        Self { fb, square_size }
    }

    fn render_rect(&self, x: u32, y: u32, size: u16, color: u32) {
        let scaled_size = size as u32 * self.square_size as u32;

        for i in 0..scaled_size {
            for j in 0..scaled_size {
                self.fb.write_pixel(x + i, y + j, color);
            }
        }
    }

    pub fn render(&self, steps: u8, left: u16, top: u16, width: u16) {
        if steps == 0 {
            return;
        }

        self.render_rect(left, top, width, FRACTAL_COLOR);

        let w = width / 2;

        render(steps - 1, left - (w / 2), top - (w / 2), w);
        render(steps - 1, left + (w / 2), top - (w / 2), w);
        render(steps - 1, left + (w / 2), top + (w / 2), w);
        render(steps - 1, left - (w / 2), top + (w / 2), w);
    }
}
