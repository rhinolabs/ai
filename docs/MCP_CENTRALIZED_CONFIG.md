# Centralized MCP Configuration Management

This document explains how Rhinolabs manages Model Context Protocol (MCP) servers centrally using mcp-toolkit as the source of truth.

---

## Overview

Instead of each developer maintaining their own `.mcp.json` configuration, Rhinolabs uses a **centralized configuration approach**:

```
┌─────────────────────────┐
│   mcp-toolkit (GUI)     │  ← DevOps manages official configs
│  Source of Truth        │
└───────────┬─────────────┘
            │ export
            ↓
┌─────────────────────────┐
│ rhinolabs-ai/.mcp.json  │  ← Synced configuration
│  (versioned in Git)     │
└───────────┬─────────────┘
            │ git pull
            ↓
┌─────────────────────────┐
│  Developer machines     │  ← Consume configs automatically
│  Claude Code instances  │
└─────────────────────────┘
```

---

## Benefits

| Benefit | Description |
|---------|-------------|
| **Consistency** | All developers use the same MCP servers and versions |
| **Security** | Centralized secret management with AES-256 encryption |
| **Maintenance** | Update once, applies to entire team |
| **Onboarding** | New devs get correct config automatically |
| **Compliance** | Audit which MCP servers are approved for use |
| **Version Control** | Configuration changes tracked in Git |

---

## Architecture

### Components

**1. mcp-toolkit (Source of Truth)**
- Desktop app for visual MCP management
- Stores configs in encrypted local database
- Provides project-level organization
- Manages secrets securely (OS keychain)

**2. rhinolabs-ai Plugin**
- Contains sync script for importing configs
- Maintains `.mcp.json` (generated, not manually edited)
- Distributed via Git to all developers

**3. Developer Machines**
- Git pull to get latest configs
- Restart Claude Code to apply changes
- No manual configuration needed

---

## Setup Guide

### For DevOps/Admin (One-Time Setup)

#### Step 1: Install mcp-toolkit

```bash
# Download from GitHub releases
# https://github.com/chrisllontop/mcp-toolkit/releases

# Or clone and build
git clone https://github.com/chrisllontop/mcp-toolkit.git
cd mcp-toolkit
npm install
npm run tauri build
```

#### Step 2: Configure Official MCP Servers

Open mcp-toolkit and add enterprise MCP servers:

**Example configuration:**

| Server | Purpose | Command |
|--------|---------|---------|
| `git` | Git operations | `npx -y @modelcontextprotocol/server-git` |
| `filesystem` | File operations | `npx -y @modelcontextprotocol/server-filesystem` |
| `playwright` | E2E testing | `npx -y @automatalabs/mcp-server-playwright` |
| `database` | DB queries | `npx -y @modelcontextprotocol/server-postgres` |

#### Step 3: Export Configuration

**Option A: Manual Export (Current)**

1. In mcp-toolkit, select "Export Configuration"
2. Save to: `rhinolabs-ai/rhinolabs-claude/mcp-toolkit-export.json`
3. Run sync script:
   ```bash
   cd rhinolabs-ai/rhinolabs-claude/scripts
   MCP_CONFIG_SOURCE=file MCP_CONFIG_FILE=../mcp-toolkit-export.json ./sync-mcp-config.sh
   ```

**Option B: Remote Sync (Recommended)**

1. Export from mcp-toolkit
2. Upload to internal server (S3, GitHub releases, etc.)
3. Configure sync script with URL:
   ```bash
   export MCP_CONFIG_URL="https://config.rhinolabs.com/mcp-config.json"
   ./sync-mcp-config.sh
   ```

#### Step 4: Commit and Push

```bash
cd rhinolabs-ai
git add rhinolabs-claude/.mcp.json
git commit -m "chore: update MCP configuration from mcp-toolkit"
git push origin main
```

---

### For Developers (Regular Usage)

#### Initial Setup

```bash
# 1. Clone rhinolabs-ai (if not already done)
git clone <rhinolabs-ai-repo>
cd rhinolabs-ai

# 2. Install the plugin
cd rhinolabs-claude/scripts
./install.sh

# 3. Restart Claude Code
# Configuration is automatically loaded
```

#### Updating Configuration

```bash
# 1. Pull latest changes
cd rhinolabs-ai
git pull origin main

# 2. Restart Claude Code
# New MCP servers are automatically available
```

**That's it!** No manual configuration needed.

---

## Sync Script Usage

The sync script supports multiple configuration sources:

### Manual Export from mcp-toolkit

```bash
# 1. Export from mcp-toolkit to mcp-toolkit-export.json
# 2. Run sync
cd rhinolabs-claude/scripts
MCP_CONFIG_SOURCE=file \
MCP_CONFIG_FILE=../mcp-toolkit-export.json \
./sync-mcp-config.sh
```

### Remote URL (Recommended for Production)

```bash
cd rhinolabs-claude/scripts
MCP_CONFIG_SOURCE=remote \
MCP_CONFIG_URL="https://config.rhinolabs.com/mcp-config.json" \
./sync-mcp-config.sh
```

### Local mcp-toolkit Database (Future)

```bash
cd rhinolabs-claude/scripts
MCP_CONFIG_SOURCE=local \
./sync-mcp-config.sh
```

---

## Configuration Format

### mcp-toolkit Export Format

```json
{
  "mcpServers": {
    "git": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-git"],
      "env": {}
    },
    "playwright": {
      "command": "npx",
      "args": ["-y", "@automatalabs/mcp-server-playwright"],
      "env": {
        "PLAYWRIGHT_BROWSERS_PATH": "${HOME}/.cache/ms-playwright"
      }
    }
  },
  "settings": {
    "defaultTimeout": 30000,
    "retryAttempts": 3,
    "logLevel": "info"
  }
}
```

### rhinolabs-ai `.mcp.json` (Generated)

Same format as above. **DO NOT edit manually** - always sync from mcp-toolkit.

---

## Secret Management

### In mcp-toolkit

Secrets are stored securely:
- **macOS**: Keychain
- **Linux**: Secret Service API
- **Windows**: Credential Manager

Encrypted with **AES-256-GCM**.

### In rhinolabs-ai

**DO NOT commit secrets** to `.mcp.json`.

Instead, use environment variable placeholders:

```json
{
  "mcpServers": {
    "database": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "DATABASE_URL": "${RHINOLABS_DB_URL}"
      }
    }
  }
}
```

Developers set environment variables locally:
```bash
# ~/.bashrc or ~/.zshrc
export RHINOLABS_DB_URL="postgresql://..."
```

---

## Approval Workflow

### Adding New MCP Servers

**Process:**

1. **Request**: Developer requests new MCP server via Jira/Slack
2. **Security Review**: DevOps reviews server security
3. **Approval**: Tech Lead approves for enterprise use
4. **Configuration**: DevOps adds to mcp-toolkit
5. **Export**: DevOps runs sync script
6. **Distribution**: Committed to rhinolabs-ai
7. **Notification**: Team notified via Slack

**Review Checklist:**
- [ ] MCP server from trusted source (Anthropic official or vetted)
- [ ] Security audit completed
- [ ] Secrets management reviewed
- [ ] Performance impact assessed
- [ ] Documentation updated

---

## Automation (Future)

### CI/CD Integration

**GitHub Actions workflow** (future enhancement):

```yaml
name: Sync MCP Config

on:
  schedule:
    - cron: '0 */6 * * *'  # Every 6 hours
  workflow_dispatch:

jobs:
  sync-mcp:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Fetch MCP Config
        env:
          MCP_CONFIG_URL: ${{ secrets.MCP_CONFIG_URL }}
        run: |
          cd rhinolabs-claude/scripts
          ./sync-mcp-config.sh

      - name: Commit Changes
        run: |
          git config user.name "MCP Sync Bot"
          git config user.email "devops@rhinolabs.com"
          git add rhinolabs-claude/.mcp.json
          git diff --staged --quiet || git commit -m "chore: sync MCP config from mcp-toolkit"
          git push
```

---

## Troubleshooting

### Config Not Updating

**Symptoms**: Developers don't see new MCP servers

**Solutions**:
1. Verify git pull completed: `git log --oneline -1`
2. Check `.mcp.json` has changes: `git diff HEAD~1 rhinolabs-claude/.mcp.json`
3. Restart Claude Code completely
4. Check Claude Code logs for MCP loading errors

### Sync Script Fails

**Symptoms**: `./sync-mcp-config.sh` errors

**Solutions**:
1. Check MCP_CONFIG_SOURCE is set correctly
2. Verify file exists (for `file` source)
3. Verify URL is accessible (for `remote` source)
4. Check JSON is valid: `jq . mcp-toolkit-export.json`

### mcp-toolkit Not Found

**Symptoms**: "mcp-toolkit not found" error

**Solutions**:
1. Install mcp-toolkit from GitHub releases
2. Or use `file` source with manual export
3. Or use `remote` source with URL

---

## Security Best Practices

### 1. Never Commit Secrets

```json
// ❌ BAD - Hardcoded secret
{
  "env": {
    "API_KEY": "sk-1234567890abcdef"
  }
}

// ✅ GOOD - Environment variable reference
{
  "env": {
    "API_KEY": "${RHINOLABS_API_KEY}"
  }
}
```

### 2. Use Approved Servers Only

- Only MCP servers reviewed by DevOps
- No custom/untrusted servers without approval
- Check `rhinolabs-ai/.mcp.json` for approved list

### 3. Regular Audits

- Quarterly review of enabled MCP servers
- Check for deprecated/vulnerable versions
- Update to latest versions

### 4. Access Control

- Only DevOps has write access to mcp-toolkit
- Developers pull read-only configs
- Changes require PR review

---

## FAQ

**Q: Can I add custom MCP servers locally?**
A: No. Use only approved servers from mcp-toolkit. Request new servers via DevOps.

**Q: What if I need a server urgently?**
A: Contact DevOps for expedited review. Emergency additions possible within 24h.

**Q: How often is config synced?**
A: Currently manual. Future: automated every 6 hours via CI/CD.

**Q: Can I override config locally for testing?**
A: Yes, but changes won't persist. Use a separate test project outside rhinolabs-ai.

**Q: What MCP servers are currently approved?**
A: Check `rhinolabs-claude/.mcp.json` or ask DevOps team.

---

## Resources

- [mcp-toolkit GitHub](https://github.com/chrisllontop/mcp-toolkit)
- [Model Context Protocol Spec](https://modelcontextprotocol.io)
- [MCP Integration Guide](MCP_INTEGRATION.md) - Technical details
- [Official MCP Servers](https://github.com/modelcontextprotocol/servers)

---

## Support

For issues with:
- **mcp-toolkit**: DevOps team (#mcp-support Slack)
- **Sync script**: DevOps team (devops@rhinolabs.com)
- **MCP servers not working**: Check Claude Code logs, then contact DevOps

---

**Last Updated**: 2026-01-23
**Owner**: DevOps Team
**Version**: 1.0.0
