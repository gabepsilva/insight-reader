# Release Notes

## Version 1.0.2

### 🎉 Major Changes Since v1.0.1

#### Rebranding
- **Complete rebrand from "grars" to "Insight Reader"**
  - All references updated throughout the codebase
  - New branding applied to UI, documentation, and configuration files
  - Configuration path changed from `~/.config/grars/` to `~/.config/insight-reader/`

#### New Features

**🎤 Enhanced TTS Capabilities**
- **AWS Polly Voice Selection**: Added comprehensive voice selection interface for AWS Polly
  - Browse and select from available AWS Polly voices
  - Voice metadata management with language and voice information
  - New `src/voices/` module for voice management
  - Improved AWS Polly integration with better error handling

- **Piper TTS Speed and Pause Controls**: 
  - Added playback speed control for Piper TTS
  - Enhanced pause/resume functionality
  - Better audio playback control

**🎨 UI/UX Improvements**

- **Window Management**:
  - Added window dragging capability
  - Bottom-left positioning option for the floating window
  - Improved window positioning and user control

- **Settings Dialog**:
  - Modern layout with scrollable content
  - Improved visual design and user experience
  - Better organization of settings options
  - Enhanced UI responsiveness

**🔧 Installation & Deployment**

- **Release Management**:
  - Updated to use GitHub `/latest/download/` URL
  - Show release tag in application logs
  - Improved release tracking and version display

### 🔄 Code Quality

- **Refactoring**:
  - Code simplifications and UI improvements
  - Better code organization and structure
  - Improved maintainability

### 📊 Statistics

- **7 commits** since v1.0.1
- **2,292 additions** and **395 deletions** across 21 files
- Major new modules:
  - `src/voices/` - Voice metadata management (AWS Polly voice selection)
  - `src/voices/aws.rs` - AWS voice management
  - `src/voices/download.rs` - Voice download functionality

### 🚀 Migration Notes

If upgrading from v1.0.1:

1. **Configuration Path**: Configuration now uses `~/.config/insight-reader/` instead of `~/.config/grars/`
2. **Installation**: Run the new install script to get the latest version
3. **AWS Polly Users**: You can now browse and select specific AWS Polly voices in the settings

### 📝 Contributors

- gabriel (Gabriel)

