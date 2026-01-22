# Rhinolabs Claude Plugin Installer for Windows
# Requires PowerShell 5.1+

$ErrorActionPreference = "Stop"

Write-Host "🚀 Rhinolabs Claude Plugin Installer" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

# Detect Windows version
$OS = "Windows"
Write-Host "✓ Detected: Windows" -ForegroundColor Green

# Set plugin directory
$PluginDir = "$env:APPDATA\Claude Code\plugins"

Write-Host ""

# Check if Claude Code is installed
if (-not (Test-Path "$env:APPDATA\Claude Code")) {
    Write-Host "⚠️  Warning: Claude Code installation not found" -ForegroundColor Yellow
    Write-Host "   Please install Claude Code first" -ForegroundColor Yellow
    $continue = Read-Host "   Continue anyway? (y/N)"
    if ($continue -ne "y" -and $continue -ne "Y") {
        exit 1
    }
}

# Check if plugin already exists
if (Test-Path "$PluginDir\rhinolabs-claude") {
    Write-Host "⚠️  Existing plugin found at: $PluginDir\rhinolabs-claude" -ForegroundColor Yellow
    $overwrite = Read-Host "   Overwrite? (y/N)"
    if ($overwrite -ne "y" -and $overwrite -ne "Y") {
        Write-Host "❌ Installation cancelled" -ForegroundColor Red
        exit 1
    }
    Write-Host "🗑️  Removing existing plugin..." -ForegroundColor Yellow
    Remove-Item -Path "$PluginDir\rhinolabs-claude" -Recurse -Force
}

# Create plugin directory
Write-Host "📁 Creating plugin directory..." -ForegroundColor Cyan
New-Item -ItemType Directory -Path $PluginDir -Force | Out-Null

# Get script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$PluginSource = Split-Path -Parent $ScriptDir

# Copy plugin files
Write-Host "📦 Installing plugin files..." -ForegroundColor Cyan
Copy-Item -Path $PluginSource -Destination "$PluginDir\rhinolabs-claude" -Recurse -Force

# Verify installation
if (Test-Path "$PluginDir\rhinolabs-claude\.claude-plugin\plugin.json") {
    Write-Host ""
    Write-Host "✅ Installation successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Plugin installed at: $PluginDir\rhinolabs-claude" -ForegroundColor White
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Restart Claude Code" -ForegroundColor White
    Write-Host "2. The plugin will be automatically loaded" -ForegroundColor White
    Write-Host "3. Check Claude Code settings to verify plugin is active" -ForegroundColor White
    Write-Host ""
} else {
    Write-Host ""
    Write-Host "❌ Installation failed!" -ForegroundColor Red
    Write-Host "   .claude-plugin/plugin.json not found in target directory" -ForegroundColor Red
    exit 1
}
