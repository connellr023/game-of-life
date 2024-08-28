pub trait Framebuffer {
    fn new(width: u32, height: u32) -> Self;
    fn create_window(&self, title: &str);
    fn handle_events(&self);
    fn render(&self);
    fn write_pixel(&self, x: u32, y: u32, color: u32);
    fn register_keydown_listener(&self, keycode: u32, listener: Box<dyn Fn()>);
    fn clear_keydown_listeners(&self);

    fn is_running(&self) -> bool;
    fn stop(&self);
}