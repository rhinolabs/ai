# Rhinolabs AI Plugin Installer for Windows
# Requires PowerShell 5.1+
# Targets: Claude Code, Amp, Antigravity (Gemini), OpenCode
#
# This script installs the plugin base configuration.
# Skills are installed via the rhinolabs-ai CLI using the "main" profile.

param(
    [Parameter()]
    [string[]]$Target,

    [Parameter()]
    [switch]$SkipSkills,

    [Parameter()]
    [switch]$Help
)

$ErrorActionPreference = "Stop"

# Available targets
$AvailableTargets = @("claude-code", "amp", "antigravity", "opencode")

# Show help
if ($Help) {
    Write-Host "Rhinolabs AI Plugin Installer" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\install.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Target TARGET    Install for specific target (can be used multiple times)"
    Write-Host "                    Available: claude-code, amp, antigravity, opencode, all"
    Write-Host "  -SkipSkills       Skip skill installation (plugin base only)"
    Write-Host "  -Help             Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\install.ps1                              # Interactive mode"
    Write-Host "  .\install.ps1 -Target claude-code          # Install for Claude Code only"
    Write-Host "  .\install.ps1 -Target claude-code,amp      # Install for Claude Code and Amp"
    Write-Host "  .\install.ps1 -Target all                  # Install for all targets"
    Write-Host "  .\install.ps1 -SkipSkills                  # Install plugin base only, no skills"
    exit 0
}

Write-Host "🚀 Rhinolabs AI Plugin Installer" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "✓ Detected: Windows" -ForegroundColor Green
Write-Host ""

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PluginSource = Split-Path -Parent $ScriptDir

# Functions to get paths for each target
function Get-ConfigDir {
    param([string]$TargetName)
    switch ($TargetName) {
        "claude-code" { return "$env:USERPROFILE\.claude" }
        "amp" { return "$env:APPDATA\agents" }
        "antigravity" { return "$env:USERPROFILE\.gemini\antigravity" }
        "opencode" { return "$env:APPDATA\opencode" }
    }
}

function Get-McpFilename {
    param([string]$TargetName)
    switch ($TargetName) {
        "claude-code" { return ".mcp.json" }
        "amp" { return "settings.json" }
        "antigravity" { return "config.json" }
        "opencode" { return "opencode.json" }
    }
}

function Get-DisplayName {
    param([string]$TargetName)
    switch ($TargetName) {
        "claude-code" { return "Claude Code" }
        "amp" { return "Amp" }
        "antigravity" { return "Antigravity (Gemini)" }
        "opencode" { return "OpenCode" }
    }
}

function Test-TargetInstalled {
    param([string]$TargetName)
    return Test-Path (Get-ConfigDir $TargetName)
}

function Test-CliAvailable {
    try {
        $null = Get-Command rhinolabs-ai -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

# Parse targets
$SelectedTargets = @()
if ($Target) {
    if ($Target -contains "all") {
        $SelectedTargets = $AvailableTargets
    } else {
        $SelectedTargets = $Target
    }
}

# Interactive target selection if none specified
if ($SelectedTargets.Count -eq 0) {
    Write-Host "Select targets to install:"
    Write-Host ""

    for ($i = 0; $i -lt $AvailableTargets.Count; $i++) {
        $t = $AvailableTargets[$i]
        $displayName = Get-DisplayName $t
        $configDir = Get-ConfigDir $t

        if (Test-TargetInstalled $t) {
            $status = "[installed]"
            $statusColor = "Green"
        } else {
            $status = "[not found]"
            $statusColor = "Yellow"
        }

        Write-Host "  $($i + 1). $displayName " -NoNewline
        Write-Host $status -ForegroundColor $statusColor
        Write-Host "      Config: $configDir" -ForegroundColor DarkGray
    }

    Write-Host "  5. All targets"
    Write-Host ""

    $selection = Read-Host "Enter numbers separated by space (e.g., '1 2' or '5' for all)"
    Write-Host ""

    if ($selection -match "5") {
        $SelectedTargets = $AvailableTargets
    } else {
        $nums = $selection -split '\s+' | Where-Object { $_ -match '^\d+$' }
        foreach ($num in $nums) {
            $idx = [int]$num - 1
            if ($idx -ge 0 -and $idx -lt $AvailableTargets.Count) {
                $SelectedTargets += $AvailableTargets[$idx]
            }
        }
    }
}

if ($SelectedTargets.Count -eq 0) {
    Write-Host "❌ No targets selected" -ForegroundColor Red
    exit 1
}

Write-Host "Selected targets:"
foreach ($t in $SelectedTargets) {
    Write-Host "  • $(Get-DisplayName $t)" -ForegroundColor Cyan
}
Write-Host ""

# Confirm installation
$confirm = Read-Host "Continue with installation? (Y/n)"
if ($confirm -eq "n" -or $confirm -eq "N") {
    Write-Host "❌ Installation cancelled" -ForegroundColor Yellow
    exit 1
}
Write-Host ""

# Install plugin base for each target
foreach ($t in $SelectedTargets) {
    $displayName = Get-DisplayName $t
    $configDir = Get-ConfigDir $t

    Write-Host "📦 Installing plugin base for $displayName..." -ForegroundColor Cyan

    # Create directories
    New-Item -ItemType Directory -Path $configDir -Force | Out-Null
    New-Item -ItemType Directory -Path "$configDir\output-styles" -Force | Out-Null

    # Copy output style
    if (Test-Path "$PluginSource\output-styles\rhinolabs.md") {
        Copy-Item -Path "$PluginSource\output-styles\rhinolabs.md" -Destination "$configDir\output-styles\" -Force
        Write-Host "   ✓ Output style installed" -ForegroundColor Green
    }

    # NOTE: MCP config is NOT deployed by this script.
    # MCP servers (including rhinolabs-rag) should be configured via the GUI.
    # See docs/RAG_MCP_ARCHITECTURE.md for details.

    # Target-specific installations
    switch ($t) {
        "claude-code" {
            # Copy Claude Code plugin (without skills - those come from CLI)
            $PluginDir = "$env:APPDATA\Claude Code\plugins"

            if (Test-Path "$PluginDir\rhinolabs-claude") {
                Write-Host "   ⚠️  Existing plugin found, overwriting..." -ForegroundColor Yellow
                Remove-Item -Path "$PluginDir\rhinolabs-claude" -Recurse -Force
            }

            New-Item -ItemType Directory -Path "$PluginDir\rhinolabs-claude" -Force | Out-Null

            # Copy plugin structure (excluding skills directory)
            Copy-Item -Path "$PluginSource\.claude-plugin" -Destination "$PluginDir\rhinolabs-claude\" -Recurse -Force
            if (Test-Path "$PluginSource\output-styles") {
                Copy-Item -Path "$PluginSource\output-styles" -Destination "$PluginDir\rhinolabs-claude\" -Recurse -Force -ErrorAction SilentlyContinue
            }
            if (Test-Path "$PluginSource\settings.json") {
                Copy-Item -Path "$PluginSource\settings.json" -Destination "$PluginDir\rhinolabs-claude\" -Force
            }
            # NOTE: .mcp.json is NOT copied - MCP config is managed via GUI
            if (Test-Path "$PluginSource\statusline.sh") {
                Copy-Item -Path "$PluginSource\statusline.sh" -Destination "$PluginDir\rhinolabs-claude\" -Force
            }

            Write-Host "   ✓ Plugin installed to $PluginDir\rhinolabs-claude" -ForegroundColor Green

            # Handle settings.json (merge instead of overwrite)
            if (Test-Path "$PluginSource\settings.json") {
                if (Test-Path "$configDir\settings.json") {
                    Write-Host "   → Merging settings.json..." -ForegroundColor Cyan

                    # Load both JSON files
                    $existing = Get-Content "$configDir\settings.json" -Raw | ConvertFrom-Json -AsHashtable
                    $new = Get-Content "$PluginSource\settings.json" -Raw | ConvertFrom-Json -AsHashtable

                    # Deep merge function (new values only added if key doesn't exist)
                    function Merge-Hashtable {
                        param($Base, $Override)
                        $result = @{}

                        # Add all keys from base
                        foreach ($key in $Base.Keys) {
                            if ($Override.ContainsKey($key)) {
                                if ($Base[$key] -is [hashtable] -and $Override[$key] -is [hashtable]) {
                                    $result[$key] = Merge-Hashtable $Base[$key] $Override[$key]
                                } elseif ($Base[$key] -is [array] -and $Override[$key] -is [array]) {
                                    # Merge arrays and remove duplicates
                                    $result[$key] = @($Base[$key] + $Override[$key] | Select-Object -Unique)
                                } else {
                                    # Keep existing value
                                    $result[$key] = $Base[$key]
                                }
                            } else {
                                $result[$key] = $Base[$key]
                            }
                        }

                        # Add keys from override that don't exist in base
                        foreach ($key in $Override.Keys) {
                            if (-not $Base.ContainsKey($key)) {
                                $result[$key] = $Override[$key]
                            }
                        }

                        return $result
                    }

                    $merged = Merge-Hashtable $existing $new
                    $merged | ConvertTo-Json -Depth 10 | Set-Content "$configDir\settings.json" -Encoding UTF8
                    Write-Host "   ✓ Settings merged (your settings preserved)" -ForegroundColor Green
                } else {
                    Copy-Item -Path "$PluginSource\settings.json" -Destination $configDir -Force
                    Write-Host "   ✓ Settings installed" -ForegroundColor Green
                }
            }
        }
    }

    Write-Host ""
}

Write-Host "✅ Plugin base installation complete!" -ForegroundColor Green
Write-Host ""

# Install skills via CLI if available
if (-not $SkipSkills) {
    Write-Host "📚 Installing skills via CLI..." -ForegroundColor Cyan
    Write-Host ""

    if (Test-CliAvailable) {
        # Build target arguments for CLI
        $targetArgs = @()
        foreach ($t in $SelectedTargets) {
            $targetArgs += "--target"
            $targetArgs += $t
        }

        Write-Host "   → Running: rhinolabs-ai profile install main $($targetArgs -join ' ')" -ForegroundColor Cyan
        Write-Host ""

        # Run the CLI to install the main profile's skills
        try {
            & rhinolabs-ai profile install main @targetArgs
            Write-Host ""
            Write-Host "   ✓ Skills installed via main profile" -ForegroundColor Green
        } catch {
            Write-Host ""
            Write-Host "   ⚠️  CLI skill installation failed" -ForegroundColor Yellow
            Write-Host "   →  You can manually run: rhinolabs-ai profile install main" -ForegroundColor Yellow
        }
    } else {
        Write-Host "   ⚠️  rhinolabs-ai CLI not found" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "   To install skills, first install the CLI:"
        Write-Host ""
        Write-Host "   Option 1: Download from releases"
        Write-Host "     Visit the Releases page and download for your platform"
        Write-Host ""
        Write-Host "   Option 2: Build from source"
        Write-Host "     cd rhinolabs-ai\cli; cargo build --release"
        Write-Host ""
        Write-Host "   Then run:"
        Write-Host "     rhinolabs-ai profile install main --target all"
        Write-Host ""
    }
}

# Summary
Write-Host ""
Write-Host "Installed for:"
foreach ($t in $SelectedTargets) {
    $displayName = Get-DisplayName $t
    $configDir = Get-ConfigDir $t
    Write-Host "  • $displayName → $configDir" -ForegroundColor Green
}
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Restart your AI coding assistant(s)"
if (-not (Test-CliAvailable) -and -not $SkipSkills) {
    Write-Host "  2. Install rhinolabs-ai CLI to manage skills and profiles"
} else {
    Write-Host "  2. The plugin and skills will be automatically loaded"
}
Write-Host ""
