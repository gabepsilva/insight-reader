//! Windows-specific text extraction implementation

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, error, info, warn};

use dirs;

/// Find Python interpreter in the venv (Windows uses Scripts\ instead of bin/)
fn find_venv_python() -> Option<PathBuf> {
    // Check project-local virtualenv first (development)
    if let Ok(current_dir) = env::current_dir() {
        let project_python = current_dir.join("venv").join("Scripts").join("python.exe");
        if project_python.exists() {
            return Some(project_python);
        }
    }
    
    // Check user installation (Windows: %LOCALAPPDATA%\insight-reader)
    if let Some(data_dir) = dirs::data_local_dir() {
        let user_python = data_dir.join("insight-reader").join("venv").join("Scripts").join("python.exe");
        if user_python.exists() {
            return Some(user_python);
        }
    }
    
    // Also check data_dir (which may differ on Windows)
    if let Some(data_dir) = dirs::data_dir() {
        let user_python = data_dir.join("insight-reader").join("venv").join("Scripts").join("python.exe");
        if user_python.exists() {
            return Some(user_python);
        }
    }
    
    None
}

/// Extracts text from an image on Windows using Python script with EasyOCR.
pub(super) fn extract_text_from_image_windows(image_path: &str) -> Result<String, String> {
    info!(path = %image_path, "Starting text extraction from image on Windows");
    
    // Verify the image file exists
    if !Path::new(image_path).exists() {
        error!(path = %image_path, "Image file does not exist");
        return Err(format!("Image file does not exist: {}", image_path));
    }
    
    // Find the Python script path: try executable directory, parent, then current directory
    let script_path = env::current_exe()
        .ok()
        .and_then(|exe_path| {
            exe_path.parent()
                .map(|dir| dir.join("extract_text_from_image.py"))
                .filter(|p| p.exists())
        })
        .or_else(|| {
            env::current_exe()
                .ok()
                .and_then(|exe_path| {
                    exe_path.parent()
                        .and_then(|dir| dir.parent())
                        .map(|dir| dir.join("extract_text_from_image.py"))
                        .filter(|p| p.exists())
                })
        })
        .or_else(|| {
            // Check in %LOCALAPPDATA%\insight-reader\bin folder (standard installation location)
            dirs::data_local_dir()
                .map(|data_dir| data_dir.join("insight-reader").join("bin").join("extract_text_from_image.py"))
                .filter(|p| p.exists())
        })
        .or_else(|| {
            // Also check data_dir
            dirs::data_dir()
                .map(|data_dir| data_dir.join("insight-reader").join("bin").join("extract_text_from_image.py"))
                .filter(|p| p.exists())
        })
        .or_else(|| {
            // Check current directory (development)
            Path::new("install/extract_text_from_image.py")
                .exists()
                .then(|| Path::new("install/extract_text_from_image.py").to_path_buf())
        })
        .ok_or_else(|| {
            error!("extract_text_from_image.py script not found");
            "extract_text_from_image.py script not found".to_string()
        })?;
    
    debug!(script = %script_path.display(), "Using Python script for text extraction");
    
    // Find Python interpreter in venv
    let python_interpreter = find_venv_python()
        .unwrap_or_else(|| {
            warn!("Venv Python not found, falling back to system python");
            // On Windows, try 'python' first, then 'py' (Python launcher)
            PathBuf::from("python")
        });
    
    debug!(python = %python_interpreter.display(), "Using Python interpreter for text extraction");
    
    // Execute Python script
    let output = match Command::new(&python_interpreter)
        .arg(script_path.as_os_str())
        .arg(image_path)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            // If 'python' fails, try 'py' (Windows Python launcher)
            if python_interpreter == PathBuf::from("python") {
                debug!("Trying Python launcher 'py' as fallback");
                match Command::new("py")
                    .arg("-3")
                    .arg(script_path.as_os_str())
                    .arg(image_path)
                    .output()
                {
                    Ok(output) => output,
                    Err(e2) => {
                        error!(error = %e2, "Failed to execute python command (tried python and py)");
                        return Err(format!("Failed to execute text extraction: {}", e2));
                    }
                }
            } else {
                error!(error = %e, "Failed to execute python command");
                return Err(format!("Failed to execute text extraction: {}", e));
            }
        }
    };
    
    // Check if the command succeeded
    if !output.status.success() {
        let exit_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Exit code 1 might mean "no text found" (which is not an error)
        // Check if stderr contains an actual error message
        if exit_code == 1 && stderr.trim().is_empty() {
            warn!("No text found in image");
            return Err("No text found in image".to_string());
        }
        
        error!(
            code = exit_code,
            stderr = %stderr.trim(),
            "Text extraction failed"
        );
        return Err(format!("Text extraction failed: {}", stderr.trim()));
    }
    
    let extracted_text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if extracted_text.is_empty() {
        warn!("No text found in image");
        return Err("No text found in image".to_string());
    }
    
    info!(bytes = extracted_text.len(), "Text extracted successfully from image");
    debug!(text = %extracted_text.chars().take(100).collect::<String>(), "Extracted text preview");
    
    Ok(extracted_text)
}
