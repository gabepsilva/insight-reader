//! Windows-specific clipboard implementation

use super::process_text;

/// Gets the currently selected text on Windows.
/// Windows doesn't have a PRIMARY selection like Linux, so we only read from clipboard.
pub(super) fn get_selected_text_windows() -> Option<String> {
    use arboard::Clipboard;

    Clipboard::new()
        .ok()?
        .get_text()
        .ok()
        .and_then(|text| process_text(text, "clipboard"))
}
