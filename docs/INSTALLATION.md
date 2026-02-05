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

## Supported Platforms

| Platform | Version | Status |
|----------|---------|--------|
| Ubuntu | 20.04+ | ✅ Supported |
| Arch Linux | Latest | ✅ Supported |
| macOS | 11+ | ✅ Supported |
| Windows | 10/11 | ✅ Supported |

## Installation Steps

### Linux (Ubuntu/Arch) & macOS

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>/rhinolabs-ai.git
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
   ```

5. **Restart your AI assistant(s)**

### Windows

1. **Clone the repository**
   ```powershell
   git clone <your-repo-url>/rhinolabs-ai.git
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
   ```

4. **Restart your AI assistant(s)**

### PowerShell Execution Policy

If you get a script execution error on Windows:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\install.ps1
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

Skills are installed to `<config-dir>/skills/` for each target.

### Claude Code Plugin Directory

The Claude Code plugin is additionally installed to:
- **Linux**: `~/.config/claude-code/plugins/rhinolabs-claude`
- **macOS**: `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude`
- **Windows**: `%APPDATA%\Claude Code\plugins\rhinolabs-claude`

## What Gets Installed

For **all targets**:
- Skills → `<config-dir>/skills/`
- Output styles → `<config-dir>/output-styles/`
- MCP config → `<config-dir>/<mcp-filename>`

For **Claude Code** specifically:
- Plugin files → `<plugins-dir>/rhinolabs-claude/`
- Settings → `~/.claude/settings.json`
- Status line script → `~/.claude/statusline.sh`

## Verification

### Claude Code

1. Open Claude Code
2. Go to Settings → Plugins
3. Look for "rhinolabs-claude" in the list
4. Ensure it's enabled
5. Verify `.claude-plugin/plugin.json` exists in the plugin directory

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
- Verify the config directory exists for your target
- Check that skills were copied to `<config-dir>/skills/`
- Restart your AI assistant
- Check file permissions

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
- Backup your existing configs before installing
- Use the interactive mode to see what will be installed

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

3. **Restart your AI assistant(s)**

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

**Last Updated**: 2026-02-05
