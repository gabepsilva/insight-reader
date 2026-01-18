//! Stub implementation for platforms without hotkey support

use global_hotkey::hotkey::{Code, Modifiers};
use tracing::warn;

/// Hotkey configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyConfig {
    pub modifiers: Modifiers,
    pub key: Code,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            modifiers: Modifiers::CONTROL,
            key: Code::KeyR,
        }
    }
}

/// Global hotkey manager (stub)
pub struct HotkeyManager {
    enabled: bool,
}

impl HotkeyManager {
    /// Create a new hotkey manager (stub)
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        warn!("Global hotkeys not supported on this platform");
        Ok(Self { enabled: false })
    }

    /// Register a hotkey (stub)
    pub fn register(&mut self, _config: HotkeyConfig) -> Result<(), Box<dyn std::error::Error>> {
        warn!("Global hotkeys not supported on this platform");
        Ok(())
    }

    /// Unregister the current hotkey (stub)
    pub fn unregister(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// Check if hotkey is currently enabled
    pub fn is_enabled(&self) -> bool {
        false
    }

    /// Try to receive a hotkey press event (stub)
    pub fn try_recv(&self) -> Option<()> {
        None
    }
}

/// Format key code as a display string (stub)
pub fn format_key_code(code: Code) -> String {
    let debug_str = format!("{:?}", code);
    debug_str
        .strip_prefix("Key")
        .unwrap_or(&debug_str)
        .to_uppercase()
}

/// Format hotkey configuration as a display string (stub)
pub fn format_hotkey_display(config: &HotkeyConfig) -> String {
    let mut parts = Vec::<String>::new();
    let key_str = format_key_code(config.key);

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
    parts.push(key_str);
    parts.join(" + ")
}
