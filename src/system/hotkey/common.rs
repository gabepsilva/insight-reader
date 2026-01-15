//! Shared hotkey implementation code for platforms that support global hotkeys

use std::sync::mpsc;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager, GlobalHotKeyEvent,
};
use tracing::{info, warn};

/// Hotkey configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeyConfig {
    pub modifiers: Modifiers,
    pub key: Code,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        let modifiers = Modifiers::META;
        #[cfg(not(target_os = "macos"))]
        let modifiers = Modifiers::CONTROL;
        
        Self {
            modifiers,
            key: Code::KeyR,
        }
    }
}

/// Global hotkey manager
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    receiver: mpsc::Receiver<()>,
    _sender: mpsc::Sender<()>,
    current_hotkey: Option<HotKey>,
}

impl HotkeyManager {
    /// Create a new hotkey manager
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| format!("Failed to create hotkey manager: {e}"))?;
        
        let (sender, receiver) = mpsc::channel();
        
        // Set up event handler for hotkey presses
        GlobalHotKeyEvent::set_event_handler(Some({
            let sender = sender.clone();
            move |_event: GlobalHotKeyEvent| {
                let _ = sender.send(());
            }
        }));
        
        info!("Hotkey manager initialized");
        
        Ok(Self {
            manager,
            receiver,
            _sender: sender,
            current_hotkey: None,
        })
    }
    
    /// Register a hotkey with the given configuration
    pub fn register(&mut self, config: HotkeyConfig) -> Result<(), Box<dyn std::error::Error>> {
        // Unregister existing hotkey if any
        if let Some(ref hotkey) = self.current_hotkey {
            if let Err(e) = self.manager.unregister(*hotkey) {
                warn!(error = %e, "Failed to unregister previous hotkey");
            }
        }
        
        let hotkey = HotKey::new(Some(config.modifiers), config.key);
        
        self.manager.register(hotkey)
            .map_err(|e| format!("Failed to register hotkey: {e}"))?;
        
        self.current_hotkey = Some(hotkey);
        info!(?config, "Hotkey registered successfully");
        Ok(())
    }
    
    /// Unregister the current hotkey
    pub fn unregister(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref hotkey) = self.current_hotkey {
            self.manager.unregister(*hotkey)
                .map_err(|e| format!("Failed to unregister hotkey: {e}"))?;
            self.current_hotkey = None;
            info!("Hotkey unregistered");
        }
        Ok(())
    }
    
    /// Try to receive a hotkey press event (non-blocking)
    pub fn try_recv(&self) -> Option<()> {
        self.receiver.try_recv().ok()
    }
}

/// Format key code as a display string (shared implementation)
pub(crate) fn format_key_code(code: Code) -> String {
    match code {
        Code::KeyR => "R".to_string(),
        Code::KeyT => "T".to_string(),
        Code::KeyS => "S".to_string(),
        Code::Space => "Space".to_string(),
        _ => {
            let debug_str = format!("{:?}", code);
            let stripped = debug_str.strip_prefix("Key").unwrap_or(&debug_str);
            #[cfg(target_os = "macos")]
            {
                stripped.to_uppercase()
            }
            #[cfg(not(target_os = "macos"))]
            {
                stripped.to_string()
            }
        }
    }
}
