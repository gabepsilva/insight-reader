//! Stub implementation for platforms without hotkey support

use tracing::warn;

/// Hotkey configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyConfig {
    pub modifiers: u8, // Placeholder
    pub key: u8,       // Placeholder
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            modifiers: 0,
            key: 0,
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
        Ok(Self {
            enabled: false,
        })
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
pub fn format_key_code(_code: u8) -> String {
    "N/A".to_string()
}

/// Format hotkey configuration as a display string (stub)
pub fn format_hotkey_display(_config: &HotkeyConfig) -> String {
    "Not supported".to_string()
}
