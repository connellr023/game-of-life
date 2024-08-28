use super::framebuffer::Framebuffer;

pub struct WindowsFramebuffer {
    width: u32,
    height: u32
}

impl Framebuffer for WindowsFramebuffer {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }
    
    fn create_window(&self, title: &str) {
        todo!()
    }
    
    fn handle_events(&self) {
        todo!()
    }
    
    fn render(&self) {
        todo!()
    }
    
    fn write_pixel(&self, x: u32, y: u32, color: u32) {
        todo!()
    }
    
    fn register_keydown_listener(&self, keycode: u32, listener: Box<dyn Fn()>) {
        todo!()
    }
    
    fn clear_keydown_listeners(&self) {
        todo!()
    }
    
    fn is_running(&self) -> bool {
        todo!()
    }
    
    fn stop(&self) {
        todo!()
    }
}