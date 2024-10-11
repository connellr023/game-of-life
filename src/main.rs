use crate::fractal::fractal::Fractal;
use crate::framebuffer::prelude::*;
use anyhow::Result;
use std::rc::Rc;

mod fractal;
mod framebuffer;
mod macros;

const WINDOW_TITLE: &str = "Fractal Test";

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const SQUARE_PIXEL_SIZE: u8 = 5;

fn main() -> Result<()> {
    let fb = Rc::new(PlatformFramebuffer::create_window(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);

    let fractal = Fractal::new(Rc::clone(&fb), SQUARE_PIXEL_SIZE);

    fractal.render(4, WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, WINDOW_WIDTH / 2);

    while fb.is_running() {
        fb.handle_events();
        fb.render();
    }

    Ok(())
}
