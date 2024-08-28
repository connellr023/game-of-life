use super::ffi::*;

pub struct Framebuffer;

impl Framebuffer {
    pub fn create_window(title: &str, width: u32, height: u32) {
        assert!(!Self::is_running(), "Window is already running!");
        create_window_rs(title, width, height);
    }

    #[inline(always)]
    pub fn is_running() -> bool {
        is_window_running_rs()
    }

    pub fn handle_events() {
        handle_events_rs();
    }

    pub fn render() {
        render_rs();
    }

    pub fn destroy_window() {
        destroy_window_rs();
    }
}