use crate::framebuffer::prelude::*;
use crate::game::prelude::*;
use anyhow::Result;
use std::rc::Rc;
use std::time::Duration;

mod framebuffer;
mod game;

const WINDOW_TITLE: &str = "Game Of Life";

const GRID_WIDTH: u16 = 370;
const GRID_HEIGHT: u16 = 230;
const TILE_PIXEL_SIZE: u16 = 4;

const WINDOW_WIDTH: u32 = (GRID_WIDTH * TILE_PIXEL_SIZE) as u32;
const WINDOW_HEIGHT: u32 = (GRID_HEIGHT * TILE_PIXEL_SIZE) as u32;

const UPDATE_MS: u64 = 110;

const KEY_ESCAPE: u32 = 0x1B;
const KEY_ENTER: u32 = 0x0D;
const KEY_LEFT: u32 = 0x25;
const KEY_UP: u32 = 0x26;
const KEY_RIGHT: u32 = 0x27;
const KEY_DOWN: u32 = 0x28;
const KEY_SPACE: u32 = 0x20;

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
        Duration::from_millis(UPDATE_MS),
    );

    register_game_keydown_listener!(fb, KEY_ENTER, game, generate);
    register_game_keydown_listener!(fb, KEY_SPACE, game, place_cursor);
    register_game_keydown_listener!(fb, KEY_LEFT, game, move_cursor, CursorDirection::Left);
    register_game_keydown_listener!(fb, KEY_UP, game, move_cursor, CursorDirection::Up);
    register_game_keydown_listener!(fb, KEY_RIGHT, game, move_cursor, CursorDirection::Right);
    register_game_keydown_listener!(fb, KEY_DOWN, game, move_cursor, CursorDirection::Down);

    game.generate();

    while fb.is_running() {
        fb.handle_events();
        fb.render();

        game.update();
        game.render();
    }

    Ok(())
}
