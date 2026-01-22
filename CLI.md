# Rhinolabs CLI

Command-line tool for managing the Rhinolabs Claude plugin.

## Installation

### Download Binary

Download the latest release for your platform:

**macOS (Intel)**:
```bash
curl -L https://github.com/rhinolabs/rhinolabs-ai/releases/latest/download/rhinolabs-darwin-x64 -o rhinolabs
chmod +x rhinolabs
sudo mv rhinolabs /usr/local/bin/
```

**macOS (Apple Silicon)**:
```bash
curl -L https://github.com/rhinolabs/rhinolabs-ai/releases/latest/download/rhinolabs-darwin-arm64 -o rhinolabs
chmod +x rhinolabs
sudo mv rhinolabs /usr/local/bin/
```

**Linux (x64)**:
```bash
curl -L https://github.com/rhinolabs/rhinolabs-ai/releases/latest/download/rhinolabs-linux-x64 -o rhinolabs
chmod +x rhinolabs
sudo mv rhinolabs /usr/local/bin/
```

**Windows**:
Download `rhinolabs-windows-x64.exe` from [releases](https://github.com/rhinolabs/rhinolabs-ai/releases/latest) and add to PATH.

---

## Commands

### Interactive Mode

Run without arguments for interactive menu:

```bash
rhinolabs
```

```
═══════════════════════════════════════════════════
  Rhinolabs AI Plugin Manager
═══════════════════════════════════════════════════

? What would you like to do?
❯ Install plugin
  Update plugin
  Sync MCP configuration
  Check status
  Run diagnostics
  Uninstall plugin
  Exit
```

---

### Install Plugin

Install the Rhinolabs Claude plugin from GitHub releases:

```bash
rhinolabs install
```

**Options**:
- `--local <PATH>`: Install from local directory (for development)
- `--dry-run`: Show what would be done without making changes

**Examples**:
```bash
# Install from GitHub releases
rhinolabs install

# Install from local directory
rhinolabs install --local ./rhinolabs-claude

# Dry run
rhinolabs install --dry-run
```

---

### Update Plugin

Update to the latest version:

```bash
rhinolabs update
```

**Options**:
- `--dry-run`: Show what would be done without making changes

---

### Uninstall Plugin

Remove the plugin:

```bash
rhinolabs uninstall
```

**Options**:
- `--dry-run`: Show what would be done without making changes

---

### Sync MCP Configuration

Sync MCP server configuration from centralized source:

```bash
# From remote URL
rhinolabs sync-mcp --url https://config.rhinolabs.com/mcp.json

# From local file
rhinolabs sync-mcp --file ./mcp-config.json
```

**Options**:
- `--url <URL>`: Remote URL to fetch configuration from
- `--file <PATH>`: Local file to import configuration from
- `--dry-run`: Show what would be done without making changes

---

### Check Status

View plugin installation status and version info:

```bash
rhinolabs status
```

**Output example**:
```
═══════════════════════════════════════════════════
  📊 Rhinolabs AI Plugin Status
═══════════════════════════════════════════════════

Plugin
  Version:      1.0.0
  Installed at: 2026-01-23 15:30:45 UTC
  Location:     ~/.config/claude-code/plugins/rhinolabs-claude
  Status:       ✓ Installed

Claude Code
  Status:       ✓ Detected

MCP Configuration
  Status:       ✓ Configured
  Location:     ~/.config/claude-code/plugins/rhinolabs-claude/.mcp.json
```

---

### Run Diagnostics

Run comprehensive system checks:

```bash
rhinolabs doctor
```

**Output example**:
```
═══════════════════════════════════════════════════
  🔍 Running Diagnostics
═══════════════════════════════════════════════════

✓ Claude Code Installation: Claude Code is installed
✓ Plugin Installation: Plugin v1.0.0 installed
✓ Node.js: Node.js detected
✓ Git: Git is installed
✓ MCP Configuration: MCP config file exists
✓ Updates: Up to date

──────────────────────────────────────────────────
6 passed, 0 failed, 0 warnings

✓ All checks passed!
```

---

### Version

Show CLI version:

```bash
rhinolabs version
```

---

## Building from Source

### Prerequisites

- Rust 1.70+ (`rustup install stable`)
- Git

### Build

```bash
# Clone repository
git clone https://github.com/rhinolabs/rhinolabs-ai.git
cd rhinolabs-ai

# Build CLI
cargo build --release --bin rhinolabs

# Binary will be at: target/release/rhinolabs
```

### Install Locally

```bash
cargo install --path cli
```

---

## Development

### Run Tests

```bash
cargo test --workspace
```

### Run Clippy

```bash
cargo clippy --workspace
```

### Format Code

```bash
cargo fmt --all
```

---

## Troubleshooting

### "Claude Code not found"

**Solution**: Install Claude Code from https://code.claude.com

### "Permission denied"

**Solution** (macOS/Linux):
```bash
chmod +x rhinolabs
```

### "Plugin already installed"

**Solution**: Use `rhinolabs update` to update or `rhinolabs uninstall` first.

---

## Support

- **Documentation**: https://github.com/rhinolabs/rhinolabs-ai
- **Issues**: https://github.com/rhinolabs/rhinolabs-ai/issues
- **Internal**: Contact DevOps team

---

**Version**: 1.0.0
