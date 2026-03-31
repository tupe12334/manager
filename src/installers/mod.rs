pub mod hooks;
pub mod mcp;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(windows)]
pub mod windows;
