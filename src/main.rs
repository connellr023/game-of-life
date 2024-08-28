use framebuffer::framebuffer::Framebuffer;

mod framebuffer;

fn main() {
    let fb = Framebuffer::new(800, 600);

    fb.create_window("Test");

    loop {
        fb.render();
    }
}
