//! macOS system tray implementation

use crate::system::{format_hotkey_display, HotkeyConfig};
use std::sync::mpsc;
use tracing::info;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconBuilder,
};

// Embedded logo asset
const LOGO_PNG: &[u8] = include_bytes!("../../../assets/logo.png");

/// System tray handle
pub struct SystemTray {
    _tray_icon: TrayIcon,
    receiver: mpsc::Receiver<TrayEvent>,
}

/// Events from the system tray
#[derive(Debug, Clone)]
pub enum TrayEvent {
    ShowWindow,
    HideWindow,
    ReadSelected,
    Quit,
}

impl SystemTray {
    /// Create and initialize the system tray icon
    pub fn new(hotkey_config: Option<&HotkeyConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let (sender, receiver) = mpsc::channel();

        // Format hotkey display for menu item
        let read_selected_label = if let Some(config) = hotkey_config {
            let hotkey_display = format_hotkey_display(config);
            format!("Read Selected\t{}", hotkey_display)
        } else {
            "Read Selected".to_string()
        };

        // Create menu items
        let read_selected_item = MenuItem::new(&read_selected_label, true, None);
        let show_item = MenuItem::new("Show Window", true, None);
        let hide_item = MenuItem::new("Hide Window", true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        // Store menu item IDs
        let read_selected_item_id = read_selected_item.id();
        let show_item_id = show_item.id();
        let hide_item_id = hide_item.id();
        let quit_item_id = quit_item.id();

        // Create menu - Read Selected first, then separator, then other items, then separator before Quit
        let separator = PredefinedMenuItem::separator();
        let menu = Menu::new();
        menu.append(&read_selected_item)?;
        menu.append(&separator)?;
        menu.append(&show_item)?;
        menu.append(&hide_item)?;
        menu.append(&separator)?;
        menu.append(&quit_item)?;

        // Load and resize the app logo for the tray icon
        // macOS menu bar icons are typically 16x16 or 22x22 (retina)
        let (rgba_data, width, height) = load_tray_icon_from_logo()?;
        let icon = tray_icon::Icon::from_rgba(rgba_data, width, height)
            .map_err(|e| format!("Failed to create icon: {e}"))?;

        // Set up menu event handler before creating the tray icon
        let sender_clone = sender.clone();
        let show_id = show_item_id.clone();
        let hide_id = hide_item_id.clone();
        let read_selected_id = read_selected_item_id.clone();
        let quit_id = quit_item_id.clone();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            let event_to_send = match event.id {
                id if id == show_id => Some(TrayEvent::ShowWindow),
                id if id == hide_id => Some(TrayEvent::HideWindow),
                id if id == read_selected_id => Some(TrayEvent::ReadSelected),
                id if id == quit_id => Some(TrayEvent::Quit),
                _ => None,
            };

            if let Some(evt) = event_to_send {
                let _ = sender_clone.send(evt);
            }
        }));

        // Create tray icon
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Insight Reader")
            .with_icon(icon)
            .build()
            .map_err(|e| format!("Failed to create tray icon: {e}"))?;

        info!("System tray icon created successfully");

        Ok(Self {
            _tray_icon: tray_icon,
            receiver,
        })
    }

    /// Try to receive a tray event (non-blocking)
    pub fn try_recv(&self) -> Option<TrayEvent> {
        self.receiver.try_recv().ok()
    }
}

/// Load the app logo and convert it to RGBA format for the tray icon
/// Returns (rgba_data, width, height)
fn load_tray_icon_from_logo() -> Result<(Vec<u8>, u32, u32), Box<dyn std::error::Error>> {
    // Decode PNG and convert to RGBA
    let img = image::load_from_memory(LOGO_PNG)
        .map_err(|e| format!("Failed to decode logo PNG: {e}"))?
        .to_rgba8();

    // Resize to 22x22 for better quality on retina displays
    const TARGET_SIZE: u32 = 22;
    let rgba_data = image::imageops::resize(
        &img,
        TARGET_SIZE,
        TARGET_SIZE,
        image::imageops::FilterType::Lanczos3,
    )
    .into_raw();

    Ok((rgba_data, TARGET_SIZE, TARGET_SIZE))
}
