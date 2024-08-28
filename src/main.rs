use framebuffer::prelude::*;

mod framebuffer;

fn main() {
    #[cfg(target_os = "windows")]
    let fb = WindowsFramebuffer::new(800, 600);

    #[cfg(target_os = "linux")]
    let fb = LinuxFramebuffer::new(800, 600);
}
