use anyhow::Result;

pub type BoxedListener<'a> = Box<dyn Fn() + 'a>;

pub trait Framebuffer<'a>: Sized {
    fn create_window(title: &str, width: u32, height: u32) -> Result<Self>;
    fn handle_events(&self);
    fn render(&self);
    fn write_pixel(&self, x: u32, y: u32, color: u32);
    fn register_keydown_listener(&mut self, keycode: u32, listener: BoxedListener<'a>);

    fn is_running(&self) -> bool;
}
