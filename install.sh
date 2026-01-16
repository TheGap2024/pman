#!/bin/bash
set -e

echo "Building pman..."
cargo build --release

INSTALL_PATH="${HOME}/.local/bin/pman"
mkdir -p "$(dirname "$INSTALL_PATH")"

echo "Installing pman to $INSTALL_PATH..."
cp target/release/pman "$INSTALL_PATH"
chmod +x "$INSTALL_PATH"

echo ""
echo "✓ pman installed successfully!"
echo ""
echo "Make sure ~/.local/bin is in your PATH:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "Then install tmux keybindings:"
echo "  pman install"
echo ""
echo "And reload your tmux config:"
echo "  tmux source-file ~/.tmux.conf"
