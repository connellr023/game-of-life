use framebuffer::framebuffer::Framebuffer;

mod framebuffer;

fn main() {
    Framebuffer::create_window("Game of Life", 800, 600);

    loop {
        if !Framebuffer::is_running() {
            break;
        }

        Framebuffer::handle_events();
        Framebuffer::render();
    }

    Framebuffer::destroy_window();
}
