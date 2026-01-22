# Rhinolabs Claude Plugin Scripts

This directory contains scripts for managing the Rhinolabs Claude plugin.

---

## Available Scripts

### `install.sh`

Installs the rhinolabs-claude plugin to Claude Code's plugin directory.

**Supported OS:**
- macOS 11+
- Ubuntu 20.04+
- Arch Linux
- Windows 10/11 (use `install.ps1`)

**Usage:**
```bash
chmod +x install.sh
./install.sh
```

**What it does:**
1. Detects operating system
2. Copies plugin to appropriate directory
3. Validates installation
4. Prompts to restart Claude Code

---

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

**Option 3: Local mcp-toolkit (future)**
```bash
MCP_CONFIG_SOURCE=local \
./sync-mcp-config.sh
```

**Environment Variables:**

| Variable | Required | Description |
|----------|----------|-------------|
| `MCP_CONFIG_SOURCE` | Yes | Source type: `file`, `remote`, `local`, `manual` |
| `MCP_CONFIG_FILE` | If `file` | Path to exported mcp-toolkit JSON |
| `MCP_CONFIG_URL` | If `remote` | URL to fetch configuration from |

**Features:**
- ✅ Automatic backup of existing config
- ✅ JSON validation
- ✅ Multi-source support (file, remote, local)
- ✅ Clear error messages

**See also:** [MCP_CENTRALIZED_CONFIG.md](../../docs/MCP_CENTRALIZED_CONFIG.md)

---

## Script Development Guidelines

### Adding New Scripts

When adding new scripts to this directory:

1. **Make executable:**
   ```bash
   chmod +x script-name.sh
   ```

2. **Add shebang:**
   ```bash
   #!/usr/bin/env bash
   set -euo pipefail  # Fail fast
   ```

3. **Document:**
   - Add to this README
   - Include usage examples
   - Document environment variables
   - Add error handling

4. **Test on all platforms:**
   - macOS
   - Linux (Ubuntu + Arch)
   - Windows (if applicable)

### Best Practices

- ✅ Use `set -euo pipefail` for safety
- ✅ Quote all variables: `"$VAR"`
- ✅ Check dependencies exist before use
- ✅ Provide clear error messages
- ✅ Support dry-run mode if destructive
- ✅ Use colors for output (optional)
- ✅ Backup before modifying files

---

## Troubleshooting

### Permission Denied

```bash
chmod +x script-name.sh
```

### Script Not Found

Ensure you're in the scripts directory:
```bash
cd rhinolabs-ai/rhinolabs-claude/scripts
```

### Dependencies Missing

Install required tools:
```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq

# Arch
sudo pacman -S jq
```

---

## Support

For script issues:
- Check script output for error messages
- Review logs in Claude Code
- Contact DevOps team (devops@rhinolabs.com)

---

**Last Updated**: 2026-01-23
