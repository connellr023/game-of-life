use crate::framebuffer::prelude::*;
use crate::game::prelude::*;
use anyhow::Result;
use std::rc::Rc;

mod framebuffer;
mod game;

const WINDOW_TITLE: &str = "Game Of Life";

const GRID_WIDTH: u16 = 64;
const GRID_HEIGHT: u16 = 64;
const TILE_PIXEL_SIZE: u16 = 8;

const WINDOW_WIDTH: u32 = (GRID_WIDTH * TILE_PIXEL_SIZE) as u32;
const WINDOW_HEIGHT: u32 = (GRID_HEIGHT * TILE_PIXEL_SIZE) as u32;

const KEY_ESCAPE: u32 = 0x1B;

fn main() -> Result<()> {
    let mut fb = PlatformFramebuffer::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)?;
    let fb_ptr = Rc::new(&mut fb as *mut PlatformFramebuffer);

    fb.register_keydown_listener(
        KEY_ESCAPE,
        Box::new({
            let fb_ptr = Rc::clone(&fb_ptr);
            move || unsafe {
                (**fb_ptr).stop();
            }
        }),
    );

    let mut game = Game::new(&fb, GRID_WIDTH, GRID_HEIGHT, TILE_PIXEL_SIZE);

    while fb.is_running() && game.is_running() {
        fb.handle_events();
        fb.render();

        game.update();
    }

    Ok(())
}
