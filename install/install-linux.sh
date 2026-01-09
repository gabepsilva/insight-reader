#!/bin/bash

set -euo pipefail

# Source common functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common-bash.sh"

log_info "Installing to: $INSTALL_DIR"
log_info "Binary will be installed to: $BIN_DIR"

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        # Source os-release, temporarily disable unbound variable check
        set +u
        . /etc/os-release
        DISTRO_ID="${ID:-unknown}"
        DISTRO_ID_LIKE="${ID_LIKE:-}"
        set -u
    elif [ -f /etc/arch-release ]; then
        DISTRO_ID="arch"
    elif [ -f /etc/debian_version ]; then
        DISTRO_ID="debian"
    elif [ -f /etc/redhat-release ]; then
        DISTRO_ID="rhel"
    else
        DISTRO_ID="unknown"
    fi
    log_info "Detected distribution: $DISTRO_ID"
}

# Check all required dependencies and install if missing
check_and_install_dependencies() {
    local missing_deps=()
    local packages_to_install=()
    
    log_info "Checking required dependencies..."
    
    # Check espeak-ng
    if ! command_exists espeak-ng; then
        missing_deps+=("espeak-ng")
        log_warn "espeak-ng not found (required)"
    else
        log_success "espeak-ng found"
    fi
    
    # Check clipboard utilities (need at least one for Wayland or X11)
    local clipboard_available=false
    if command_exists wl-paste; then
        log_success "wl-paste found (Wayland clipboard support)"
        clipboard_available=true
    else
        log_warn "wl-paste not found (Wayland clipboard support missing)"
    fi
    
    if command_exists xclip; then
        log_success "xclip found (X11 clipboard support)"
        clipboard_available=true
    else
        log_warn "xclip not found (X11 clipboard support missing)"
    fi
    
    if [ "$clipboard_available" = false ]; then
        missing_deps+=("clipboard-utils")
        log_warn "No clipboard utility found (need wl-clipboard for Wayland or xclip for X11)"
    fi
    
    # Check Python3
    local python_missing=false
    local venv_missing=false
    if ! command_exists python3; then
        missing_deps+=("python3")
        python_missing=true
        log_warn "python3 not found (required)"
    else
        PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
        log_info "Python3 found: $PYTHON_VERSION"
        
        # Check venv module - try to actually use it, not just check help
        if ! python3 -m venv --help >/dev/null 2>&1; then
            missing_deps+=("python3-venv")
            venv_missing=true
            log_warn "python3-venv module not found (required)"
        else
            # Test if venv can actually create a venv (requires ensurepip)
            local test_venv_dir
            test_venv_dir=$(mktemp -d)
            if python3 -m venv "$test_venv_dir" >/dev/null 2>&1; then
                rm -rf "$test_venv_dir"
                log_success "Python3 venv module is available"
            else
                missing_deps+=("python3-venv")
                venv_missing=true
                log_warn "python3-venv module cannot create virtual environments (required)"
                rm -rf "$test_venv_dir" 2>/dev/null || true
            fi
        fi
    fi
    
    # If all dependencies are present, return
    if [ ${#missing_deps[@]} -eq 0 ]; then
        log_success "All required dependencies are installed"
        return 0
    fi
    
    # Show missing dependencies and ask user
    echo ""
    log_warn "Missing required dependencies:"
    for dep in "${missing_deps[@]}"; do
        echo "  - $dep"
    done
    echo ""
    read -p "Install missing dependencies? [Y/n] " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        log_error "Cannot continue without required dependencies"
        exit 1
    fi
    
    # Determine packages to install based on distribution
    case "$DISTRO_ID" in
        arch|manjaro|endeavouros)
            if command_exists pacman; then
                [ "$python_missing" = true ] && packages_to_install+=("python" "python-pip")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via pacman: ${packages_to_install[*]}"
                    sudo pacman -S --needed --noconfirm "${packages_to_install[@]}"
                fi
            else
                log_error "pacman not found. Please install dependencies manually."
                exit 1
            fi
            ;;
        debian|ubuntu|linuxmint|pop)
            if command_exists apt-get; then
                [ "$python_missing" = true ] && packages_to_install+=("python3" "python3-venv" "python3-pip")
                [ "$venv_missing" = true ] && packages_to_install+=("python3-venv")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                # Remove duplicates
                IFS=" " read -r -a packages_to_install <<< "$(printf '%s\n' "${packages_to_install[@]}" | sort -u | tr '\n' ' ')"
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via apt: ${packages_to_install[*]}"
                    sudo apt-get update
                    sudo apt-get install -y "${packages_to_install[@]}"
                fi
            else
                log_error "apt-get not found. Please install dependencies manually."
                exit 1
            fi
            ;;
        fedora|rhel|centos|rocky|almalinux)
            if command_exists dnf; then
                [ "$python_missing" = true ] && packages_to_install+=("python3" "python3-pip")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via dnf: ${packages_to_install[*]}"
                    sudo dnf install -y "${packages_to_install[@]}"
                fi
            elif command_exists yum; then
                [ "$python_missing" = true ] && packages_to_install+=("python3" "python3-pip")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via yum: ${packages_to_install[*]}"
                    sudo yum install -y "${packages_to_install[@]}"
                fi
            else
                log_error "dnf/yum not found. Please install dependencies manually."
                exit 1
            fi
            ;;
        opensuse*|sles)
            if command_exists zypper; then
                [ "$python_missing" = true ] && packages_to_install+=("python3" "python3-pip")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via zypper: ${packages_to_install[*]}"
                    sudo zypper install -y "${packages_to_install[@]}"
                fi
            else
                log_error "zypper not found. Please install dependencies manually."
                exit 1
            fi
            ;;
        alpine)
            if command_exists apk; then
                [ "$python_missing" = true ] && packages_to_install+=("python3" "py3-pip")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via apk: ${packages_to_install[*]}"
                    sudo apk add --no-cache "${packages_to_install[@]}"
                fi
            else
                log_error "apk not found. Please install dependencies manually."
                exit 1
            fi
            ;;
        void)
            if command_exists xbps-install; then
                [ "$python_missing" = true ] && packages_to_install+=("python3")
                [[ " ${missing_deps[@]} " =~ " espeak-ng " ]] && packages_to_install+=("espeak-ng")
                [[ " ${missing_deps[@]} " =~ " clipboard-utils " ]] && packages_to_install+=("wl-clipboard" "xclip")
                if [ ${#packages_to_install[@]} -gt 0 ]; then
                    log_info "Installing packages via xbps-install: ${packages_to_install[*]}"
                    sudo xbps-install -S -y "${packages_to_install[@]}"
                fi
            else
                log_error "xbps-install not found. Please install dependencies manually."
                exit 1
            fi
            ;;
        *)
            log_error "Unsupported distribution: $DISTRO_ID"
            log_warn "Please install required dependencies manually, then run this script again."
            exit 1
            ;;
    esac
    
    # Verify installations
    if ! command_exists python3; then
        log_error "Python3 installation failed or not found in PATH"
        exit 1
    fi
    
    # Verify venv can actually create a venv
    local test_venv_dir
    test_venv_dir=$(mktemp -d)
    if ! python3 -m venv "$test_venv_dir" >/dev/null 2>&1; then
        rm -rf "$test_venv_dir" 2>/dev/null || true
        log_error "Python3 venv module cannot create virtual environments"
        log_error "On Debian/Ubuntu, you may need: sudo apt install python3-venv"
        exit 1
    fi
    rm -rf "$test_venv_dir"
    log_success "Python3 venv module verified"
    
    if ! command_exists espeak-ng; then
        log_warn "espeak-ng installation may have failed. Piper may not work correctly."
    fi
    
    # Verify clipboard utilities
    local clipboard_ok=false
    if command_exists wl-paste; then
        log_success "wl-paste verified (Wayland clipboard support)"
        clipboard_ok=true
    fi
    if command_exists xclip; then
        log_success "xclip verified (X11 clipboard support)"
        clipboard_ok=true
    fi
    if [ "$clipboard_ok" = false ]; then
        log_warn "Clipboard utilities installation may have failed. App may not be able to read selected text."
    fi
    
    log_success "Dependencies installed successfully"
}

# Install desktop file and icon
install_desktop() {
    DESKTOP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"
    DESKTOP_FILE="$DESKTOP_DIR/grars.desktop"
    ICON_FILE="$ICON_DIR/grars.svg"
    
    # Get script directory (or current directory if script is not available)
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || pwd)"
    
    # Create directories
    mkdir -p "$DESKTOP_DIR"
    mkdir -p "$ICON_DIR"
    
    # GitHub URLs for desktop file and icon
    DESKTOP_URL="https://raw.githubusercontent.com/$GITHUB_REPO/master/grars.desktop"
    ICON_URL="https://raw.githubusercontent.com/$GITHUB_REPO/master/assets/logo.svg"
    
    # Try to find desktop file locally first
    local desktop_source=""
    local temp_desktop=""
    if [ -f "$SCRIPT_DIR/grars.desktop" ]; then
        desktop_source="$SCRIPT_DIR/grars.desktop"
        log_info "Found desktop file locally at $desktop_source"
    else
        # Download from GitHub
        log_info "Desktop file not found locally. Downloading from GitHub..."
        temp_desktop=$(mktemp)
        if download_file "$DESKTOP_URL" "$temp_desktop"; then
            desktop_source="$temp_desktop"
            log_success "Desktop file downloaded from GitHub"
        else
            rm -f "$temp_desktop" 2>/dev/null || true
            log_warn "Failed to download desktop file from GitHub (skipping)"
            desktop_source=""
        fi
    fi
    
    # Install desktop file if we have a source
    if [ -n "$desktop_source" ] && [ -f "$desktop_source" ]; then
        # Process desktop file: replace $HOME with actual home directory
        sed "s#\\\$HOME#$HOME#g" "$desktop_source" > "$DESKTOP_FILE"
        chmod 644 "$DESKTOP_FILE"
        log_success "Desktop file installed to $DESKTOP_FILE"
        
        # Clean up temporary file if it was downloaded
        [ "$desktop_source" = "$temp_desktop" ] && rm -f "$temp_desktop" 2>/dev/null || true
        
        # Update desktop database (try multiple methods for different DEs)
        # KDE uses kbuildsycoca (Plasma 5/6)
        if command -v kbuildsycoca6 >/dev/null 2>&1; then
            kbuildsycoca6 --noincremental >/dev/null 2>&1 || true
            log_info "KDE 6 application database updated"
        elif command -v kbuildsycoca5 >/dev/null 2>&1; then
            kbuildsycoca5 --noincremental >/dev/null 2>&1 || true
            log_info "KDE 5 application database updated"
        fi
        # Generic freedesktop.org tool (works on GNOME, XFCE, etc.)
        if command -v update-desktop-database >/dev/null 2>&1; then
            update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
            log_info "Desktop database updated"
        fi
    fi
    
    # Try to find icon locally first
    local icon_source=""
    local temp_icon=""
    if [ -f "$SCRIPT_DIR/assets/logo.svg" ]; then
        icon_source="$SCRIPT_DIR/assets/logo.svg"
        log_info "Found icon locally at $icon_source"
    else
        # Download from GitHub
        log_info "Icon not found locally. Downloading from GitHub..."
        temp_icon=$(mktemp)
        if download_file "$ICON_URL" "$temp_icon"; then
            icon_source="$temp_icon"
            log_success "Icon downloaded from GitHub"
        else
            rm -f "$temp_icon" 2>/dev/null || true
            log_warn "Failed to download icon from GitHub (skipping)"
            icon_source=""
        fi
    fi
    
    # Install icon if we have a source
    if [ -n "$icon_source" ] && [ -f "$icon_source" ]; then
        cp "$icon_source" "$ICON_FILE"
        log_success "Icon installed to $ICON_FILE"
        
        # Clean up temporary file if it was downloaded
        [ "$icon_source" = "$temp_icon" ] && rm -f "$temp_icon" 2>/dev/null || true
        
        # Update icon cache (try multiple methods for different DEs)
        # GTK-based DEs (GNOME, XFCE, etc.)
        if command -v gtk-update-icon-cache >/dev/null 2>&1; then
            gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
            log_info "GTK icon cache updated"
        fi
        # KDE uses kbuildsycoca for icons too
        if command -v kbuildsycoca6 >/dev/null 2>&1; then
            kbuildsycoca6 --noincremental >/dev/null 2>&1 || true
        elif command -v kbuildsycoca5 >/dev/null 2>&1; then
            kbuildsycoca5 --noincremental >/dev/null 2>&1 || true
        fi
    fi
}

# Main installation function
main() {
    echo "=========================================="
    echo "  grars Installation Script"
    echo "=========================================="
    echo ""
    
    detect_distro
    check_and_install_dependencies
    install_binary
    create_venv
    install_piper
    
    # Download model if not present (download_model checks if it exists first)
    echo ""
    download_model
    
    # Install desktop file and icon
    echo ""
    install_desktop
    
    echo ""
    log_success "Installation complete!"
    echo ""
    echo "grars binary: $GRARS_BIN"
    echo "Piper venv: $VENV_DIR/bin/piper"
    echo "Models directory: $MODELS_DIR"
    echo ""
    if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
        log_warn "$HOME/.local/bin is not in your PATH"
        echo "Add this to your shell configuration (~/.bashrc, ~/.zshrc, etc.):"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
    fi
    echo "Run grars with: grars"
    echo ""
}

# Run main function
main "$@"
