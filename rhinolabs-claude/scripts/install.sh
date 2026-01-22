#!/bin/bash

# Rhinolabs Claude Plugin Installer
# Supports: Ubuntu, Arch Linux, macOS

set -e

echo "🚀 Rhinolabs Claude Plugin Installer"
echo "===================================="
echo ""

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ -f /etc/arch-release ]; then
        OS="arch"
        echo "✓ Detected: Arch Linux"
    elif [ -f /etc/lsb-release ] || [ -f /etc/debian_version ]; then
        OS="ubuntu"
        echo "✓ Detected: Ubuntu/Debian"
    else
        echo "❌ Unsupported Linux distribution"
        exit 1
    fi
    PLUGIN_DIR="$HOME/.config/claude-code/plugins"
    CLAUDE_CONFIG_DIR="$HOME/.claude"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    echo "✓ Detected: macOS"
    PLUGIN_DIR="$HOME/Library/Application Support/Claude Code/plugins"
    CLAUDE_CONFIG_DIR="$HOME/.claude"
else
    echo "❌ Unsupported operating system: $OSTYPE"
    exit 1
fi

echo ""

# Check if Claude Code is installed
if [ "$OS" == "macos" ]; then
    if [ ! -d "$HOME/Library/Application Support/Claude Code" ]; then
        echo "⚠️  Warning: Claude Code installation not found"
        echo "   Please install Claude Code first"
        read -p "   Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
else
    if [ ! -d "$HOME/.config/claude-code" ]; then
        echo "⚠️  Warning: Claude Code installation not found"
        echo "   Please install Claude Code first"
        read -p "   Continue anyway? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            exit 1
        fi
    fi
fi

# Check if plugin already exists
if [ -d "$PLUGIN_DIR/rhinolabs-claude" ]; then
    echo "⚠️  Existing plugin found at: $PLUGIN_DIR/rhinolabs-claude"
    read -p "   Overwrite? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Installation cancelled"
        exit 1
    fi
    echo "🗑️  Removing existing plugin..."
    rm -rf "$PLUGIN_DIR/rhinolabs-claude"
fi

# Create directories
echo "📁 Creating directories..."
mkdir -p "$PLUGIN_DIR"
mkdir -p "$CLAUDE_CONFIG_DIR/output-styles"

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PLUGIN_SOURCE="$( cd "$SCRIPT_DIR/.." && pwd )"

# Copy plugin files
echo "📦 Installing plugin files..."
cp -r "$PLUGIN_SOURCE" "$PLUGIN_DIR/rhinolabs-claude"

# Install global configurations
echo "⚙️  Installing global configurations..."

# Copy output style
if [ -f "$PLUGIN_SOURCE/output-styles/rhinolabs.md" ]; then
    cp "$PLUGIN_SOURCE/output-styles/rhinolabs.md" "$CLAUDE_CONFIG_DIR/output-styles/"
    echo "   ✓ Output style installed"
fi

# Copy statusline script
if [ -f "$PLUGIN_SOURCE/statusline.sh" ]; then
    cp "$PLUGIN_SOURCE/statusline.sh" "$CLAUDE_CONFIG_DIR/"
    chmod +x "$CLAUDE_CONFIG_DIR/statusline.sh"
    echo "   ✓ Status line script installed"
fi

# Handle settings.json (ask before overwriting)
if [ -f "$PLUGIN_SOURCE/settings.json" ]; then
    if [ -f "$CLAUDE_CONFIG_DIR/settings.json" ]; then
        echo ""
        echo "⚠️  Existing settings.json found at: $CLAUDE_CONFIG_DIR/settings.json"
        read -p "   Overwrite with Rhinolabs settings? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            cp "$PLUGIN_SOURCE/settings.json" "$CLAUDE_CONFIG_DIR/"
            echo "   ✓ Settings installed"
        else
            echo "   ⏭️  Skipped settings.json (keeping existing)"
        fi
    else
        cp "$PLUGIN_SOURCE/settings.json" "$CLAUDE_CONFIG_DIR/"
        echo "   ✓ Settings installed"
    fi
fi

# Verify installation
if [ -f "$PLUGIN_DIR/rhinolabs-claude/.claude-plugin/plugin.json" ]; then
    echo ""
    echo "✅ Installation successful!"
    echo ""
    echo "Installed components:"
    echo "  • Plugin: $PLUGIN_DIR/rhinolabs-claude"
    echo "  • Output style: $CLAUDE_CONFIG_DIR/output-styles/rhinolabs.md"
    echo "  • Status line: $CLAUDE_CONFIG_DIR/statusline.sh"
    echo "  • Settings: $CLAUDE_CONFIG_DIR/settings.json"
    echo ""
    echo "Next steps:"
    echo "1. Restart Claude Code"
    echo "2. The plugin will be automatically loaded"
    echo "3. Check Claude Code settings to verify plugin is active"
    echo ""
else
    echo ""
    echo "❌ Installation failed!"
    echo "   .claude-plugin/plugin.json not found in target directory"
    exit 1
fi
