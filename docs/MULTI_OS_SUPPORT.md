# Multi-OS Support Guide

This document explains the multi-operating system support in the Rhinolabs Claude plugin.

## Supported Operating Systems

The plugin supports the following operating systems:

| OS | Versions | Package Manager | Status |
|----|----------|----------------|--------|
| Ubuntu | 20.04+ | APT | ✅ Fully Supported |
| Arch Linux | Latest | Pacman | ✅ Fully Supported |
| macOS | 11+ (Big Sur+) | Homebrew | ✅ Fully Supported |
| Windows | 10/11 | N/A | ✅ Fully Supported |

## Installation Paths

Each operating system uses a different installation path for Claude Code plugins:

### Linux (Ubuntu/Arch)
```
~/.config/claude-code/plugins/rhinolabs-claude/
```

**Breakdown**:
- `~/.config/` - User configuration directory (XDG Base Directory)
- `claude-code/` - Claude Code configuration
- `plugins/` - Plugin directory
- `rhinolabs-claude/` - This plugin

### macOS
```
~/Library/Application Support/Claude Code/plugins/rhinolabs-claude/
```

**Breakdown**:
- `~/Library/Application Support/` - macOS application data directory
- `Claude Code/` - Claude Code application data
- `plugins/` - Plugin directory
- `rhinolabs-claude/` - This plugin

### Windows
```
%APPDATA%\Claude Code\plugins\rhinolabs-claude\
```

**Expanded**:
```
C:\Users\<username>\AppData\Roaming\Claude Code\plugins\rhinolabs-claude\
```

**Breakdown**:
- `%APPDATA%` - Windows application data directory
- `Claude Code\` - Claude Code application data
- `plugins\` - Plugin directory
- `rhinolabs-claude\` - This plugin

## OS Detection

The installation scripts automatically detect the operating system.

## Platform-Specific Considerations

### Ubuntu

**Package Manager**: APT

**Characteristics**:
- Debian-based distribution
- Uses systemd for services
- Standard XDG directories
- LTS versions recommended

**Installation Notes**:
- No sudo required for plugin installation (user-level)
- Ensure Claude Code is installed via official method
- Check `~/.config/claude-code/` exists

### Arch Linux

**Package Manager**: Pacman

**Characteristics**:
- Rolling release distribution
- Uses systemd for services
- Follows XDG Base Directory specification
- Cutting-edge packages

**Installation Notes**:
- No sudo required for plugin installation (user-level)
- May need to install Claude Code from AUR
- Check `~/.config/claude-code/` exists

### macOS

**Package Manager**: Homebrew

**Characteristics**:
- Unix-based operating system
- Uses launchd for services
- Application Support directory for app data
- Case-insensitive filesystem (default)

**Installation Notes**:
- No admin privileges required (user-level)
- Ensure Claude Code is installed in Applications
- Path may contain spaces (handle with quotes)
- Use `~/Library/Application Support/` (with space)

### Windows

**Characteristics**:
- Different path separators (backslash)
- Case-insensitive filesystem
- Uses %APPDATA% for application data
- PowerShell for scripting

**Installation Notes**:
- No admin privileges required (user-level)
- Use PowerShell 5.1+ (built into Windows 10/11)
- Path uses backslashes
- May need to adjust execution policy

## File Permissions

### Linux/macOS

**Plugin Files**:
- Directories: `755` (rwxr-xr-x)
- Files: `644` (rw-r--r--)
- Scripts: `755` (rwxr-xr-x)

### Windows

Windows uses ACLs (Access Control Lists) instead of Unix permissions. The installer handles this automatically.

## Line Endings

Different operating systems use different line endings:

| OS | Line Ending | Representation |
|----|-------------|----------------|
| Linux | LF | `\n` |
| macOS | LF | `\n` |
| Windows | CRLF | `\r\n` |

## Testing Across Platforms

### Local Testing

Test the plugin on each platform:

1. **Ubuntu**: Use VM or Docker
2. **Arch Linux**: Use VM or Docker
3. **macOS**: Use physical Mac or VM
4. **Windows**: Use physical PC or VM

## Troubleshooting

### Path Not Found (All Platforms)

**Issue**: Claude Code directory doesn't exist

**Solution**:
- Verify Claude Code is installed
- Check installation path
- Create directory manually if needed

### Permission Denied (Linux/macOS)

**Issue**: Cannot write to plugin directory

**Solution**: Check and fix permissions on the plugin directory

### Execution Policy (Windows)

**Issue**: Cannot run PowerShell script

**Solution**: Adjust PowerShell execution policy for current user

## Best Practices

### Cross-Platform Development

1. **Use relative paths**: Avoid hardcoded absolute paths
2. **Test on all platforms**: Don't assume behavior
3. **Handle spaces**: Quote paths that may contain spaces
4. **Use appropriate separators**: `/` for Unix, `\` for Windows
5. **Check line endings**: Use `.gitattributes` to control

### Git Configuration

Create `.gitattributes` in repository root:
```
# Set default behavior to automatically normalize line endings
* text=auto

# Force bash scripts to use LF
*.sh text eol=lf

# Force PowerShell scripts to use CRLF
*.ps1 text eol=crlf

# Denote binary files
*.png binary
*.jpg binary
```

---

**Last Updated**: 2026-01-22  
**Version**: 1.0.0
