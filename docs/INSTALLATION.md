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

| Platform | Binary | Full name |
|----------|--------|-----------|
| macOS (Apple Silicon) | `rlai-darwin-arm64` | `rhinolabs-ai-darwin-arm64` |
| macOS (Intel) | `rlai-darwin-x64` | `rhinolabs-ai-darwin-x64` |
| Linux (x64) | `rlai-linux-x64` | `rhinolabs-ai-linux-x64` |
| Linux (ARM64) | `rlai-linux-arm64` | `rhinolabs-ai-linux-arm64` |
| Windows (x64) | `rlai-windows-x64.exe` | `rhinolabs-ai-windows-x64.exe` |

#### Linux

```bash
# x64
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-linux-x64
chmod +x rlai
sudo mv rlai /usr/local/bin/

# ARM64
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-linux-arm64
chmod +x rlai
sudo mv rlai /usr/local/bin/
```

#### macOS

```bash
# Apple Silicon (M1/M2/M3/M4)
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-darwin-arm64
chmod +x rlai
sudo mv rlai /usr/local/bin/

# Intel
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-darwin-x64
chmod +x rlai
sudo mv rlai /usr/local/bin/
```

> **Note**: If macOS blocks the binary with a Gatekeeper warning, run:
> ```bash
> xattr -d com.apple.quarantine /usr/local/bin/rlai
> ```

#### Windows

**Option A — PowerShell (recommended):**

```powershell
# Download the binary
Invoke-WebRequest -Uri "https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-windows-x64.exe" -OutFile "$env:LOCALAPPDATA\rlai.exe"

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
curl.exe -L -o "$env:LOCALAPPDATA\rlai.exe" https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-windows-x64.exe

# Add to PATH (current user, persistent)
$pathEntries = [Environment]::GetEnvironmentVariable("Path", "User") -split ";"
if ($pathEntries -notcontains $env:LOCALAPPDATA) {
    [Environment]::SetEnvironmentVariable("Path", ($pathEntries + $env:LOCALAPPDATA) -join ";", "User")
}
```

Restart your terminal after running these commands.

> **Note**: In PowerShell, `curl` is an alias for `Invoke-WebRequest`. Always use `curl.exe` to invoke the real curl.

> **Tip**: The full name `rhinolabs-ai` is also available for all platforms. Install it the same way:
>
> **Linux:**
> ```bash
> # x64
> curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-linux-x64
> chmod +x rhinolabs-ai
> sudo mv rhinolabs-ai /usr/local/bin/
>
> # ARM64
> curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-linux-arm64
> chmod +x rhinolabs-ai
> sudo mv rhinolabs-ai /usr/local/bin/
> ```
>
> **macOS:**
> ```bash
> # Apple Silicon
> curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-darwin-arm64
> chmod +x rhinolabs-ai && sudo mv rhinolabs-ai /usr/local/bin/
> xattr -d com.apple.quarantine /usr/local/bin/rhinolabs-ai
>
> # Intel
> curl -L -o rhinolabs-ai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-darwin-x64
> chmod +x rhinolabs-ai && sudo mv rhinolabs-ai /usr/local/bin/
> xattr -d com.apple.quarantine /usr/local/bin/rhinolabs-ai
> ```
>
> **Windows (PowerShell):**
> ```powershell
> Invoke-WebRequest -Uri "https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rhinolabs-ai-windows-x64.exe" -OutFile "$env:LOCALAPPDATA\rhinolabs-ai.exe"
> ```
> (PATH setup is shared — if you already added `$env:LOCALAPPDATA` to PATH above, `rhinolabs-ai` will work immediately after restarting your terminal.)

#### Build from source (all platforms)

Requires [Rust](https://rustup.rs/):

```bash
cd cli && cargo build --release
# Linux/macOS
sudo cp target/release/rlai /usr/local/bin/
# Windows (PowerShell)
# Copy-Item target\release\rlai.exe $env:LOCALAPPDATA\
```

### 2. Install plugin + skills (one command)

```bash
rlai install
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
rlai install --target claude-code
rlai install --target amp
rlai install --target all

# Install plugin only, no skills
rlai install --skip-profile

# Install from local source (development)
rlai install --local ./rhinolabs-claude

# Dry run
rlai install --dry-run
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
rlai profile list

# Show profile details
rlai profile show main

# Re-install skills (update to latest)
rlai profile install main --target all

# Update installed skills
rlai profile update

# Uninstall
rlai profile uninstall --target all
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

- Run `rlai profile install main`
- Verify skills in `~/.claude/skills/`
- Restart Claude Code

### Windows: CLI exits silently with no output

If `rlai --version` produces no output, you may be running a pre-v0.1.1 binary that requires the Visual C++ Redistributable. Either:

- **Upgrade**: Download the latest binary (v0.1.1+), which includes static CRT linking
- **Or install the runtime**: Download [Visual C++ Redistributable x64](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist) and restart your terminal

### Updating

```bash
rlai update        # Update plugin to latest release
rlai profile update # Update skills to latest versions
```

### Uninstalling

```bash
rlai uninstall  # Remove plugin
rlai profile uninstall --target all  # Remove skills
```

## Support

For issues: check [the issues page](https://github.com/javiermontescarrera/rhinolabs-ai/issues)

---

**Last Updated**: 2026-02-12
