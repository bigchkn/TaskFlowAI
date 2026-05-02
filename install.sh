#!/bin/bash
set -e

echo "Building and installing taskflow-ai..."

# Ensure we are in the project root
cd "$(dirname "$0")"

# Build and install
cargo install --path . --force

# Check if ~/.cargo/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "Warning: ~/.cargo/bin is not in your PATH."
    echo "Please add 'export PATH=\$HOME/.cargo/bin:\$PATH' to your shell profile (e.g., .zshrc or .bashrc)."
fi

echo "Success! taskflow-ai $(taskflow-ai --version) is installed."
