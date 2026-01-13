//! macOS-specific screenshot capture implementation

use std::env;
use std::process::Command;
use tracing::{debug, error, info};

/// Captures a screenshot region on macOS using screencapture.
pub(super) fn capture_region_macos() -> Result<String, String> {
    info!("Starting interactive screenshot region selection");
    
    // Create temporary file path for the screenshot
    let temp_dir = env::temp_dir();
    let screenshot_path = temp_dir.join("insight-reader-screenshot.png");
    
    debug!(path = %screenshot_path.display(), "Screenshot will be saved to temp file");
    
    // Execute screencapture with -i flag for interactive region selection
    // -i: interactive mode (shows crosshair for region selection)
    // The user can press Escape to cancel
    let output = match Command::new("screencapture")
        .arg("-i")
        .arg(screenshot_path.as_os_str())
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            error!(error = %e, "Failed to execute screencapture command");
            return Err(format!("Failed to execute screenshot command: {}", e));
        }
    };
    
    // Check if the command succeeded
    if !output.status.success() {
        let exit_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Exit code 1 typically means user cancelled (Escape key)
        if exit_code == 1 {
            debug!("User cancelled screenshot selection");
            return Err("Screenshot selection cancelled".to_string());
        }
        
        error!(
            code = exit_code,
            stderr = %stderr.trim(),
            "screencapture command failed"
        );
        return Err(format!("Screenshot failed: {}", stderr.trim()));
    }
    
    // Verify the file was actually created
    if !screenshot_path.exists() {
        error!(path = %screenshot_path.display(), "Screenshot file was not created");
        return Err("Screenshot file was not created".to_string());
    }
    
    // Get the file path as a string
    let path_str = screenshot_path.to_string_lossy().to_string();
    info!(path = %path_str, "Screenshot captured successfully");
    
    Ok(path_str)
}
