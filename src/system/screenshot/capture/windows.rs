//! Windows-specific screenshot capture implementation

use std::env;
use std::process::Command;
use tracing::{debug, error, info};

/// PowerShell script for interactive screenshot region selection using Windows Forms.
/// This creates a translucent overlay and allows the user to draw a rectangle for capture.
const SCREENSHOT_PS_SCRIPT: &str = r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

# Get all screens for multi-monitor support
$screens = [System.Windows.Forms.Screen]::AllScreens
$bounds = [System.Drawing.Rectangle]::Empty
foreach ($screen in $screens) {
    $bounds = [System.Drawing.Rectangle]::Union($bounds, $screen.Bounds)
}

# Create a full-screen bitmap
$bitmap = New-Object System.Drawing.Bitmap($bounds.Width, $bounds.Height)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)

# Capture the entire screen
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$graphics.Dispose()

# Create a form for region selection
$form = New-Object System.Windows.Forms.Form
$form.FormBorderStyle = 'None'
$form.StartPosition = 'Manual'
$form.Location = $bounds.Location
$form.Size = $bounds.Size
$form.TopMost = $true
$form.BackgroundImage = $bitmap
$form.Cursor = [System.Windows.Forms.Cursors]::Cross

# Variables for selection
$script:startPoint = $null
$script:endPoint = $null
$script:selecting = $false
$script:cancelled = $false

# Create a picture box for drawing the selection rectangle
$pictureBox = New-Object System.Windows.Forms.PictureBox
$pictureBox.Dock = 'Fill'
$pictureBox.Image = $bitmap.Clone()
$form.Controls.Add($pictureBox)

$pictureBox.Add_MouseDown({
    param($sender, $e)
    if ($e.Button -eq 'Left') {
        $script:startPoint = $e.Location
        $script:selecting = $true
    }
})

$pictureBox.Add_MouseMove({
    param($sender, $e)
    if ($script:selecting) {
        $script:endPoint = $e.Location
        # Redraw with selection rectangle
        $tempBitmap = $bitmap.Clone()
        $g = [System.Drawing.Graphics]::FromImage($tempBitmap)
        $rect = New-Object System.Drawing.Rectangle(
            [Math]::Min($script:startPoint.X, $script:endPoint.X),
            [Math]::Min($script:startPoint.Y, $script:endPoint.Y),
            [Math]::Abs($script:endPoint.X - $script:startPoint.X),
            [Math]::Abs($script:endPoint.Y - $script:startPoint.Y)
        )
        $pen = New-Object System.Drawing.Pen([System.Drawing.Color]::Red, 2)
        $brush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(50, 0, 120, 215))
        $g.FillRectangle($brush, $rect)
        $g.DrawRectangle($pen, $rect)
        $g.Dispose()
        $pen.Dispose()
        $brush.Dispose()
        $pictureBox.Image = $tempBitmap
    }
})

$pictureBox.Add_MouseUp({
    param($sender, $e)
    if ($e.Button -eq 'Left' -and $script:selecting) {
        $script:endPoint = $e.Location
        $script:selecting = $false
        $form.Close()
    }
})

$form.Add_KeyDown({
    param($sender, $e)
    if ($e.KeyCode -eq 'Escape') {
        $script:cancelled = $true
        $form.Close()
    }
})

[System.Windows.Forms.Application]::Run($form)

if ($script:cancelled -or $script:startPoint -eq $null -or $script:endPoint -eq $null) {
    $bitmap.Dispose()
    exit 1
}

# Calculate selection rectangle
$x = [Math]::Min($script:startPoint.X, $script:endPoint.X)
$y = [Math]::Min($script:startPoint.Y, $script:endPoint.Y)
$width = [Math]::Abs($script:endPoint.X - $script:startPoint.X)
$height = [Math]::Abs($script:endPoint.Y - $script:startPoint.Y)

if ($width -lt 5 -or $height -lt 5) {
    $bitmap.Dispose()
    exit 1
}

# Crop the selection
$rect = New-Object System.Drawing.Rectangle($x, $y, $width, $height)
$cropped = $bitmap.Clone($rect, $bitmap.PixelFormat)
$bitmap.Dispose()

# Save to the output path (passed as argument)
$outputPath = $args[0]
$cropped.Save($outputPath, [System.Drawing.Imaging.ImageFormat]::Png)
$cropped.Dispose()

exit 0
"#;

/// Captures a screenshot region on Windows using PowerShell with Windows Forms.
pub(super) fn capture_region_windows() -> Result<String, String> {
    info!("Starting interactive screenshot region selection on Windows");
    
    let screenshot_path = env::temp_dir().join("insight-reader-screenshot.png");
    debug!(path = %screenshot_path.display(), "Screenshot will be saved to temp file");
    
    // Execute PowerShell script for region selection
    let output = match Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy", "Bypass",
            "-Command", SCREENSHOT_PS_SCRIPT,
            screenshot_path.to_str().unwrap_or(""),
        ])
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            error!(error = %e, "Failed to execute PowerShell screenshot command");
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
            "PowerShell screenshot command failed"
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
