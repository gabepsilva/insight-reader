//! Clipboard and selection reading utilities

use tracing::{debug, info, warn};
#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use tracing::trace;

/// Creates a preview string for logging (first 200 chars).
fn text_preview(text: &str) -> String {
    if text.chars().count() > 200 {
        format!("{}...", text.chars().take(200).collect::<String>())
    } else {
        text.to_string()
    }
}


/// Gets the currently selected text.
/// - On Linux: Uses wl-paste for Wayland, xclip for X11 (PRIMARY selection)
/// - On macOS: Uses pbpaste to read from clipboard
/// - On other platforms: Returns None
pub fn get_selected_text() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        get_selected_text_macos()
    }
    
    #[cfg(target_os = "linux")]
    {
        get_selected_text_linux()
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        warn!("Platform not supported for text selection");
        None
    }
}

#[cfg(target_os = "linux")]
fn get_selected_text_linux() -> Option<String> {
    let try_cmd = |cmd: &str, args: &[&str]| -> Option<String> {
        trace!(cmd, ?args, "Trying clipboard command");
        
        let output = match Command::new(cmd).args(args).output() {
            Ok(output) => output,
            Err(e) => {
                warn!(cmd, error = %e, "Failed to execute clipboard command");
                return None;
            }
        };
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!(
                cmd,
                code = ?output.status.code(),
                stderr = %stderr.trim(),
                "Clipboard command failed"
            );
            return None;
        }
        
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() {
            trace!(cmd, "Clipboard command returned empty text");
            return None;
        }
        
        Some(text)
    };

    // Try wl-paste first (Wayland), fallback to xclip (X11)
    info!("Attempting to read selected text from clipboard/selection");
    let result = try_cmd("wl-paste", &["--primary", "--no-newline"])
        .or_else(|| {
            debug!("wl-paste failed, trying xclip");
            try_cmd("xclip", &["-selection", "primary", "-o"])
        });

    if let Some(ref text) = result {
        info!(bytes = text.len(), "Successfully retrieved selected text");
        debug!(text = %text_preview(text), "Captured text content");
    } else {
        warn!("No text available from clipboard/selection (no text selected or commands failed)");
    }

    result
}

#[cfg(target_os = "macos")]
fn get_selected_text_macos() -> Option<String> {
    use std::process::Command;
    
    info!("Attempting to read text from macOS clipboard using pbpaste");
    
    let output = match Command::new("pbpaste").output() {
        Ok(output) => output,
        Err(e) => {
            warn!(error = %e, "Failed to execute pbpaste command");
            return None;
        }
    };
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!(
            code = ?output.status.code(),
            stderr = %stderr.trim(),
            "pbpaste command failed"
        );
        return None;
    }
    
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        debug!("Clipboard is empty");
        return None;
    }
    
    info!(bytes = text.len(), "Successfully retrieved text from clipboard");
    debug!(text = %text_preview(&text), "Captured text content");
    Some(text)
}

