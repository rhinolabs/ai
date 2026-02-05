# Installation Guide

This guide covers the installation process for the Rhinolabs AI plugin across all supported platforms and AI coding assistants.

## Supported AI Assistants

| Target | Instructions File | Config Directory |
|--------|------------------|------------------|
| Claude Code | `CLAUDE.md` | `~/.claude/` |
| Amp | `AGENTS.md` | `~/.config/agents/` |
| Antigravity (Gemini) | `GEMINI.md` | `~/.gemini/antigravity/` |
| OpenCode | `opencode.json` | `~/.config/opencode/` |

## Prerequisites

Before installing, ensure you have:

- At least one supported AI coding assistant installed
- Git installed
- Internet connection for initial setup
- Terminal/PowerShell access
- **rhinolabs-ai CLI** (optional, but required for skill management)

## Supported Platforms

| Platform | Version | Status |
|----------|---------|--------|
| Ubuntu | 20.04+ | Supported |
| Arch Linux | Latest | Supported |
| macOS | 11+ | Supported |
| Windows | 10/11 | Supported |

## Installation Architecture

The installation process has two components:

1. **Plugin Base** (via install script): Output styles, MCP config, settings, plugin structure
2. **Skills** (via CLI): Installed from the "main" profile using `rhinolabs-ai profile install main`

This separation allows:
- Profile-based skill management
- Selective skill installation
- Easy updates without reinstalling everything

## Installation Steps

### Linux (Ubuntu/Arch) & macOS

1. **Clone the repository**
   ```bash
   git clone <repo-url>
   cd rhinolabs-ai
   ```

2. **Navigate to scripts directory**
   ```bash
   cd rhinolabs-claude/scripts
   ```

3. **Make the script executable**
   ```bash
   chmod +x install.sh
   ```

4. **Run the installer**

   **Interactive mode** (recommended for first install):
   ```bash
   ./install.sh
   ```
   The installer will show all available targets and let you select which ones to install.

   **Install for specific target(s)**:
   ```bash
   # Single target
   ./install.sh -t claude-code

   # Multiple targets
   ./install.sh -t claude-code -t amp

   # All targets
   ./install.sh -t all

   # Plugin base only (no skills)
   ./install.sh --skip-skills
   ```

5. **Restart your AI assistant(s)**

### Windows

1. **Clone the repository**
   ```powershell
   git clone <repo-url>
   cd rhinolabs-ai
   ```

2. **Navigate to scripts directory**
   ```powershell
   cd rhinolabs-claude\scripts
   ```

3. **Run the installer**

   **Interactive mode** (recommended for first install):
   ```powershell
   .\install.ps1
   ```

   **Install for specific target(s)**:
   ```powershell
   # Single target
   .\install.ps1 -Target claude-code

   # Multiple targets
   .\install.ps1 -Target claude-code,amp

   # All targets
   .\install.ps1 -Target all

   # Plugin base only (no skills)
   .\install.ps1 -SkipSkills
   ```

4. **Restart your AI assistant(s)**

### PowerShell Execution Policy

If you get a script execution error on Windows:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\install.ps1
```

## Installing the CLI

The `rhinolabs-ai` CLI is required for skill management. If not installed, the install script will show instructions.

### Option 1: Download from Releases (Recommended)

1. Go to the [Releases page](../../releases/latest)

2. Download the binary for your platform:

   | Platform | Binary |
   |----------|--------|
   | macOS (Apple Silicon) | `rhinolabs-ai-darwin-arm64` |
   | macOS (Intel) | `rhinolabs-ai-darwin-x64` |
   | Linux (x64) | `rhinolabs-ai-linux-x64` |
   | Linux (ARM64) | `rhinolabs-ai-linux-arm64` |
   | Windows | `rhinolabs-ai-windows-x64.exe` |

3. Make executable and move to PATH:

   **macOS/Linux:**
   ```bash
   chmod +x rhinolabs-ai-*
   sudo mv rhinolabs-ai-* /usr/local/bin/rhinolabs-ai
   ```

   **Windows (PowerShell as Admin):**
   ```powershell
   # Move to a directory in your PATH, e.g.:
   Move-Item rhinolabs-ai-windows-x64.exe C:\Windows\rhinolabs-ai.exe
   ```

4. Verify installation:
   ```bash
   rhinolabs-ai --version
   ```

### Option 2: Build from Source

Requires [Rust](https://rustup.rs/) installed.

```bash
cd rhinolabs-ai/cli
cargo build --release

# Binary at: target/release/rhinolabs-ai
# Optionally move to PATH:
sudo cp target/release/rhinolabs-ai /usr/local/bin/
```

### Short Alias

The releases also include `rlai`, a short alias for `rhinolabs-ai`. Download and install it the same way if you prefer shorter commands.

## Managing Skills with the CLI

After installing the CLI, you can manage skills using profiles:

```bash
# Install main profile skills for all targets
rhinolabs-ai profile install main --target all

# Install for specific target
rhinolabs-ai profile install main --target claude-code

# List available profiles
rhinolabs-ai profile list

# Show profile details
rhinolabs-ai profile show main

# Update installed skills
rhinolabs-ai profile update

# Uninstall profile
rhinolabs-ai profile uninstall --target all
```

## Installation Paths

### User-Level Config Directories

| Target | Linux | macOS | Windows |
|--------|-------|-------|---------|
| Claude Code | `~/.claude/` | `~/.claude/` | `%USERPROFILE%\.claude\` |
| Amp | `~/.config/agents/` | `~/.config/agents/` | `%APPDATA%\agents\` |
| Antigravity | `~/.gemini/antigravity/` | `~/.gemini/antigravity/` | `%USERPROFILE%\.gemini\antigravity\` |
| OpenCode | `~/.config/opencode/` | `~/.config/opencode/` | `%APPDATA%\opencode\` |

### Skills Directories

Skills are installed to `<config-dir>/skills/` for each target by the CLI.

### Claude Code Plugin Directory

The Claude Code plugin base is installed to:
- **Linux**: `~/.config/claude-code/plugins/rhinolabs-claude`
- **macOS**: `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude`
- **Windows**: `%APPDATA%\Claude Code\plugins\rhinolabs-claude`

## What Gets Installed

### Plugin Base (via install script)

For **all targets**:
- Output styles `<config-dir>/output-styles/`

**Note**: MCP configuration is NOT deployed by the install script. Configure MCP servers via the GUI (MCP Servers page). See [RAG_MCP_ARCHITECTURE.md](./RAG_MCP_ARCHITECTURE.md) for details.

For **Claude Code** specifically:
- Plugin structure `<plugins-dir>/rhinolabs-claude/`
- Settings `~/.claude/settings.json` (merged with existing)
- Status line script `~/.claude/statusline.sh`

### Skills (via CLI)

Skills from the "main" profile are installed to `<config-dir>/skills/` for each target.

## Verification

### Claude Code

1. Open Claude Code
2. Go to Settings Plugins
3. Look for "rhinolabs-claude" in the list
4. Ensure it's enabled
5. Verify `.claude-plugin/plugin.json` exists in the plugin directory
6. Check that skills are present in `~/.claude/skills/`

### Other Targets

1. Check that the config directory exists
2. Verify skills are present in `<config-dir>/skills/`
3. Open your AI assistant and test that skills are recognized

## Troubleshooting

### Plugin not appearing (Claude Code)

**Issue**: Plugin doesn't appear in Claude Code after installation.

**Solutions**:
- Ensure Claude Code was completely restarted
- Check that the plugin files are in the correct directory
- Verify `.claude-plugin/plugin.json` exists in the plugin directory
- Check Claude Code logs for errors

### Skills not loading

**Issue**: Skills don't appear in your AI assistant.

**Solutions**:
- Ensure `rhinolabs-ai` CLI is installed
- Run `rhinolabs-ai profile install main --target <your-target>`
- Verify skills were installed to `<config-dir>/skills/`
- Restart your AI assistant
- Check file permissions

### CLI not found

**Issue**: The install script reports "rhinolabs-ai CLI not found".

**Solutions**:
- Install the CLI (see "Installing the CLI" section)
- Add the CLI to your PATH
- Run skill installation manually after installing CLI:
  ```bash
  rhinolabs-ai profile install main --target all
  ```

### Permission errors (Linux/macOS)

**Issue**: Permission denied when running the installer.

**Solutions**:
```bash
chmod +x install.sh
./install.sh
```

### Existing config conflicts

**Issue**: You have existing configurations that conflict.

**Solutions**:
- The installer will prompt before overwriting existing files
- Settings are merged (your settings take precedence)
- MCP config is skipped if it already exists
- Backup your existing configs before installing

## Updating

To update the plugin to the latest version:

1. **Pull latest changes**
   ```bash
   cd rhinolabs-ai
   git pull origin main
   ```

2. **Run the installer again**
   ```bash
   cd rhinolabs-claude/scripts
   ./install.sh -t <your-targets>  # or .\install.ps1 on Windows
   ```

3. **Update skills via CLI**
   ```bash
   rhinolabs-ai profile update
   ```

4. **Restart your AI assistant(s)**

## Uninstallation

### Using the CLI (recommended)

```bash
rhinolabs-ai profile uninstall --target all
```

### Manual Removal

#### Linux/macOS

```bash
# Claude Code
rm -rf ~/.claude/skills
rm -rf ~/.config/claude-code/plugins/rhinolabs-claude

# Amp
rm -rf ~/.config/agents/skills

# Antigravity
rm -rf ~/.gemini/antigravity/skills

# OpenCode
rm -rf ~/.config/opencode/skills
```

#### Windows

```powershell
# Claude Code
Remove-Item -Path "$env:USERPROFILE\.claude\skills" -Recurse -Force
Remove-Item -Path "$env:APPDATA\Claude Code\plugins\rhinolabs-claude" -Recurse -Force

# Amp
Remove-Item -Path "$env:APPDATA\agents\skills" -Recurse -Force

# Antigravity
Remove-Item -Path "$env:USERPROFILE\.gemini\antigravity\skills" -Recurse -Force

# OpenCode
Remove-Item -Path "$env:APPDATA\opencode\skills" -Recurse -Force
```

Then restart your AI assistant(s).

## Support

For installation issues:
- Check the troubleshooting section above
- Review your AI assistant's logs
- Contact the DevOps team (internal)

---

**Last Updated**: 2026-02-06
