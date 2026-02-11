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

Download the binary for your platform from [GitHub Releases](https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest):

| Platform | Binary | Alias |
|----------|--------|-------|
| macOS (Apple Silicon) | `rhinolabs-ai-darwin-arm64` | `rlai-darwin-arm64` |
| macOS (Intel) | `rhinolabs-ai-darwin-x64` | `rlai-darwin-x64` |
| Linux (x64) | `rhinolabs-ai-linux-x64` | `rlai-linux-x64` |
| Linux (ARM64) | `rhinolabs-ai-linux-arm64` | `rlai-linux-arm64` |
| Windows (x64) | `rhinolabs-ai-windows-x64.exe` | `rlai-windows-x64.exe` |

#### Linux

```bash
# x64
curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-linux-x64
chmod +x rhinolabs-ai
sudo mv rhinolabs-ai /usr/local/bin/

# ARM64
curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-linux-arm64
chmod +x rhinolabs-ai
sudo mv rhinolabs-ai /usr/local/bin/
```

#### macOS

```bash
# Apple Silicon (M1/M2/M3/M4)
curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-darwin-arm64
chmod +x rhinolabs-ai
sudo mv rhinolabs-ai /usr/local/bin/

# Intel
curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-darwin-x64
chmod +x rhinolabs-ai
sudo mv rhinolabs-ai /usr/local/bin/
```

> **Note**: If macOS blocks the binary with a Gatekeeper warning, run:
> ```bash
> xattr -d com.apple.quarantine /usr/local/bin/rhinolabs-ai
> ```

#### Windows

**Option A — PowerShell (recommended):**

```powershell
# Download the binary
Invoke-WebRequest -Uri "https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-windows-x64.exe" -OutFile "$env:LOCALAPPDATA\rhinolabs-ai.exe"

# Add to PATH (current user, persistent)
$pathEntries = [Environment]::GetEnvironmentVariable("Path", "User") -split ";"
if ($pathEntries -notcontains $env:LOCALAPPDATA) {
    [Environment]::SetEnvironmentVariable("Path", ($pathEntries + $env:LOCALAPPDATA) -join ";", "User")
}
```

Restart your terminal after adding to PATH.

**Option B — curl:**

```powershell
# Use curl.exe (not the PowerShell alias) to follow redirects correctly
curl.exe -L -o "$env:LOCALAPPDATA\rhinolabs-ai.exe" https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-windows-x64.exe

# Add to PATH (current user, persistent)
$pathEntries = [Environment]::GetEnvironmentVariable("Path", "User") -split ";"
if ($pathEntries -notcontains $env:LOCALAPPDATA) {
    [Environment]::SetEnvironmentVariable("Path", ($pathEntries + $env:LOCALAPPDATA) -join ";", "User")
}
```

Restart your terminal after running these commands.

> **Note**: In PowerShell, `curl` is an alias for `Invoke-WebRequest`. Always use `curl.exe` to invoke the real curl.

> **Tip**: The short alias `rlai` is also available for all platforms. Download the corresponding `rlai-*` binary and install it the same way.

#### Build from source (all platforms)

Requires [Rust](https://rustup.rs/):

```bash
cd cli && cargo build --release
# Linux/macOS
sudo cp target/release/rhinolabs-ai /usr/local/bin/
# Windows (PowerShell)
# Copy-Item target\release\rhinolabs-ai.exe $env:LOCALAPPDATA\
```

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

### Windows (legacy PowerShell installer)

For Windows users who prefer not to install the CLI, a standalone PowerShell installer is available:

```powershell
cd rhinolabs-claude\scripts
.\install.ps1
```

> This installs the plugin and skills without the CLI. For ongoing management (profiles, updates, uninstall), the CLI is recommended.

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

### Windows: CLI exits silently with no output

If `rhinolabs-ai --version` produces no output, you may be running a pre-v0.1.1 binary that requires the Visual C++ Redistributable. Either:

- **Upgrade**: Download the latest binary (v0.1.1+), which includes static CRT linking
- **Or install the runtime**: Download [Visual C++ Redistributable x64](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist) and restart your terminal

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

**Last Updated**: 2026-02-11
