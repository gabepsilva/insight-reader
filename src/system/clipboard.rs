//! Clipboard and selection reading utilities

#[cfg(target_os = "macos")]
use core_foundation::string::CFString;
#[cfg(target_os = "linux")]
use std::process::Command;

use tracing::{debug, info, warn};
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
/// - On macOS: Uses Accessibility API to read selected text directly (no clipboard modification)
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
    info!("Attempting to read selected text on macOS using Accessibility API");
    
    use core_foundation::base::TCFType;
    use core_foundation::string::CFStringRef;
    use std::ffi::c_void;
    use std::os::raw::c_uint;
    use std::ptr;
    
    // Opaque pointer types (avoiding experimental extern types)
    #[repr(C)]
    struct AXUIElement([u8; 0]);
    
    // Link against ApplicationServices framework
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn AXUIElementCreateSystemWide() -> *mut AXUIElement;
        fn AXUIElementCopyAttributeValue(
            element: *mut AXUIElement,
            attribute: CFStringRef,
            value: *mut *mut c_void,
        ) -> c_uint;
        fn CFRelease(cf: *const c_void);
    }
    
    // Error codes
    const K_AX_ERROR_SUCCESS: c_uint = 0;
    
    // Constants for Accessibility attributes
    let k_ax_focused_ui_element: &str = "AXFocusedUIElement";
    let k_ax_selected_text: &str = "AXSelectedText";
    
    unsafe {
        // Get system-wide accessibility element
        let system_element = AXUIElementCreateSystemWide();
        if system_element.is_null() {
            warn!("Failed to create system-wide accessibility element");
            return None;
        }
        
        // Get focused UI element directly from system-wide element
        let focused_ui_attr = CFString::new(k_ax_focused_ui_element);
        let mut focused_ui: *mut c_void = ptr::null_mut();
        let result = AXUIElementCopyAttributeValue(
            system_element,
            focused_ui_attr.as_concrete_TypeRef(),
            &mut focused_ui,
        );
        
        CFRelease(system_element as *const c_void);
        
        if result != K_AX_ERROR_SUCCESS || focused_ui.is_null() {
            debug!("Could not get focused UI element (may need accessibility permissions or no text selected)");
            return None;
        }
        
        // Get selected text
        let selected_text_attr = CFString::new(k_ax_selected_text);
        let mut selected_text_value: *mut c_void = ptr::null_mut();
        let result = AXUIElementCopyAttributeValue(
            focused_ui as *mut AXUIElement,
            selected_text_attr.as_concrete_TypeRef(),
            &mut selected_text_value,
        );
        
        CFRelease(focused_ui as *const c_void);
        
        if result != K_AX_ERROR_SUCCESS || selected_text_value.is_null() {
            debug!("No text selected or element does not support AXSelectedText");
            return None;
        }
        
        // Convert CFString to Rust String
        let cf_string = CFString::wrap_under_get_rule(selected_text_value as CFStringRef);
        let text = cf_string.to_string();
        CFRelease(selected_text_value);
        
        if text.trim().is_empty() {
            debug!("Selected text is empty");
            return None;
        }
        
        info!(bytes = text.len(), "Successfully retrieved selected text via Accessibility API");
        debug!(text = %text_preview(&text), "Captured text content");
        Some(text)
    }
}

