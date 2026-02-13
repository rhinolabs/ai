# Installation Guide — Windows (WSL)

Guide for installing and validating `rlai` on Windows using WSL (Windows Subsystem for Linux).

> **Important**: In WSL you run the **Linux binary**, not the Windows `.exe`.
> Claude Code runs as a Linux process inside WSL, so all paths are Linux paths.

---

## Prerequisites

### 1. WSL 2

Open PowerShell as Administrator:

```powershell
wsl --install
```

If WSL is already installed, verify you're on WSL 2:

```powershell
wsl --list --verbose
```

The `VERSION` column should show `2`. If it shows `1`:

```powershell
wsl --set-version <distro-name> 2
```

### 2. Node.js (inside WSL)

Claude Code requires Node.js >= 18. From within your WSL terminal:

```bash
# Option A — nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
source ~/.bashrc
nvm install --lts

# Option B — system package (Ubuntu/Debian)
sudo apt update && sudo apt install -y nodejs npm
```

Verify:

```bash
node --version   # Should be >= 18
npm --version
```

### 3. Claude Code (inside WSL)

```bash
npm install -g @anthropic-ai/claude-code
```

Verify:

```bash
which claude     # Should return a path like /home/<user>/.nvm/versions/node/.../bin/claude
claude --version
```

### 4. Initialize and authenticate Claude Code

Before installing `rlai`, you **must** run Claude Code at least once to set up
your account:

```bash
claude
```

Claude Code will guide you through an interactive authentication flow:

1. It will display a URL and a code
2. Open that URL in your browser (you can copy it from the WSL terminal)
3. Log in with your Anthropic account (or create one)
4. Enter the code shown in the terminal to link the session
5. Once authenticated, Claude Code will confirm the connection

After authentication completes, you can type `/exit` to close Claude Code.

Verify that the user scope was created:

```bash
ls ~/.claude/
# Should show configuration files (settings.json, etc.)
```

> **Why this step is required**: `rlai install` deploys skills and configuration
> into `~/.claude/`. If Claude Code has never been initialized, this directory
> won't exist and the skills won't be functional even if installed.

---

## Install rlai

### Option A — Download binary (recommended)

```bash
# x64 (most common for WSL)
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-linux-x64
chmod +x rlai
sudo mv rlai /usr/local/bin/
```

For ARM64 (e.g., Windows on ARM with WSL):

```bash
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-linux-arm64
chmod +x rlai
sudo mv rlai /usr/local/bin/
```

### Option B — Build from source

Requires Rust:

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/javiermontescarrera/rhinolabs-ai.git
cd rhinolabs-ai/cli
cargo build --release
sudo cp ../target/release/rlai /usr/local/bin/
```

### Verify installation

```bash
rlai --version
rlai doctor
```

`rlai doctor` should detect Claude Code as installed.

---

## Install Plugin + Skills

```bash
rlai install
```

This will:
1. Download the plugin from GitHub releases
2. Extract to `~/.config/claude-code/plugins/rhinolabs-claude/`
3. Create the main profile with all skills
4. Install skills to `~/.claude/skills/`

### Verify

```bash
rlai status
```

Expected output should show:
- Plugin: installed
- Profile: Main-Profile
- Skills: list of installed skills

---

## Path Reference (WSL)

All paths are Linux paths inside WSL:

| Component | Path |
|-----------|------|
| rlai binary | `/usr/local/bin/rlai` |
| Plugin directory | `~/.config/claude-code/plugins/rhinolabs-claude/` |
| Skills (Claude Code) | `~/.claude/skills/` |
| Skills (Amp) | `~/.config/agents/skills/` |
| Skills (Antigravity) | `~/.gemini/antigravity/skills/` |
| Skills (OpenCode) | `~/.config/opencode/skills/` |
| Rhinolabs config | `~/.config/rhinolabs-ai/` |
| Profiles config | `~/.config/rhinolabs-ai/profiles.json` |
| MCP config | `~/.config/claude-code/plugins/rhinolabs-claude/.mcp.json` |

---

## Validation Checklist

Run each step in order after installation. All must pass.

### 1. Binary

```bash
rlai --version
# Expected: rhinolabs-ai x.y.z
```

### 2. Doctor

```bash
rlai doctor
# Expected: Claude Code detected, no errors
```

### 3. Plugin installed

```bash
rlai status
# Expected: Plugin installed, version shown
```

### 4. Skills installed

```bash
ls ~/.claude/skills/
# Expected: directories for each skill (react-patterns, typescript-best-practices, etc.)
```

### 5. Profile exists

```bash
rlai profile list
# Expected: Main-Profile listed with skills
```

### 6. Claude Code sees the skills

```bash
claude
# Inside Claude, ask: "What skills do you have available?"
# Expected: Claude should reference rhinolabs skills
```

---

## WSL-Specific Considerations

### File system performance

WSL 2 has slow I/O when accessing Windows files (`/mnt/c/...`). Keep your projects
inside the WSL filesystem (`~/projects/`) for best performance.

### Windows vs WSL environments

`rlai` installed in WSL is **independent** from any Windows-native installation.
They do not share config files:

| | WSL (Linux) | Windows native |
|-|-------------|----------------|
| Binary | `rlai-linux-x64` | `rlai-windows-x64.exe` |
| Home | `/home/<user>/` | `C:\Users\<user>\` |
| Config | `~/.config/` | `%APPDATA%` |
| Claude Code | Linux binary via npm | Windows desktop app |

If you use Claude Code only from WSL, you only need the WSL installation.
If you also use the Windows desktop app, you would need a separate Windows installation.

### Accessing WSL files from Windows

Windows can browse WSL files at `\\wsl$\<distro>\home\<user>\`. This is useful
for inspecting installed skills or configs from Windows Explorer.

### Symlinks

WSL 2 supports Linux symlinks natively. The skill deployment system uses symlinks
by default, which work correctly in WSL without any special configuration.

---

## Troubleshooting

### `rlai: command not found`

```bash
# Verify binary location
ls -la /usr/local/bin/rlai

# If missing, re-download
curl -L -o rlai https://github.com/javiermontescarrera/rhinolabs-ai/releases/latest/download/rlai-linux-x64
chmod +x rlai
sudo mv rlai /usr/local/bin/
```

### `claude: command not found`

Claude Code is not installed in WSL. Install it:

```bash
npm install -g @anthropic-ai/claude-code
```

If using nvm, make sure the correct Node version is active:

```bash
nvm use --lts
```

### `rlai doctor` says Claude Code not detected

The doctor command checks `which claude`. Make sure Claude Code's binary is in your PATH:

```bash
which claude
# If empty, check npm global bin:
npm list -g @anthropic-ai/claude-code
npm bin -g   # This is the directory that should be in PATH
```

### Permission denied on `/usr/local/bin/`

Use `sudo` when moving the binary, or install to a user-owned directory:

```bash
mkdir -p ~/.local/bin
mv rlai ~/.local/bin/

# Add to PATH if not already there (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

### Plugin download fails (network/proxy)

If behind a corporate proxy:

```bash
export HTTPS_PROXY=http://proxy:port
rlai install
```

### Skills not appearing in Claude Code

1. Verify skills are installed:
   ```bash
   ls -la ~/.claude/skills/
   ```
2. Restart Claude Code completely:
   ```bash
   claude  # Start a new session
   ```
3. Check for symlink issues:
   ```bash
   file ~/.claude/skills/react-patterns
   # Should show: symbolic link to ...
   ```

---

**Last Updated**: 2026-02-13
