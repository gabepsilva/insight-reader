//! Global hotkey management for triggering reading actions

// Shared implementation for platforms that support global hotkeys
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod common;

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
