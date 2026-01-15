//! System tray icon and menu bar integration

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod stub;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub use stub::*;
