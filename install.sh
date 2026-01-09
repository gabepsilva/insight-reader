#!/bin/bash
# Detect OS and call appropriate installer

OS=$(uname -s)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

case "$OS" in
    Linux)
        exec "$SCRIPT_DIR/install/install-linux.sh" "$@"
        ;;
    Darwin)
        exec "$SCRIPT_DIR/install/install-macos.sh" "$@"
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please use the appropriate installer script directly:"
        echo "  - Linux: install/install-linux.sh"
        echo "  - macOS: install/install-macos.sh"
        exit 1
        ;;
esac

