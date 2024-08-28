use framebuffer::prelude::*;

mod framebuffer;

const WINDOW_TITLE: &str = "Framebuffer Example";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    #[cfg(target_os = "windows")]
    let fb = WindowsFramebuffer::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

    #[cfg(target_os = "linux")]
    let fb = LinuxFramebuffer::create_window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

    let fb = match fb {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    while fb.is_running() {
        fb.handle_events();
        fb.render();
    }
}
