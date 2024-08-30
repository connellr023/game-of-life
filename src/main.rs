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

const SIM_UPDATE_MS: u64 = 30;

const KEY_ESCAPE: u32 = 0x1B;
const KEY_ENTER: u32 = 0x0D;

fn main() -> Result<()> {
    let fb = Rc::new(PlatformFramebuffer::create_window(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )?);

    fb.register_keydown_listener(
        KEY_ESCAPE,
        Box::new({
            let tmp_ptr = fb.as_ref() as *const PlatformFramebuffer;
            move || unsafe {
                (*tmp_ptr).stop();
            }
        }),
    );

    let mut game = Game::new(
        Rc::clone(&fb),
        GRID_WIDTH,
        GRID_HEIGHT,
        TILE_PIXEL_SIZE,
        SIM_UPDATE_MS,
    );

    fb.register_keydown_listener(
        KEY_ENTER,
        Box::new({
            let tmp_ptr = &mut game as *mut Game;
            move || unsafe {
                (*tmp_ptr).generate();
            }
        }),
    );

    game.generate();

    while fb.is_running() {
        fb.handle_events();
        fb.render();

        game.update();
    }

    Ok(())
}
