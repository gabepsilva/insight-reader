//! Clipboard and selection reading utilities

use tracing::{debug, info, warn};
#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use tracing::trace;

/// Creates a preview string for logging (first 200 chars).
fn text_preview(text: &str) -> String {
    let char_count = text.chars().count();
    if char_count > 200 {
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

/// Copies text to the clipboard.
/// - On macOS: Uses pbcopy
/// - On Linux: Uses wl-copy for Wayland, xclip for X11
/// - On other platforms: Returns an error
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        copy_to_clipboard_macos(text)
    }
    
    #[cfg(target_os = "linux")]
    {
        copy_to_clipboard_linux(text)
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        warn!("Platform not supported for clipboard copy");
        Err("Clipboard copy not supported on this platform".to_string())
    }
}

#[cfg(target_os = "macos")]
fn copy_to_clipboard_macos(text: &str) -> Result<(), String> {
    use std::process::Command;
    
    info!("Copying text to macOS clipboard using pbcopy");
    
    let mut cmd = Command::new("pbcopy");
    cmd.stdin(std::process::Stdio::piped());
    
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            warn!(error = %e, "Failed to execute pbcopy command");
            return Err(format!("Failed to execute pbcopy: {}", e));
        }
    };
    
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        if let Err(e) = stdin.write_all(text.as_bytes()) {
            warn!(error = %e, "Failed to write to pbcopy stdin");
            return Err(format!("Failed to write to clipboard: {}", e));
        }
    }
    
    match child.wait() {
        Ok(status) => {
            if status.success() {
                info!(bytes = text.len(), "Successfully copied text to clipboard");
                Ok(())
            } else {
                let error_msg = format!("pbcopy exited with code: {:?}", status.code());
                warn!(%error_msg, "pbcopy command failed");
                Err(error_msg)
            }
        }
        Err(e) => {
            warn!(error = %e, "Failed to wait for pbcopy process");
            Err(format!("Failed to wait for pbcopy: {}", e))
        }
    }
}

#[cfg(target_os = "linux")]
fn copy_to_clipboard_linux(text: &str) -> Result<(), String> {
    use std::io::Write;
    
    let try_cmd = |cmd: &str, args: &[&str]| -> Result<(), String> {
        trace!(cmd, ?args, "Trying clipboard copy command");
        
        let mut child = match Command::new(cmd).args(args).stdin(std::process::Stdio::piped()).spawn() {
            Ok(child) => child,
            Err(e) => {
                warn!(cmd, error = %e, "Failed to execute clipboard copy command");
                return Err(format!("Failed to execute {}: {}", cmd, e));
            }
        };
        
        if let Some(mut stdin) = child.stdin.take() {
            if let Err(e) = stdin.write_all(text.as_bytes()) {
                warn!(cmd, error = %e, "Failed to write to clipboard command stdin");
                return Err(format!("Failed to write to {}: {}", cmd, e));
            }
        }
        
        match child.wait() {
            Ok(status) => {
                if status.success() {
                    info!(cmd, bytes = text.len(), "Successfully copied text to clipboard");
                    Ok(())
                } else {
                    let stderr = format!("{} exited with code: {:?}", cmd, status.code());
                    warn!(cmd, %stderr, "Clipboard copy command failed");
                    Err(stderr)
                }
            }
            Err(e) => {
                warn!(cmd, error = %e, "Failed to wait for clipboard copy process");
                Err(format!("Failed to wait for {}: {}", cmd, e))
            }
        }
    };
    
    // Try wl-copy first (Wayland), fallback to xclip (X11)
    info!("Attempting to copy text to clipboard");
    try_cmd("wl-copy", &[])
        .or_else(|_| {
            debug!("wl-copy failed, trying xclip");
            try_cmd("xclip", &["-selection", "clipboard"])
        })
}

