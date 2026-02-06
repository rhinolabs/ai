# Rhinolabs Claude Plugin Scripts

This directory contains utility scripts for managing the Rhinolabs Claude plugin.

> **Note:** Plugin installation is handled by the `rhinolabs-ai` CLI.
> Run `rhinolabs-ai install` to install the plugin and all skills in one step.

---

## Available Scripts

### `install.ps1`

Windows PowerShell installer for the rhinolabs-claude plugin.

**Usage:**
```powershell
.\install.ps1
```

**Requirements:**
- Windows 10/11
- PowerShell 5.1 or later
- Claude Code installed

---

### `sync-mcp-config.sh`

Syncs MCP server configurations from centralized source (mcp-toolkit) to the plugin.

**Purpose:** Maintain single source of truth for MCP configurations across the organization.

**Usage:**

**Option 1: Manual export from mcp-toolkit**
```bash
# 1. Export from mcp-toolkit GUI to mcp-toolkit-export.json
# 2. Run sync
MCP_CONFIG_SOURCE=file \
MCP_CONFIG_FILE=../mcp-toolkit-export.json \
./sync-mcp-config.sh
```

**Option 2: Remote URL (recommended)**
```bash
MCP_CONFIG_SOURCE=remote \
MCP_CONFIG_URL="https://config.rhinolabs.com/mcp-config.json" \
./sync-mcp-config.sh
```

**Environment Variables:**

| Variable | Required | Description |
|----------|----------|-------------|
| `MCP_CONFIG_SOURCE` | Yes | Source type: `file`, `remote`, `local`, `manual` |
| `MCP_CONFIG_FILE` | If `file` | Path to exported mcp-toolkit JSON |
| `MCP_CONFIG_URL` | If `remote` | URL to fetch configuration from |

**See also:** [MCP_CENTRALIZED_CONFIG.md](../../docs/MCP_CENTRALIZED_CONFIG.md)

---

## Support

For script issues:
- Check script output for error messages
- Review logs in Claude Code
- Contact DevOps team (devops@rhinolabs.com)

---

**Last Updated**: 2026-02-07
