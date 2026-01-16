# Release QA Checklist

## Installation Testing

### Linux - Manjaro KDE
- [ ] Install via curl pipe bash: `curl -fsSL https://insightreader.xyz/install.sh | bash`
- [ ] Verify binary is installed to `~/.local/bin/insight-reader`
- [ ] Verify binary is in PATH (or verify PATH warning message)
- [ ] Verify desktop file is created at `~/.local/share/applications/insight-reader.desktop`
- [ ] Verify icon is installed at `~/.local/share/icons/hicolor/scalable/apps/insight-reader.svg`
- [ ] Verify Python venv is created at `~/.local/share/insight-reader/venv/`
- [ ] Verify Piper is installed in venv (`~/.local/share/insight-reader/venv/bin/piper`)
- [ ] Verify default model is downloaded to `~/.local/share/insight-reader/models/`
- [ ] Verify OCR script is installed at `~/.local/share/insight-reader/bin/extract_text_from_image.py`
- [ ] Verify EasyOCR dependencies are installed in venv
- [ ] Verify application appears in KDE application menu
- [ ] Test installation on fresh Manjaro KDE system (no prior installation)
- [ ] Test installation on system with existing installation (upgrade scenario)

### Linux - Ubuntu Minimal
- [ ] Install via curl pipe bash: `curl -fsSL https://insightreader.xyz/install.sh | bash`
- [ ] Verify all dependencies are auto-installed (python3, espeak-ng, build tools)
- [ ] Verify binary is installed correctly
- [ ] Verify Python venv is created correctly
- [ ] Verify Piper is installed correctly
- [ ] Verify default model is downloaded
- [ ] Verify OCR script is installed
- [ ] Test installation on fresh Ubuntu Minimal system
- [ ] Test installation on system with existing installation

### Linux - Fedora GNOME
- [ ] Install via curl pipe bash: `curl -fsSL https://insightreader.xyz/install.sh | bash`
- [ ] Verify all dependencies are auto-installed via dnf
- [ ] Verify binary is installed correctly
- [ ] Verify Python venv is created correctly
- [ ] Verify Piper is installed correctly
- [ ] Verify default model is downloaded
- [ ] Verify OCR script is installed
- [ ] Verify application appears in GNOME application menu
- [ ] Test installation on fresh Fedora GNOME system
- [ ] Test installation on system with existing installation

### Windows 11
- [ ] Install via PowerShell: `iwr https://insightreader.xyz/install.ps1 | iex`
- [ ] Verify binary is installed to `%LOCALAPPDATA%\insight-reader\bin\insight-reader.exe`
- [ ] Verify binary is added to user PATH
- [ ] Verify Python venv is created at `%LOCALAPPDATA%\insight-reader\venv\`
- [ ] Verify Piper is installed in venv (`%LOCALAPPDATA%\insight-reader\venv\Scripts\piper.exe`)
- [ ] Verify default model is downloaded to `%LOCALAPPDATA%\insight-reader\models\`
- [ ] Verify Start Menu shortcut is created
- [ ] Verify Desktop shortcut is created
- [ ] Verify shortcuts have correct icon
- [ ] Test Python auto-installation via winget (if Python not present)
- [ ] Test installation on fresh Windows 11 system
- [ ] Test installation on system with existing installation
- [ ] Verify Windows Media OCR API is available (no Python script needed)

### macOS
- [ ] Install via curl pipe bash: `curl -fsSL https://insightreader.xyz/install.sh | bash`
- [ ] Verify binary is installed to `~/.local/bin/insight-reader`
- [ ] Verify app bundle is created at `/Applications/insight-reader.app`
- [ ] Verify Python venv is created at `~/.local/share/insight-reader/venv/`
- [ ] Verify Piper is installed in venv (`~/.local/share/insight-reader/venv/bin/piper`)
- [ ] Verify default model is downloaded to `~/.local/share/insight-reader/models/`
- [ ] Verify OCR script is installed at `~/.local/share/insight-reader/bin/extract_text_from_image.swift`
- [ ] Verify app bundle has correct Info.plist
- [ ] Verify app bundle has correct icon (ICNS or PNG)
- [ ] Test Homebrew auto-installation (if Homebrew not present)
- [ ] Test installation on fresh macOS system
- [ ] Test installation on system with existing installation
- [ ] Verify macOS Vision framework is available for OCR

## Core Functionality

### Application Launch
- [ ] Application starts without errors
- [ ] Main window appears (floating, borderless, always on top)
- [ ] Window is positioned at bottom-left corner with margin
- [ ] Window size is correct (410x70)
- [ ] Application reads selected text from clipboard automatically on startup
- [ ] Application reads selected text from system selection automatically on startup (Linux/macOS)
- [ ] Application handles case when no text is selected/copied
- [ ] Logs are written to correct location:
  - [ ] Linux/macOS: `~/.local/share/insight-reader/logs/insight-reader-YYYY-MM-DD.log`
  - [ ] Windows: `%LOCALAPPDATA%\insight-reader\logs\insight-reader-YYYY-MM-DD.log`

### Main Window UI
- [ ] Floating borderless window displays correctly
- [ ] Window can be dragged by clicking and holding
- [ ] Window stays always on top
- [ ] Real-time waveform visualization displays during playback
- [ ] Waveform animates smoothly (~75ms updates)
- [ ] Play button is visible and functional
- [ ] Pause button is visible and functional
- [ ] Stop button is visible and functional
- [ ] Skip forward button (5 seconds) works
- [ ] Skip backward button (5 seconds) works
- [ ] Progress bar displays and updates during playback
- [ ] Window auto-closes when playback completes
- [ ] Window closes when stop button is pressed

### TTS Providers

#### Piper TTS (Local)
- [ ] Piper provider can be selected in settings
- [ ] Piper voices are listed correctly
- [ ] Default voice is selected on first launch
- [ ] Voice selection persists across restarts
- [ ] Text is synthesized correctly with Piper
- [ ] Audio playback works with Piper
- [ ] Play/pause/stop controls work with Piper
- [ ] Skip forward/backward works with Piper
- [ ] Progress tracking works with Piper
- [ ] Waveform visualization works with Piper
- [ ] Multiple voices can be tested and switched
- [ ] Voice download interface works
- [ ] Voices can be downloaded from Hugging Face
- [ ] Downloaded voices appear in voice list
- [ ] Language flags display correctly in voice download interface
- [ ] Voice preview works (if implemented)

#### AWS Polly (Cloud)
- [ ] AWS Polly provider can be selected in settings
- [ ] AWS credentials are detected correctly (environment variables, credentials file, profile)
- [ ] AWS Polly voices are fetched and listed correctly
- [ ] Voice engines are selectable (Standard, Neural, Generative, LongForm)
- [ ] Engine selection persists across restarts
- [ ] Text is synthesized correctly with AWS Polly
- [ ] Audio playback works with AWS Polly
- [ ] Play/pause/stop controls work with AWS Polly
- [ ] Skip forward/backward works with AWS Polly
- [ ] Progress tracking works with AWS Polly
- [ ] Waveform visualization works with AWS Polly
- [ ] Error handling works when AWS credentials are invalid
- [ ] Error handling works when AWS service is unavailable
- [ ] AWS Polly pricing information modal displays correctly
- [ ] Multiple voices can be tested and switched
- [ ] Multiple languages can be tested

### Settings Window
- [ ] Settings window opens from main window or tray menu
- [ ] Settings window is scrollable
- [ ] Provider selection (Piper/AWS Polly) works
- [ ] Provider selection persists across restarts
- [ ] Voice selection dropdown works
- [ ] Voice selection persists across restarts
- [ ] Engine selection (for AWS Polly) works
- [ ] Engine selection persists across restarts
- [ ] Log level selection works
- [ ] Log level selection persists across restarts
- [ ] Natural Reading toggle works
- [ ] Natural Reading toggle persists across restarts
- [ ] Hotkey configuration UI works
- [ ] Hotkey can be captured live
- [ ] Hotkey configuration persists across restarts
- [ ] Settings are saved to config file:
  - [ ] Linux/macOS: `~/.config/insight-reader/config.json`
  - [ ] Windows: `%APPDATA%\insight-reader\config.json`
- [ ] Error messages display correctly in settings window
- [ ] Settings window can be closed
- [ ] Settings window can be reopened

### System Integration

#### System Tray
- [ ] System tray icon appears (Windows, macOS, Linux)
- [ ] Tray icon is visible in system tray/menu bar
- [ ] Tray menu displays correctly:
  - [ ] "Read selected text" option
  - [ ] "Show/Hide window" option
  - [ ] Configured hotkey display (if enabled)
  - [ ] "Quit" option
- [ ] "Read selected text" from tray works
- [ ] "Show/Hide window" from tray works
- [ ] "Quit" from tray works
- [ ] Tray icon works on Linux with AppIndicator support (GNOME extension)
- [ ] Application continues to work if tray icon fails to initialize

#### Global Hotkeys
- [ ] Global hotkeys work on Windows
- [ ] Global hotkeys work on macOS
- [ ] Global hotkeys are disabled on Linux Wayland (with appropriate message)
- [ ] Hotkey can be configured in settings
- [ ] Hotkey capture UI works (live capture)
- [ ] Hotkey can be cancelled with Escape key
- [ ] Configured hotkey triggers text reading
- [ ] Hotkey works when application is in background
- [ ] Hotkey works when main window is hidden
- [ ] Hotkey is displayed in tray menu
- [ ] Hotkey configuration persists across restarts

#### Clipboard/Text Selection
- [ ] Clipboard reading works on Windows
- [ ] Selected text reading works on Linux (X11/Wayland)
- [ ] Selected text reading works on macOS
- [ ] Text is read automatically on application startup
- [ ] Text can be read via tray menu "Read selected text"
- [ ] Text can be read via global hotkey
- [ ] Application handles empty clipboard/selection gracefully
- [ ] Application handles very long text correctly
- [ ] Application handles special characters correctly
- [ ] Application handles multiple languages correctly

### OCR Functionality

#### Windows OCR
- [ ] OCR button/option is available
- [ ] Screenshot capture works (region selection)
- [ ] Windows Media OCR API extracts text correctly
- [ ] Extracted text is displayed correctly
- [ ] Extracted text can be read with TTS
- [ ] OCR works with different languages (user's profile languages)
- [ ] Screenshot selection can be cancelled with Escape
- [ ] OCR handles images with no text gracefully

#### macOS OCR
- [ ] OCR button/option is available
- [ ] Screenshot capture works
- [ ] macOS Vision framework extracts text correctly
- [ ] Extracted text is displayed correctly
- [ ] Extracted text can be read with TTS
- [ ] OCR works with different languages
- [ ] Screen recording permission is requested (if needed)
- [ ] OCR handles images with no text gracefully

#### Linux OCR
- [ ] OCR button/option is available
- [ ] Screenshot capture works
- [ ] EasyOCR extracts text correctly
- [ ] Extracted text is displayed correctly
- [ ] Extracted text can be read with TTS
- [ ] OCR works with different languages
- [ ] EasyOCR is installed in venv correctly
- [ ] OCR handles images with no text gracefully
- [ ] OCR script (`extract_text_from_image.py`) is executable and works

### Natural Reading (Text Cleanup)
- [ ] Natural Reading toggle is available in settings
- [ ] Natural Reading can be enabled/disabled
- [ ] Text cleanup works when enabled:
  - [ ] Removes extra whitespace
  - [ ] Fixes common formatting issues
  - [ ] Preserves line breaks appropriately
- [ ] Text cleanup can be disabled
- [ ] Natural Reading info modal displays correctly
- [ ] Natural Reading setting persists across restarts

### Voice Management
- [ ] Voice selection window opens correctly
- [ ] Voice list displays correctly (Piper voices)
- [ ] Voice list displays correctly (AWS Polly voices)
- [ ] Voice download interface opens correctly
- [ ] Voice download interface shows language flags
- [ ] Voices can be filtered by language
- [ ] Voices can be downloaded from Hugging Face
- [ ] Download progress is displayed
- [ ] Downloaded voices appear in voice list
- [ ] Voice selection persists across restarts
- [ ] Voice files are stored in correct location:
  - [ ] Linux/macOS: `~/.local/share/insight-reader/models/`
  - [ ] Windows: `%LOCALAPPDATA%\insight-reader\models\`

### Error Handling
- [ ] Application handles missing TTS provider gracefully
- [ ] Application handles TTS provider errors gracefully
- [ ] Error messages are displayed in settings window
- [ ] Application handles missing voice files gracefully
- [ ] Application handles network errors (AWS Polly) gracefully
- [ ] Application handles invalid AWS credentials gracefully
- [ ] Application handles missing OCR dependencies gracefully
- [ ] Application handles clipboard access errors gracefully
- [ ] Application handles hotkey registration errors gracefully
- [ ] Application handles system tray initialization errors gracefully
- [ ] Application logs errors to log file correctly

### Performance
- [ ] Application starts quickly (< 2 seconds)
- [ ] UI is responsive during playback
- [ ] Waveform animation is smooth (no stuttering)
- [ ] Audio playback is smooth (no stuttering)
- [ ] Memory usage is reasonable (< 200MB typical)
- [ ] CPU usage is reasonable during playback
- [ ] Application handles long text efficiently
- [ ] Application handles multiple rapid operations gracefully

### Cross-Platform Specific

#### Windows Specific
- [ ] Windows Media OCR API works correctly
- [ ] Windows shortcuts are created correctly
- [ ] Windows PATH is updated correctly
- [ ] Application works with Windows 10
- [ ] Application works with Windows 11
- [ ] Global hotkeys work on Windows
- [ ] System tray icon works on Windows

#### macOS Specific
- [ ] macOS Vision framework works correctly
- [ ] App bundle is created correctly
- [ ] App bundle can be launched from Applications
- [ ] App bundle can be launched from Spotlight
- [ ] Screen recording permission is requested correctly
- [ ] Accessibility permissions are requested correctly (if needed)
- [ ] Global hotkeys work on macOS
- [ ] System tray (menu bar) icon works on macOS
- [ ] Application works on Apple Silicon (M1/M2/M3)
- [ ] Application works on Intel Macs

#### Linux Specific
- [ ] EasyOCR works correctly
- [ ] Desktop file is created correctly
- [ ] Icon is installed correctly
- [ ] Application appears in application menu
- [ ] System tray icon works with AppIndicator (GNOME extension)
- [ ] Application works on X11
- [ ] Application works on Wayland
- [ ] Application handles Wayland hotkey limitations correctly
- [ ] Application works on different desktop environments (GNOME, KDE, etc.)

## Regression Testing

### Configuration Persistence
- [ ] All settings persist across application restarts
- [ ] Config file is created in correct location
- [ ] Config file format is valid JSON
- [ ] Config file is readable and writable
- [ ] Config file survives application crashes

### Audio Playback
- [ ] Audio plays correctly with both providers
- [ ] Audio can be paused and resumed
- [ ] Audio can be stopped
- [ ] Audio can be skipped forward/backward
- [ ] Audio progress is tracked correctly
- [ ] Audio waveform visualization works correctly
- [ ] Multiple audio streams don't conflict

### Window Management
- [ ] Main window opens correctly
- [ ] Settings window opens correctly
- [ ] Voice selection window opens correctly
- [ ] Multiple windows can be open simultaneously
- [ ] Windows close correctly
- [ ] Window positions are remembered (if implemented)

## Documentation Verification
- [ ] README.md installation instructions are accurate
- [ ] README.md feature list is accurate
- [ ] README.md troubleshooting section is accurate
- [ ] README.md platform support table is accurate
- [ ] Installation scripts match README instructions

## Final Checks
- [ ] All tests pass on all target platforms
- [ ] No critical bugs or crashes
- [ ] Performance is acceptable
- [ ] Memory leaks are not present (test with extended use)
- [ ] Application can be uninstalled cleanly
- [ ] Release notes are prepared
- [ ] Version number is correct
- [ ] Git tags are created correctly
- [ ] GitHub release is created with correct binaries
- [ ] Installation URLs point to correct release
