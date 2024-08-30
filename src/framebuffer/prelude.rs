pub use super::framebuffer::BoxedListener;
pub use super::framebuffer::Framebuffer;

#[cfg(target_os = "windows")]
pub type PlatformFramebuffer<'a> = super::windows_framebuffer::WindowsFramebuffer<'a>;
