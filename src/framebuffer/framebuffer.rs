use anyhow::Result;

pub type BoxedListener = Box<dyn Fn()>;

pub trait Framebuffer: Sized {
    fn create_window(title: &str, width: u32, height: u32) -> Result<Self>;
    fn handle_events(&self);
    fn render(&self);
    fn write_pixel(&self, x: u32, y: u32, color: u32);
    fn register_keydown_listener(&self, keycode: u32, listener: BoxedListener);

    fn is_running(&self) -> bool;
    fn stop(&self);
}
