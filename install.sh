#!/bin/bash
set -e

INSTALL_SKILLS=false

# Simple argument parsing
for arg in "$@"; do
    if [ "$arg" == "--skill" ]; then
        INSTALL_SKILLS=true
    fi
done

echo "Building and installing taskflow-ai..."

# Ensure we are in the project root
cd "$(dirname "$0")"

# Build and install
cargo install --path . --force

# Define the binary path (since it might not be in PATH yet)
TASKFLOW_BIN="$HOME/.cargo/bin/taskflow-ai"

# Check if ~/.cargo/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo "Warning: ~/.cargo/bin is not in your PATH."
    echo "Please add 'export PATH=\$HOME/.cargo/bin:\$PATH' to your shell profile (e.g., .zshrc or .bashrc)."
fi

echo "Success! taskflow-ai $($TASKFLOW_BIN --version) is installed."

if [ "$INSTALL_SKILLS" = true ]; then
    echo "Checking for AI provider locations to install skills..."
    
    # List of providers and their base directories (compatible with Bash 3.2)
    # Format: "provider_name:base_directory"
    provider_configs=(
        "claude:$HOME/.claude"
        "gemini:$HOME/.gemini"
        "codex:$HOME/.codex"
        "dirac:$HOME/.dirac"
        "opencode:$HOME/.config/opencode"
        "agents:$HOME/.agents"
    )

    for config in "${provider_configs[@]}"; do
        provider="${config%%:*}"
        base_dir="${config#*:}"
        
        if [ -d "$base_dir" ]; then
            echo "Detected $provider environment at $base_dir. Installing skill..."
            $TASKFLOW_BIN skill install "$provider"
        fi
    done
fi
