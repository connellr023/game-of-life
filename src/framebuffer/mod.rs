pub mod prelude;
pub mod framebuffer;

#[cfg(target_os = "windows")]
pub mod windows_framebuffer;

#[cfg(target_os = "linux")]
pub mod linux_framebuffer;