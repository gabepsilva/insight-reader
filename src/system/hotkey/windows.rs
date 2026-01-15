//! Windows-specific hotkey display formatting

use global_hotkey::hotkey::Modifiers;
use super::common::format_key_code;

// Re-export common types and functions
pub use super::common::{HotkeyConfig, HotkeyManager};

/// Format hotkey configuration as a display string for menu items (Windows uses text labels)
pub fn format_hotkey_display(config: &super::common::HotkeyConfig) -> String {
    let mut parts = Vec::new();
    
    if config.modifiers.contains(Modifiers::META) {
        parts.push("Meta".to_string());
    }
    if config.modifiers.contains(Modifiers::CONTROL) {
        parts.push("Ctrl".to_string());
    }
    if config.modifiers.contains(Modifiers::SHIFT) {
        parts.push("Shift".to_string());
    }
    if config.modifiers.contains(Modifiers::ALT) {
        parts.push("Alt".to_string());
    }
    parts.push(format_key_code(config.key));
    parts.join(" + ")
}
