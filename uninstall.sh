#!/bin/bash
# Detect OS and call appropriate uninstaller

OS=$(uname -s)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

case "$OS" in
    Linux)
        exec "$SCRIPT_DIR/install/uninstall-linux.sh" "$@"
        ;;
    Darwin)
        exec "$SCRIPT_DIR/install/uninstall-macos.sh" "$@"
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please use the appropriate uninstaller script directly:"
        echo "  - Linux: install/uninstall-linux.sh"
        echo "  - macOS: install/uninstall-macos.sh"
        exit 1
        ;;
esac

