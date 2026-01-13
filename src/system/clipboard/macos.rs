//! macOS-specific clipboard implementation

use super::process_text;

/// Gets the currently selected text on macOS.
pub(super) fn get_selected_text_macos() -> Option<String> {
    use arboard::Clipboard;
    
    Clipboard::new()
        .ok()?
        .get_text()
        .ok()
        .and_then(|text| process_text(text, "clipboard"))
}
