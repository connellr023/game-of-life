use std::cell::Cell;

use super::framebuffer::Framebuffer;
use anyhow::{anyhow, Result};
use windows::{
    core::{w, PCWSTR},
    Win32::{
        Foundation::{HANDLE, HWND, LPARAM, LRESULT, WPARAM}, Graphics::Gdi::{
            BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteDC, GetDC, SelectObject, UpdateWindow, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HBRUSH, HDC, RGBQUAD, SRCCOPY
        }, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageA, GetMessageA, PeekMessageA, PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HCURSOR, HICON, MSG, PEEK_MESSAGE_REMOVE_TYPE, SHOW_WINDOW_CMD, WINDOW_EX_STYLE, WM_CLOSE, WM_DESTROY, WM_QUIT, WNDCLASSW, WS_OVERLAPPEDWINDOW
        }
    }
};

pub struct WindowsFramebuffer {
    hwnd: HWND,
    hdc: HDC,
    bitmap: HBITMAP,
    buffer: Box<[u32]>,
    width: u32,
    height: u32,
    is_running: Cell<bool>
}

impl WindowsFramebuffer {
    unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            },
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            },
            _ => DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

impl Framebuffer for WindowsFramebuffer {
    fn create_window(title: &str, width: u32, height: u32) -> Result<Self> {
        let h_instance = unsafe { GetModuleHandleW(None) }?.into();
        let w_title = w!("For now");

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::window_proc),
            hInstance: h_instance,
            lpszClassName: w_title,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON::default(),
            hCursor: HCURSOR::default(),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCWSTR(&0)
        };

        if unsafe { RegisterClassW(&wc) } == 0 {
            return Err(anyhow!("RegisterClassW failed"));
        }

        let hwnd = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                wc.lpszClassName,
                w_title,
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width as i32,
                height as i32,
                None,
                None,
                h_instance,
                None
            )
        }?;

        if hwnd.is_invalid() {
            return Err(anyhow!("CreateWindowExW failed"));
        }

        let hdc = unsafe { GetDC(hwnd) };
        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0
            },
            bmiColors: [RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }]
        };

        let buffer = vec![0; (width * height) as usize].into_boxed_slice();
        let bitmap = unsafe { CreateDIBSection(
            hdc,
            &bmi,
            DIB_RGB_COLORS,
            &mut buffer.as_ptr() as *mut _ as *mut _,
            HANDLE::default(),
            0
        ) }?;

        if bitmap.is_invalid() {
            return Err(anyhow!("CreateDIBSection failed"));
        }

        unsafe {
            let _ = ShowWindow(hwnd, SHOW_WINDOW_CMD(5));
            let _ = UpdateWindow(hwnd);
        };

        Ok(Self {
            hwnd,
            hdc,
            bitmap,
            buffer,
            width,
            height,
            is_running: Cell::new(true)
        })
    }

    fn handle_events(&self) {
        let mut msg = MSG::default();

        unsafe {
            while self.is_running() {
                let result = GetMessageA(&mut msg, None, 0, 0);

                if !result.as_bool() {
                    self.is_running.set(false);
                }
                else {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
        }
    }
    
    fn render(&self) {
        unsafe {
            let hdc_mem = CreateCompatibleDC(self.hdc);
            let old_bitmap = SelectObject(hdc_mem, self.bitmap);

            match BitBlt(self.hdc, 0, 0, self.width as i32, self.height as i32, hdc_mem, 0, 0, SRCCOPY) {
                Ok(_) => (),
                Err(_) => {
                    self.is_running.set(false);
                    return;
                }
            };

            SelectObject(hdc_mem, old_bitmap);
            let _ = DeleteDC(hdc_mem);
        }
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
        self.is_running.get()
    }
    
    fn stop(&self) {
        todo!()
    }
}