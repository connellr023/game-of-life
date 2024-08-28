pub use super::framebuffer::Framebuffer;

#[cfg(target_os = "windows")]
pub use super::windows_framebuffer::WindowsFramebuffer;

#[cfg(target_os = "linux")]
pub use super::linux_framebuffer::LinuxFramebuffer;