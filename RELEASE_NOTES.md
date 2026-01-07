# Release Notes - grars v1.0.0

**First Release** - December 2024

grars is a modern, lightweight Text-to-Speech (TTS) application written in Rust with Iced GUI. This is the initial release with a complete feature set for reading selected text aloud.

## 🎉 What's New

This is the first public release of grars! A complete rewrite of the original Python GTK4 TTS application in Rust, providing better performance, reliability, and a modern user interface.

## ✨ Features

### Multiple TTS Providers

- **Piper TTS** (Local, Offline)
  - Fast, privacy-focused local text-to-speech
  - No internet connection required
  - Uses ONNX models for high-quality synthesis
  - Automatic model detection and installation

- **AWS Polly** (Cloud)
  - High-quality neural voices
  - Professional-grade speech synthesis
  - Requires AWS credentials configuration
  - Automatic credential detection and validation

### Modern User Interface

- **Floating Borderless Window**
  - Clean, minimal design that stays out of your way
  - Transparent background with modern styling
  - Compact 360×70px window

- **Real-time Waveform Visualization**
  - FFT-driven animated waveform bars
  - 10-band frequency visualization
  - Smooth 75ms update intervals
  - Visual feedback during playback

- **Playback Controls**
  - Play/Pause toggle
  - Stop button
  - Skip forward/backward (5 seconds)
  - Dynamic progress bar showing playback position

### Settings & Configuration

- **Settings Window**
  - Easy provider selection (Piper/Polly)
  - Log level configuration (Error, Warn, Info, Debug, Trace)
  - Error message display
  - Persistent settings saved to `~/.config/grars/config.json`

### Smart Features

- **Automatic Text Detection**
  - Reads selected text from clipboard/selection at startup
  - Works with both Wayland (`wl-clipboard`) and X11 (`xclip`)
  - Supports primary selection on Linux

- **Auto-close on Completion**
  - Window automatically closes when playback finishes
  - Handles edge cases (stop, errors) gracefully

- **Error Handling**
  - User-friendly error messages
  - Automatic settings window opening on credential errors
  - Graceful fallback if provider is unavailable

### Logging & Debugging

- **Comprehensive Logging**
  - Structured logging with `tracing`
  - Logs to both stderr and files
  - Log files: `~/.local/share/grars/logs/grars-YYYY-MM-DD.log`
  - Configurable log levels

## 🛠️ Technical Details

### Architecture

- Built with **Iced 0.14** GUI framework
- Async-capable application with subscriptions
- Elm Architecture pattern (Message → Update → View)
- Modular provider system for easy extensibility

### Audio Processing

- **rodio** for audio playback
- **rustfft** for FFT-based frequency analysis
- Real-time audio processing and visualization
- Support for 16kHz sample rate (Polly neural voices)

### Platform Support

- **Tested Platforms:**
  - Ubuntu with GNOME (Wayland)
  - Arch Linux with Hyprland (Wayland)
- **Build Targets:**
  - Linux x86_64
  - Linux ARM64 (aarch64)
  - macOS x86_64 and ARM64 (via cross-compilation)

## 📦 Installation

### Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/gabepsilva/grars/main/install.sh | bash
```

### Manual Installation

See [README.md](README.md) for detailed installation instructions.

## 🔧 Configuration

### AWS Polly Setup

Configure AWS credentials via:
- Environment variables: `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`
- Credentials file: `~/.aws/credentials`
- Named profiles: `AWS_PROFILE` environment variable

### Application Config

Settings are automatically saved to `~/.config/grars/config.json`:
- Voice provider selection
- Log level preference

## 🐛 Known Issues

- CLI interface not yet implemented (planned for future release)
- Cross-compilation may require additional setup for native dependencies

## 🙏 Acknowledgments

- Built with [Iced](https://iced.rs/) GUI framework
- Uses [Piper TTS](https://github.com/rhasspy/piper) for local TTS
- AWS Polly integration via AWS SDK for Rust

## 📝 What's Next

Future enhancements planned:
- CLI interface with argument parsing
- Additional TTS provider support
- Enhanced configuration options
- Performance optimizations

---

**Full Changelog**: This is the initial release. See [plan.md](plan.md) for implementation details.

