use std::{ffi::CString, os::raw::c_char};

extern "C" {
    fn create_window(title: *const c_char, width: u32, height: u32);
    fn render();
    fn set_pixel(x: u32, y: u32, color: u32);
    fn destroy_window();
}

pub fn create_window_rs(title: &str, width: u32, height: u32) {
    let c_title = CString::new(title).expect("Failed to create CString");

    unsafe {
        let c_title_ptr = c_title.as_ptr();
        create_window(c_title_ptr, width, height);
    }
}

pub fn render_rs() {
    unsafe {
        render();
    }
}

pub fn set_pixel_rs(x: u32, y: u32, color: u32) {
    unsafe {
        set_pixel(x, y, color);
    }
}

pub fn destroy_window_rs() {
    unsafe {
        destroy_window();
    }
}