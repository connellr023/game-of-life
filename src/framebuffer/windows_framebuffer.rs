use super::prelude::*;
use anyhow::{anyhow, Result};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ffi::{c_void, OsStr},
    os::windows::ffi::OsStrExt,
    ptr::null_mut,
};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HANDLE, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

pub struct WindowsFramebuffer {
    hwnd: HWND,
    hdc: HDC,
    bitmap: HBITMAP,
    buffer: *mut u8,
    width: u32,
    height: u32,
    is_running: Cell<bool>,
    keydown_listeners: RefCell<HashMap<u32, BoxedListener>>,
}

impl WindowsFramebuffer {
    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_CLOSE => {
                let _ = DestroyWindow(hwnd);
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

impl Framebuffer for WindowsFramebuffer {
    fn create_window(title: &str, width: u32, height: u32) -> Result<Self> {
        let h_instance = unsafe { GetModuleHandleW(None) }?.into();
        let w_title = {
            let s = OsStr::new(title)
                .encode_wide()
                .chain(Some(0))
                .collect::<Box<[u16]>>();

            PCWSTR(s.as_ptr())
        };

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::window_proc),
            hInstance: h_instance,
            lpszClassName: w_title,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON::default(),
            hCursor: unsafe { LoadCursorW(HINSTANCE::default(), IDC_ARROW) }.expect("LoadCursorW"),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCWSTR(&0),
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
                None,
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
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }],
        };

        let mut buffer: *mut u8 = null_mut();
        let bitmap = unsafe {
            CreateDIBSection(
                hdc,
                &bmi,
                DIB_RGB_COLORS,
                &mut buffer as *mut *mut u8 as *mut *mut c_void,
                HANDLE::default(),
                0,
            )
        }?;

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
            is_running: Cell::new(true),
            keydown_listeners: RefCell::new(HashMap::new()),
        })
    }

    fn handle_events(&self) {
        let mut msg = MSG::default();

        unsafe {
            while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    self.is_running.set(false);
                    break;
                } else {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageA(&msg);

                    if msg.message == WM_KEYDOWN {
                        let keycode = msg.wParam.0 as u32;

                        if let Some(listener) = self.keydown_listeners.borrow().get(&keycode) {
                            listener();
                        }
                    }
                }
            }
        }
    }

    fn render(&self) {
        unsafe {
            let hdc_mem = CreateCompatibleDC(self.hdc);
            let old_bitmap = SelectObject(hdc_mem, self.bitmap);

            match BitBlt(
                self.hdc,
                0,
                0,
                self.width as i32,
                self.height as i32,
                hdc_mem,
                0,
                0,
                SRCCOPY,
            ) {
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
        debug_assert!(x < self.width && y < self.height, "Pixel out of bounds");

        let offset = (y * self.width + x) * 4;

        unsafe {
            let pixel_ptr = self.buffer.add(offset as usize) as *mut u32;
            *pixel_ptr = color;
        }
    }

    fn register_keydown_listener(&self, keycode: u32, listener: BoxedListener) {
        self.keydown_listeners
            .borrow_mut()
            .insert(keycode, listener);
    }

    fn is_running(&self) -> bool {
        self.is_running.get()
    }

    fn stop(&self) {
        self.is_running.set(false);
    }
}

impl Drop for WindowsFramebuffer {
    fn drop(&mut self) {
        self.keydown_listeners.borrow_mut().clear();

        unsafe {
            if !self.bitmap.is_invalid() {
                let _ = DeleteObject(self.bitmap);
            }

            if !self.hdc.is_invalid() {
                let _ = DeleteDC(self.hdc);
            }

            if !self.hwnd.is_invalid() {
                let _ = DestroyWindow(self.hwnd);
            }
        }
    }
}
