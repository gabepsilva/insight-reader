//! macOS-specific hotkey display formatting

use super::common::format_key_code;
use global_hotkey::hotkey::Modifiers;

// Re-export common types and functions
pub use super::common::{HotkeyConfig, HotkeyManager};

/// Format hotkey configuration as a display string for menu items (macOS uses symbols)
pub fn format_hotkey_display(config: &super::common::HotkeyConfig) -> String {
    let mut parts = Vec::new();
    let key_str = format_key_code(config.key);

    if config.modifiers.contains(Modifiers::META) {
        parts.push("⌘".to_string());
    }
    if config.modifiers.contains(Modifiers::SHIFT) {
        parts.push("⇧".to_string());
    }
    if config.modifiers.contains(Modifiers::ALT) {
        parts.push("⌥".to_string());
    }
    if config.modifiers.contains(Modifiers::CONTROL) {
        parts.push("⌃".to_string());
    }
    parts.push(key_str);
    parts.join("")
}
