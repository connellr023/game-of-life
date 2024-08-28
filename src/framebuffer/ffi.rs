use std::{ffi::CString, os::raw::c_char};

extern "C" {
    fn create_window(title: *const c_char, width: u32, height: u32);
    fn handle_events();
    fn render();
    fn set_pixel(x: u32, y: u32, color: u32);
    fn destroy_window();

    fn is_window_running() -> bool;
    fn stop_window();
}

#[inline(always)]
pub fn create_window_rs(title: &str, width: u32, height: u32) {
    let c_title = CString::new(title).expect("Failed to create CString");

    unsafe {
        let c_title_ptr = c_title.as_ptr();
        create_window(c_title_ptr, width, height);
    }
}

#[inline(always)]
pub fn handle_events_rs() {
    unsafe {
        handle_events();
    }
}

#[inline(always)]
pub fn render_rs() {
    unsafe {
        render();
    }
}

#[inline(always)]
pub fn set_pixel_rs(x: u32, y: u32, color: u32) {
    unsafe {
        set_pixel(x, y, color);
    }
}

#[inline(always)]
pub fn destroy_window_rs() {
    unsafe {
        destroy_window();
    }
}

#[inline(always)]
pub fn is_window_running_rs() -> bool {
    unsafe {
        is_window_running()
    }
}

#[inline(always)]
pub fn stop_window_rs() {
    unsafe {
        stop_window();
    }
}