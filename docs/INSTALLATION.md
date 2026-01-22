# Installation Guide

This guide covers the installation process for the Rhinolabs Claude plugin across all supported platforms.

## Prerequisites

Before installing the plugin, ensure you have:

- Claude Code installed on your system
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
   git clone https://github.com/rhinolabs/rhinolabs-ai.git
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
   ```bash
   ./install.sh
   ```

5. **Follow the prompts**
   - The installer will detect your OS automatically
   - Confirm if you want to overwrite an existing installation
   - Wait for the installation to complete

6. **Restart Claude Code**
   - Close Claude Code completely
   - Reopen Claude Code
   - The plugin will be automatically loaded

### Windows

1. **Clone the repository**
   ```powershell
   git clone https://github.com/rhinolabs/rhinolabs-ai.git
   cd rhinolabs-ai
   ```

2. **Navigate to scripts directory**
   ```powershell
   cd rhinolabs-claude\scripts
   ```

3. **Run the installer**
   ```powershell
   .\install.ps1
   ```

4. **Follow the prompts**
   - Confirm if you want to overwrite an existing installation
   - Wait for the installation to complete

5. **Restart Claude Code**
   - Close Claude Code completely
   - Reopen Claude Code
   - The plugin will be automatically loaded

## Installation Paths

The plugin is installed at the following user-level locations:

- **Ubuntu/Arch Linux**: `~/.config/claude-code/plugins/rhinolabs-claude`
- **macOS**: `~/Library/Application Support/Claude Code/plugins/rhinolabs-claude`
- **Windows**: `%APPDATA%\Claude Code\plugins\rhinolabs-claude`

## Verification

To verify the installation:

1. Open Claude Code
2. Go to Settings → Plugins
3. Look for "rhinolabs-claude" in the list
4. Ensure it's enabled

## Troubleshooting

### Plugin not appearing

**Issue**: Plugin doesn't appear in Claude Code after installation.

**Solutions**:
- Ensure Claude Code was completely restarted
- Check that the plugin files are in the correct directory
- Verify `plugin.json` exists in the plugin directory
- Check Claude Code logs for errors

### Permission errors (Linux/macOS)

**Issue**: Permission denied when running the installer.

**Solutions**:
```bash
chmod +x install.sh
./install.sh
```

### PowerShell execution policy (Windows)

**Issue**: Script execution is disabled.

**Solutions**:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\install.ps1
```

### Existing plugin conflicts

**Issue**: Another plugin with the same name exists.

**Solutions**:
- The installer will prompt you to overwrite
- Manually remove the old plugin before installing
- Backup any custom configurations

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
   ./install.sh  # or install.ps1 on Windows
   ```

3. **Restart Claude Code**

## Uninstallation

To remove the plugin:

### Linux/macOS
```bash
rm -rf ~/.config/claude-code/plugins/rhinolabs-claude  # Ubuntu/Arch
rm -rf ~/Library/Application\ Support/Claude\ Code/plugins/rhinolabs-claude  # macOS
```

### Windows
```powershell
Remove-Item -Path "$env:APPDATA\Claude Code\plugins\rhinolabs-claude" -Recurse -Force
```

Then restart Claude Code.

## Support

For installation issues:
- Check the troubleshooting section above
- Review Claude Code logs
- Contact the DevOps team (internal)

---

**Last Updated**: 2026-01-22  
**Version**: 1.0.0
