#!/usr/bin/env bash
set -euo pipefail

# Sync MCP Configuration from Central Source
#
# This script imports MCP server configurations from the centralized
# mcp-toolkit source of truth into the rhinolabs-claude plugin.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
MCP_CONFIG_FILE="$PLUGIN_DIR/.mcp.json"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔄 MCP Configuration Sync"
echo "========================="
echo ""

# Configuration source options
CONFIG_SOURCE="${MCP_CONFIG_SOURCE:-local}"

case "$CONFIG_SOURCE" in
  "local")
    # Option A: Read from local mcp-toolkit database
    MCP_TOOLKIT_DIR=""

    if [[ "$OSTYPE" == "darwin"* ]]; then
      MCP_TOOLKIT_DIR="$HOME/Library/Application Support/mcp-toolkit"
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
      MCP_TOOLKIT_DIR="$HOME/.config/mcp-toolkit"
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
      MCP_TOOLKIT_DIR="$APPDATA/mcp-toolkit"
    fi

    if [[ ! -d "$MCP_TOOLKIT_DIR" ]]; then
      echo -e "${RED}✗ Error: mcp-toolkit not found at $MCP_TOOLKIT_DIR${NC}"
      echo ""
      echo "Options:"
      echo "  1. Install mcp-toolkit: https://github.com/chrisllontop/mcp-toolkit"
      echo "  2. Use manual config: Set MCP_CONFIG_SOURCE=manual"
      echo "  3. Use remote config: Set MCP_CONFIG_SOURCE=remote MCP_CONFIG_URL=<url>"
      exit 1
    fi

    echo -e "${YELLOW}⚠ Local mcp-toolkit sync not yet implemented${NC}"
    echo "This requires reading mcp-toolkit's SQLite database."
    echo ""
    echo "For now, use manual export:"
    echo "  1. Open mcp-toolkit UI"
    echo "  2. Export configuration to: $PLUGIN_DIR/mcp-toolkit-export.json"
    echo "  3. Run: MCP_CONFIG_SOURCE=file MCP_CONFIG_FILE=mcp-toolkit-export.json $0"
    exit 1
    ;;

  "file")
    # Option B: Import from exported file
    IMPORT_FILE="${MCP_CONFIG_FILE:-$PLUGIN_DIR/mcp-toolkit-export.json}"

    if [[ ! -f "$IMPORT_FILE" ]]; then
      echo -e "${RED}✗ Error: Import file not found: $IMPORT_FILE${NC}"
      exit 1
    fi

    echo "📁 Importing from: $IMPORT_FILE"

    # Validate JSON
    if ! jq empty "$IMPORT_FILE" 2>/dev/null; then
      echo -e "${RED}✗ Error: Invalid JSON in $IMPORT_FILE${NC}"
      exit 1
    fi

    # Backup existing config
    if [[ -f "$MCP_CONFIG_FILE" ]]; then
      BACKUP_FILE="$MCP_CONFIG_FILE.backup.$(date +%Y%m%d_%H%M%S)"
      cp "$MCP_CONFIG_FILE" "$BACKUP_FILE"
      echo -e "${GREEN}✓ Backed up existing config to: $BACKUP_FILE${NC}"
    fi

    # Transform mcp-toolkit format to Claude Code format
    # (This assumes mcp-toolkit export matches Claude Code format)
    # TODO: Add transformation logic if formats differ
    cp "$IMPORT_FILE" "$MCP_CONFIG_FILE"

    echo -e "${GREEN}✓ Configuration imported successfully${NC}"
    ;;

  "remote")
    # Option C: Fetch from remote URL (S3, internal server, etc.)
    CONFIG_URL="${MCP_CONFIG_URL:-}"

    if [[ -z "$CONFIG_URL" ]]; then
      echo -e "${RED}✗ Error: MCP_CONFIG_URL not set${NC}"
      echo "Example: MCP_CONFIG_URL=https://config.rhinolabs.com/mcp-config.json"
      exit 1
    fi

    echo "🌐 Fetching from: $CONFIG_URL"

    TEMP_FILE=$(mktemp)

    if command -v curl &> /dev/null; then
      curl -fsSL "$CONFIG_URL" -o "$TEMP_FILE"
    elif command -v wget &> /dev/null; then
      wget -qO "$TEMP_FILE" "$CONFIG_URL"
    else
      echo -e "${RED}✗ Error: Neither curl nor wget found${NC}"
      exit 1
    fi

    # Validate JSON
    if ! jq empty "$TEMP_FILE" 2>/dev/null; then
      echo -e "${RED}✗ Error: Invalid JSON from remote${NC}"
      rm "$TEMP_FILE"
      exit 1
    fi

    # Backup existing config
    if [[ -f "$MCP_CONFIG_FILE" ]]; then
      BACKUP_FILE="$MCP_CONFIG_FILE.backup.$(date +%Y%m%d_%H%M%S)"
      cp "$MCP_CONFIG_FILE" "$BACKUP_FILE"
      echo -e "${GREEN}✓ Backed up existing config to: $BACKUP_FILE${NC}"
    fi

    mv "$TEMP_FILE" "$MCP_CONFIG_FILE"
    echo -e "${GREEN}✓ Configuration fetched successfully${NC}"
    ;;

  "manual")
    # Option D: Manual editing (current approach)
    echo "📝 Manual mode - edit $MCP_CONFIG_FILE directly"
    echo ""
    echo "Current configuration:"
    cat "$MCP_CONFIG_FILE" | jq .
    exit 0
    ;;

  *)
    echo -e "${RED}✗ Error: Unknown MCP_CONFIG_SOURCE: $CONFIG_SOURCE${NC}"
    echo ""
    echo "Valid options: local, file, remote, manual"
    exit 1
    ;;
esac

echo ""
echo "📋 Current MCP Configuration:"
echo "=============================="
cat "$MCP_CONFIG_FILE" | jq .

echo ""
echo -e "${GREEN}✓ MCP sync complete${NC}"
echo ""
echo "Next steps:"
echo "  1. Review configuration: cat $MCP_CONFIG_FILE"
echo "  2. Commit changes: git add $MCP_CONFIG_FILE && git commit -m 'chore: sync MCP config'"
echo "  3. Restart Claude Code to apply changes"
