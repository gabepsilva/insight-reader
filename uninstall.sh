#!/bin/bash
# Detect OS and call appropriate uninstaller
# This script downloads required uninstall scripts from GitHub if they don't exist locally

set -euo pipefail

OS=$(uname -s)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GITHUB_REPO="${GITHUB_REPO:-gabepsilva/grars}"

# Use cache directory for downloaded scripts (or local install directory if in repo)
if [ -d "$SCRIPT_DIR/install" ] && [ -f "$SCRIPT_DIR/install/common-bash.sh" ]; then
    # We're in the repository, use local scripts
    INSTALL_DIR="$SCRIPT_DIR/install"
else
    # Download to cache directory
    INSTALL_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/grars-install"
    mkdir -p "$INSTALL_DIR"
fi

# Function to download file from GitHub
download_file() {
    local url="$1"
    local output="$2"
    
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "$output" "$url"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "$output" "$url"
    else
        return 1
    fi
}

# Ensure install directory exists (already created above if using cache)

# Download required scripts from GitHub if they don't exist locally
GITHUB_BASE="https://raw.githubusercontent.com/$GITHUB_REPO/master/install"

# Check and download common-bash.sh
if [ ! -f "$INSTALL_DIR/common-bash.sh" ]; then
    echo "Downloading common-bash.sh from GitHub..."
    if ! download_file "$GITHUB_BASE/common-bash.sh" "$INSTALL_DIR/common-bash.sh"; then
        echo "Error: Failed to download common-bash.sh from GitHub"
        exit 1
    fi
    chmod +x "$INSTALL_DIR/common-bash.sh"
fi

# Check and download platform-specific uninstall script
case "$OS" in
    Linux)
        if [ ! -f "$INSTALL_DIR/uninstall-linux.sh" ]; then
            echo "Downloading uninstall-linux.sh from GitHub..."
            if ! download_file "$GITHUB_BASE/uninstall-linux.sh" "$INSTALL_DIR/uninstall-linux.sh"; then
                echo "Error: Failed to download uninstall-linux.sh from GitHub"
                exit 1
            fi
            chmod +x "$INSTALL_DIR/uninstall-linux.sh"
        fi
        exec "$INSTALL_DIR/uninstall-linux.sh" "$@"
        ;;
    Darwin)
        if [ ! -f "$INSTALL_DIR/uninstall-macos.sh" ]; then
            echo "Downloading uninstall-macos.sh from GitHub..."
            if ! download_file "$GITHUB_BASE/uninstall-macos.sh" "$INSTALL_DIR/uninstall-macos.sh"; then
                echo "Error: Failed to download uninstall-macos.sh from GitHub"
                exit 1
            fi
            chmod +x "$INSTALL_DIR/uninstall-macos.sh"
        fi
        exec "$INSTALL_DIR/uninstall-macos.sh" "$@"
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please use the appropriate uninstaller script directly:"
        echo "  - Linux: install/uninstall-linux.sh"
        echo "  - macOS: install/uninstall-macos.sh"
        exit 1
        ;;
esac

