# Installation Guide

## Supported AI Assistants

| Target | Instructions File | Config Directory |
|--------|------------------|------------------|
| Claude Code | `CLAUDE.md` | `~/.claude/` |
| Amp | `AGENTS.md` | `~/.config/agents/` |
| Antigravity (Gemini) | `GEMINI.md` | `~/.gemini/antigravity/` |
| OpenCode | `opencode.json` | `~/.config/opencode/` |

## Prerequisites

- At least one supported AI coding assistant installed
- Terminal/PowerShell access
- Internet connection

## Quick Start

### 1. Install the CLI

**Download from releases (recommended):**

```bash
# Linux x64
curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-linux-x64
chmod +x rhinolabs-ai
sudo mv rhinolabs-ai /usr/local/bin/
```

| Platform | Binary |
|----------|--------|
| macOS (Apple Silicon) | `rhinolabs-ai-darwin-arm64` |
| macOS (Intel) | `rhinolabs-ai-darwin-x64` |
| Linux (x64) | `rhinolabs-ai-linux-x64` |
| Linux (ARM64) | `rhinolabs-ai-linux-arm64` |
| Windows | `rhinolabs-ai-windows-x64.exe` |

**Or build from source** (requires [Rust](https://rustup.rs/)):

```bash
cd cli && cargo build --release
sudo cp target/release/rhinolabs-ai /usr/local/bin/
```

A short alias `rlai` is also available in releases.

### 2. Install plugin + skills (one command)

```bash
rhinolabs-ai install
```

This does everything:
1. Downloads the plugin from GitHub releases
2. Extracts it to the Claude Code plugins directory
3. Creates the main profile with all available skills
4. Installs skills to `~/.claude/skills/`

### 3. Restart Claude Code

That's it.

## Advanced Usage

```bash
# Install for specific targets
rhinolabs-ai install --target claude-code
rhinolabs-ai install --target amp
rhinolabs-ai install --target all

# Install plugin only, no skills
rhinolabs-ai install --skip-profile

# Install from local source (development)
rhinolabs-ai install --local ./rhinolabs-claude

# Dry run
rhinolabs-ai install --dry-run
```

### Windows (PowerShell)

For Windows users without the CLI, a PowerShell installer is available:

```powershell
cd rhinolabs-claude\scripts
.\install.ps1
```

## Managing Skills

```bash
# List profiles
rhinolabs-ai profile list

# Show profile details
rhinolabs-ai profile show main

# Re-install skills (update to latest)
rhinolabs-ai profile install main --target all

# Update installed skills
rhinolabs-ai profile update

# Uninstall
rhinolabs-ai profile uninstall --target all
```

## Installation Paths

### Plugin Directory

- **Linux**: `~/.config/claude-code/plugins/rhinolabs-claude/`
- **macOS**: `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude/`
- **Windows**: `%APPDATA%\Claude Code\plugins\rhinolabs-claude\`

### Skills Directories

Skills are installed to `<config-dir>/skills/` per target:

| Target | Skills Path |
|--------|-------------|
| Claude Code | `~/.claude/skills/` |
| Amp | `~/.config/agents/skills/` |
| Antigravity | `~/.gemini/antigravity/skills/` |
| OpenCode | `~/.config/opencode/skills/` |

## Troubleshooting

### Plugin not appearing in Claude Code

- Restart Claude Code completely
- Verify `~/.config/claude-code/plugins/rhinolabs-claude/.claude-plugin/plugin.json` exists
- Check Claude Code logs

### Skills not loading

- Run `rhinolabs-ai profile install main`
- Verify skills in `~/.claude/skills/`
- Restart Claude Code

### Updating

```bash
rhinolabs-ai update        # Update plugin to latest release
rhinolabs-ai profile update # Update skills to latest versions
```

### Uninstalling

```bash
rhinolabs-ai uninstall  # Remove plugin
rhinolabs-ai profile uninstall --target all  # Remove skills
```

## Support

For issues: check [the issues page](https://github.com/javiermontescarrera/rhinolabs-ai/issues)

---

**Last Updated**: 2026-02-07
