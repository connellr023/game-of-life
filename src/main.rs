use framebuffer::prelude::*;
use anyhow::Result;

mod framebuffer;

const WINDOW_TITLE: &str = "Game Of Life";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    let fb = WindowsFramebuffer::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)?;

    while fb.is_running() {
        fb.handle_events();
        fb.render();
    }

    Ok(())
}
