//! macOS-specific clipboard implementation using Cmd+C simulation

use super::process_text;
use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings as EnigoSettings};
use macos_accessibility_client::accessibility::application_is_trusted_with_prompt;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Check if we have accessibility permissions (macOS only).
/// Will prompt the user to grant permissions if not already granted.
fn check_accessibility_permissions() -> bool {
    let trusted = application_is_trusted_with_prompt();
    if !trusted {
        warn!("Accessibility permissions not granted - enable in System Settings > Privacy & Security > Accessibility");
    }
    trusted
}

/// Simulates Cmd+C using enigo keyboard simulation.
fn simulate_cmd_c() -> Result<(), String> {
    let mut enigo = Enigo::new(&EnigoSettings::default())
        .map_err(|e| format!("Failed to create keyboard simulator: {}", e))?;

    enigo
        .key(Key::Meta, Direction::Press)
        .map_err(|e| format!("Failed to press Cmd: {}", e))?;
    enigo
        .key(Key::Unicode('c'), Direction::Click)
        .map_err(|e| format!("Failed to press 'c': {}", e))?;
    enigo
        .key(Key::Meta, Direction::Release)
        .map_err(|e| format!("Failed to release Cmd: {}", e))?;

    Ok(())
}

/// Polls clipboard for new content, checking every 50ms up to max_wait.
/// Returns the text if clipboard has content, None otherwise.
fn poll_clipboard_for_text(max_wait: Duration) -> Option<String> {
    let poll_interval = Duration::from_millis(50);
    let mut elapsed = Duration::ZERO;

    while elapsed < max_wait {
        std::thread::sleep(poll_interval);
        elapsed += poll_interval;

        if let Some(text) = Clipboard::new()
            .and_then(|mut cb| cb.get_text())
            .ok()
            .filter(|t| !t.is_empty())
        {
            debug!(elapsed_ms = elapsed.as_millis(), "Clipboard updated");
            return Some(text);
        }
    }
    None
}

/// Gets the currently selected text on macOS using Cmd+C simulation.
///
/// This works by:
/// 1. Saving the current clipboard contents
/// 2. Simulating Cmd+C to copy selected text
/// 3. Polling clipboard until it updates (or timeout)
/// 4. Restoring the original clipboard contents
///
/// This workaround is necessary because macOS doesn't provide direct API access
/// to read selected text from other applications.
pub(super) fn get_selected_text_macos() -> Option<String> {
    debug!("Capturing selected text via Cmd+C simulation");

    // Let system settle after hotkey press
    std::thread::sleep(Duration::from_millis(100));

    // Check accessibility permissions
    if !check_accessibility_permissions() {
        return None;
    }

    // Save current clipboard contents
    let mut clipboard = match Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            warn!(error = %e, "Failed to access clipboard");
            return None;
        }
    };

    let original_text = clipboard.get_text().ok();

    // Clear clipboard to detect when copy completes
    let _ = clipboard.clear();

    // Simulate Cmd+C
    if let Err(e) = simulate_cmd_c() {
        warn!(error = %e, "Failed to simulate Cmd+C");
        restore_clipboard(original_text);
        return None;
    }

    // Poll clipboard for new content (up to 300ms)
    let selected_text = poll_clipboard_for_text(Duration::from_millis(300));

    if let Some(text) = &selected_text {
        info!(chars = text.len(), "Captured selected text");
    } else {
        debug!("No text selected or clipboard didn't update");
    }

    // Restore original clipboard contents
    restore_clipboard(original_text);

    // Process and return
    selected_text.and_then(|text| process_text(text, "selected text"))
}

/// Restores clipboard to its original contents.
fn restore_clipboard(original_text: Option<String>) {
    let Ok(mut clipboard) = Clipboard::new() else {
        return;
    };
    match original_text {
        Some(text) => {
            let _ = clipboard.set_text(text);
        }
        None => {
            let _ = clipboard.clear();
        }
    }
}
