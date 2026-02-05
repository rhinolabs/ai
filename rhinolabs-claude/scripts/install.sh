#!/bin/bash

# Rhinolabs AI Plugin Installer
# Supports: Ubuntu, Arch Linux, macOS
# Targets: Claude Code, Amp, Antigravity (Gemini), OpenCode

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 Rhinolabs AI Plugin Installer${NC}"
echo "===================================="
echo ""

# Available targets
AVAILABLE_TARGETS=("claude-code" "amp" "antigravity" "opencode")

# Parse arguments
SELECTED_TARGETS=()
SHOW_HELP=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--target)
            if [[ "$2" == "all" ]]; then
                SELECTED_TARGETS=("${AVAILABLE_TARGETS[@]}")
            else
                SELECTED_TARGETS+=("$2")
            fi
            shift 2
            ;;
        -h|--help)
            SHOW_HELP=true
            shift
            ;;
        *)
            echo -e "${RED}❌ Unknown option: $1${NC}"
            SHOW_HELP=true
            shift
            ;;
    esac
done

if $SHOW_HELP; then
    echo "Usage: ./install.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -t, --target TARGET   Install for specific target (can be used multiple times)"
    echo "                        Available: claude-code, amp, antigravity, opencode, all"
    echo "  -h, --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./install.sh                           # Interactive mode"
    echo "  ./install.sh -t claude-code            # Install for Claude Code only"
    echo "  ./install.sh -t claude-code -t amp     # Install for Claude Code and Amp"
    echo "  ./install.sh -t all                    # Install for all targets"
    exit 0
fi

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if [ -f /etc/arch-release ]; then
        OS="arch"
        echo -e "${GREEN}✓${NC} Detected: Arch Linux"
    elif [ -f /etc/lsb-release ] || [ -f /etc/debian_version ]; then
        OS="ubuntu"
        echo -e "${GREEN}✓${NC} Detected: Ubuntu/Debian"
    else
        echo -e "${RED}❌ Unsupported Linux distribution${NC}"
        exit 1
    fi
    CONFIG_DIR="$HOME/.config"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    echo -e "${GREEN}✓${NC} Detected: macOS"
    CONFIG_DIR="$HOME/.config"
else
    echo -e "${RED}❌ Unsupported operating system: $OSTYPE${NC}"
    exit 1
fi

echo ""

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PLUGIN_SOURCE="$( cd "$SCRIPT_DIR/.." && pwd )"

# Function to get config dir for target
get_config_dir() {
    local target=$1
    case $target in
        claude-code)
            echo "$HOME/.claude"
            ;;
        amp)
            echo "$CONFIG_DIR/agents"
            ;;
        antigravity)
            echo "$HOME/.gemini/antigravity"
            ;;
        opencode)
            echo "$CONFIG_DIR/opencode"
            ;;
    esac
}

# Function to get skills dir for target
get_skills_dir() {
    local target=$1
    echo "$(get_config_dir "$target")/skills"
}

# Function to get instructions filename for target
get_instructions_filename() {
    local target=$1
    case $target in
        claude-code)
            echo "CLAUDE.md"
            ;;
        amp)
            echo "AGENTS.md"
            ;;
        antigravity)
            echo "GEMINI.md"
            ;;
        opencode)
            echo "opencode.json"
            ;;
    esac
}

# Function to get MCP config filename for target
get_mcp_filename() {
    local target=$1
    case $target in
        claude-code)
            echo ".mcp.json"
            ;;
        amp)
            echo "settings.json"
            ;;
        antigravity)
            echo "config.json"
            ;;
        opencode)
            echo "opencode.json"
            ;;
    esac
}

# Function to get display name for target
get_display_name() {
    local target=$1
    case $target in
        claude-code)
            echo "Claude Code"
            ;;
        amp)
            echo "Amp"
            ;;
        antigravity)
            echo "Antigravity (Gemini)"
            ;;
        opencode)
            echo "OpenCode"
            ;;
    esac
}

# Function to check if target is installed
is_target_installed() {
    local target=$1
    local config_dir=$(get_config_dir "$target")
    [ -d "$config_dir" ]
}

# Interactive target selection if none specified
if [ ${#SELECTED_TARGETS[@]} -eq 0 ]; then
    echo "Select targets to install:"
    echo ""

    for i in "${!AVAILABLE_TARGETS[@]}"; do
        target="${AVAILABLE_TARGETS[$i]}"
        display_name=$(get_display_name "$target")
        config_dir=$(get_config_dir "$target")

        if is_target_installed "$target"; then
            status="${GREEN}[installed]${NC}"
        else
            status="${YELLOW}[not found]${NC}"
        fi

        echo -e "  $((i+1)). $display_name $status"
        echo -e "      Config: $config_dir"
    done

    echo "  5. All targets"
    echo ""

    read -p "Enter numbers separated by space (e.g., '1 2' or '5' for all): " -r
    echo ""

    if [[ "$REPLY" == *"5"* ]]; then
        SELECTED_TARGETS=("${AVAILABLE_TARGETS[@]}")
    else
        for num in $REPLY; do
            if [[ $num -ge 1 && $num -le 4 ]]; then
                SELECTED_TARGETS+=("${AVAILABLE_TARGETS[$((num-1))]}")
            fi
        done
    fi
fi

if [ ${#SELECTED_TARGETS[@]} -eq 0 ]; then
    echo -e "${RED}❌ No targets selected${NC}"
    exit 1
fi

echo "Selected targets:"
for target in "${SELECTED_TARGETS[@]}"; do
    echo -e "  ${CYAN}•${NC} $(get_display_name "$target")"
done
echo ""

# Confirm installation
read -p "Continue with installation? (Y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Nn]$ ]]; then
    echo -e "${YELLOW}❌ Installation cancelled${NC}"
    exit 1
fi
echo ""

# Install for each target
for target in "${SELECTED_TARGETS[@]}"; do
    display_name=$(get_display_name "$target")
    config_dir=$(get_config_dir "$target")
    skills_dir=$(get_skills_dir "$target")

    echo -e "${CYAN}📦 Installing for $display_name...${NC}"

    # Create directories
    mkdir -p "$config_dir"
    mkdir -p "$skills_dir"
    mkdir -p "$config_dir/output-styles"

    # Copy skills
    if [ -d "$PLUGIN_SOURCE/skills" ]; then
        cp -r "$PLUGIN_SOURCE/skills/"* "$skills_dir/" 2>/dev/null || true
        echo -e "   ${GREEN}✓${NC} Skills installed to $skills_dir"
    fi

    # Copy output style
    if [ -f "$PLUGIN_SOURCE/output-styles/rhinolabs.md" ]; then
        cp "$PLUGIN_SOURCE/output-styles/rhinolabs.md" "$config_dir/output-styles/"
        echo -e "   ${GREEN}✓${NC} Output style installed"
    fi

    # Copy MCP config (only if it doesn't exist)
    mcp_filename=$(get_mcp_filename "$target")
    if [ -f "$PLUGIN_SOURCE/.mcp.json" ] && [ ! -f "$config_dir/$mcp_filename" ]; then
        cp "$PLUGIN_SOURCE/.mcp.json" "$config_dir/$mcp_filename"
        echo -e "   ${GREEN}✓${NC} MCP config installed"
    elif [ -f "$config_dir/$mcp_filename" ]; then
        echo -e "   ${YELLOW}⏭️${NC}  MCP config exists, skipped"
    fi

    # Target-specific installations
    case $target in
        claude-code)
            # Copy Claude Code plugin
            PLUGIN_DIR="$HOME/.config/claude-code/plugins"
            if [ "$OS" == "macos" ]; then
                PLUGIN_DIR="$HOME/Library/Application Support/Claude Code/plugins"
            fi

            if [ -d "$PLUGIN_DIR/rhinolabs-claude" ]; then
                echo -e "   ${YELLOW}⚠️${NC}  Existing plugin found, overwriting..."
                rm -rf "$PLUGIN_DIR/rhinolabs-claude"
            fi

            mkdir -p "$PLUGIN_DIR"
            cp -r "$PLUGIN_SOURCE" "$PLUGIN_DIR/rhinolabs-claude"
            echo -e "   ${GREEN}✓${NC} Plugin installed to $PLUGIN_DIR/rhinolabs-claude"

            # Copy statusline script
            if [ -f "$PLUGIN_SOURCE/statusline.sh" ]; then
                cp "$PLUGIN_SOURCE/statusline.sh" "$config_dir/"
                chmod +x "$config_dir/statusline.sh"
                echo -e "   ${GREEN}✓${NC} Status line script installed"
            fi

            # Handle settings.json (merge instead of overwrite)
            if [ -f "$PLUGIN_SOURCE/settings.json" ]; then
                if [ -f "$config_dir/settings.json" ]; then
                    # Check if jq is available for merging
                    if command -v jq &> /dev/null; then
                        echo -e "   ${CYAN}→${NC} Merging settings.json..."
                        # Deep merge: existing settings take precedence for scalar values,
                        # arrays are concatenated and deduplicated
                        jq -s '
                            def deep_merge:
                                if type == "array" then
                                    .[0] as $a | .[1] as $b |
                                    if ($a | type) == "array" and ($b | type) == "array" then
                                        ($a + $b) | unique
                                    else
                                        $b // $a
                                    end
                                elif type == "object" then
                                    .[0] as $a | .[1] as $b |
                                    ($a | keys) + ($b | keys) | unique | map(
                                        . as $key |
                                        if ($a[$key] | type) == "object" and ($b[$key] | type) == "object" then
                                            {($key): ([$a[$key], $b[$key]] | deep_merge)}
                                        elif ($a[$key] | type) == "array" and ($b[$key] | type) == "array" then
                                            {($key): ([$a[$key], $b[$key]] | deep_merge)}
                                        else
                                            {($key): ($a[$key] // $b[$key])}
                                        end
                                    ) | add
                                else
                                    .[0] // .[1]
                                end;
                            deep_merge
                        ' "$config_dir/settings.json" "$PLUGIN_SOURCE/settings.json" > "$config_dir/settings.json.tmp"
                        mv "$config_dir/settings.json.tmp" "$config_dir/settings.json"
                        echo -e "   ${GREEN}✓${NC} Settings merged (your settings preserved)"
                    else
                        echo -e "   ${YELLOW}⚠️${NC}  jq not found, cannot merge settings"
                        read -p "      Overwrite existing settings? (y/N): " -n 1 -r
                        echo
                        if [[ $REPLY =~ ^[Yy]$ ]]; then
                            cp "$PLUGIN_SOURCE/settings.json" "$config_dir/"
                            echo -e "   ${GREEN}✓${NC} Settings installed"
                        else
                            echo -e "   ${YELLOW}⏭️${NC}  Settings skipped"
                        fi
                    fi
                else
                    cp "$PLUGIN_SOURCE/settings.json" "$config_dir/"
                    echo -e "   ${GREEN}✓${NC} Settings installed"
                fi
            fi
            ;;
    esac

    echo ""
done

# Summary
echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "Installed for:"
for target in "${SELECTED_TARGETS[@]}"; do
    display_name=$(get_display_name "$target")
    config_dir=$(get_config_dir "$target")
    echo -e "  ${GREEN}•${NC} $display_name → $config_dir"
done
echo ""
echo "Next steps:"
echo "  1. Restart your AI coding assistant(s)"
echo "  2. The plugin/skills will be automatically loaded"
echo ""
