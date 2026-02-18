#!/usr/bin/env bash
# Rhinolabs AI - Clean Install Script (Linux / macOS)
# Removes ALL rhinolabs-ai artifacts and reinstalls from latest release.
#
# Usage:
#   ./clean-install.sh              # Full clean + reinstall
#   ./clean-install.sh --clean-only # Only remove, don't reinstall

set -euo pipefail

# ── Config ──────────────────────────────────────────────────────────
REPO="rhinolabs/ai"
CLEAN_ONLY=false

if [[ "${1:-}" == "--clean-only" ]]; then
  CLEAN_ONLY=true
fi

# ── OS Detection ────────────────────────────────────────────────────
detect_os() {
  case "$(uname -s)" in
    Linux*)  echo "linux" ;;
    Darwin*) echo "macos" ;;
    *)       echo "unsupported" ;;
  esac
}

detect_arch() {
  case "$(uname -m)" in
    x86_64)  echo "x64" ;;
    aarch64) echo "arm64" ;;
    arm64)   echo "arm64" ;;
    *)       echo "unsupported" ;;
  esac
}

OS=$(detect_os)
ARCH=$(detect_arch)

if [[ "$OS" == "unsupported" || "$ARCH" == "unsupported" ]]; then
  echo "Error: Unsupported platform ($(uname -s) / $(uname -m))"
  exit 1
fi

# ── Path Resolution ─────────────────────────────────────────────────
if [[ "$OS" == "macos" ]]; then
  PLUGIN_DIR="$HOME/Library/Application Support/Claude Code/plugins/rhinolabs-claude"
  CONFIG_DIR="$HOME/Library/Application Support/rhinolabs-ai"
else
  PLUGIN_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/claude-code/plugins/rhinolabs-claude"
  CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/rhinolabs-ai"
fi

SKILLS_DIR="$HOME/.claude/skills"
ASSET_NAME="rhinolabs-ai-${OS}-${ARCH}"
RLAI_ASSET="rlai-${OS}-${ARCH}"

# ── Helpers ──────────────────────────────────────────────────────────
info()    { echo -e "\033[0;36m  → $1\033[0m"; }
success() { echo -e "\033[0;32m  ✓ $1\033[0m"; }
warn()    { echo -e "\033[0;33m  ! $1\033[0m"; }
header()  { echo -e "\n\033[1;37m$1\033[0m\n"; }

remove_if_exists() {
  if [[ -e "$1" ]]; then
    rm -rf "$1"
    success "Removed: $1"
  else
    info "Not found (skipped): $1"
  fi
}

# ── Phase 1: Clean ──────────────────────────────────────────────────
header "══════════════════════════════════════════════════"
header "  Rhinolabs AI - Clean Install ($OS/$ARCH)"
header "══════════════════════════════════════════════════"

header "Phase 1: Removing existing installation"

# Remove CLI binaries
for bin in rhinolabs-ai rlai; do
  BIN_PATH=$(command -v "$bin" 2>/dev/null || true)
  if [[ -n "$BIN_PATH" ]]; then
    rm -f "$BIN_PATH"
    success "Removed binary: $BIN_PATH"
  else
    info "Binary not in PATH (skipped): $bin"
  fi
done

# Remove plugin
remove_if_exists "$PLUGIN_DIR"

# Remove config
remove_if_exists "$CONFIG_DIR"

# Remove user-level skills
remove_if_exists "$SKILLS_DIR"

success "Clean complete"

if [[ "$CLEAN_ONLY" == true ]]; then
  echo ""
  success "Clean-only mode. Done."
  exit 0
fi

# ── Phase 2: Download & Install CLI ─────────────────────────────────
header "Phase 2: Installing CLI from latest release"

INSTALL_DIR="/usr/local/bin"
if [[ ! -w "$INSTALL_DIR" ]]; then
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
  warn "No write access to /usr/local/bin, using $INSTALL_DIR"
  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    warn "Add $INSTALL_DIR to your PATH"
  fi
fi

info "Downloading $ASSET_NAME from $REPO..."
DOWNLOAD_URL=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" \
  | grep "browser_download_url.*${ASSET_NAME}\"" \
  | head -1 \
  | cut -d '"' -f 4)

if [[ -z "$DOWNLOAD_URL" ]]; then
  echo "Error: Could not find asset $ASSET_NAME in latest release"
  exit 1
fi

curl -sL "$DOWNLOAD_URL" -o "$INSTALL_DIR/rhinolabs-ai"
chmod +x "$INSTALL_DIR/rhinolabs-ai"
success "Installed: $INSTALL_DIR/rhinolabs-ai"

# Download rlai alias
RLAI_URL=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" \
  | grep "browser_download_url.*${RLAI_ASSET}\"" \
  | head -1 \
  | cut -d '"' -f 4)

if [[ -n "$RLAI_URL" ]]; then
  curl -sL "$RLAI_URL" -o "$INSTALL_DIR/rlai"
  chmod +x "$INSTALL_DIR/rlai"
  success "Installed: $INSTALL_DIR/rlai"
fi

# ── Phase 3: Install plugin ─────────────────────────────────────────
header "Phase 3: Installing plugin"

info "Running: rhinolabs-ai install"
"$INSTALL_DIR/rhinolabs-ai" install

# ── Done ─────────────────────────────────────────────────────────────
header "══════════════════════════════════════════════════"
success "Clean install complete!"
echo ""
info "Restart Claude Code to activate the plugin."
echo ""
