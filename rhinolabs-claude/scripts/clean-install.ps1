# Rhinolabs AI - Clean Install Script (Windows)
# Removes ALL rhinolabs-ai artifacts and reinstalls from latest release.
#
# Usage:
#   .\clean-install.ps1              # Full clean + reinstall
#   .\clean-install.ps1 -CleanOnly   # Only remove, don't reinstall
#
# Requires PowerShell 5.1+

param(
    [switch]$CleanOnly,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

# ── Config ──────────────────────────────────────────────────────────
$Repo = "javiermontescarrera/rhinolabs-ai"

if ($Help) {
    Write-Host "Rhinolabs AI - Clean Install Script" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\clean-install.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -CleanOnly   Only remove existing installation (don't reinstall)"
    Write-Host "  -Help        Show this help"
    Write-Host ""
    exit 0
}

# ── Helpers ──────────────────────────────────────────────────────────
function Write-Info    { param($Msg) Write-Host "  -> $Msg" -ForegroundColor Cyan }
function Write-Ok      { param($Msg) Write-Host "  OK $Msg" -ForegroundColor Green }
function Write-Warn    { param($Msg) Write-Host "  !  $Msg" -ForegroundColor Yellow }
function Write-Header  { param($Msg) Write-Host "`n$Msg`n" -ForegroundColor White }

function Remove-IfExists {
    param([string]$Path)
    if (Test-Path $Path) {
        Remove-Item -Path $Path -Recurse -Force
        Write-Ok "Removed: $Path"
    } else {
        Write-Info "Not found (skipped): $Path"
    }
}

# ── Path Resolution ─────────────────────────────────────────────────
$PluginDir  = "$env:APPDATA\Claude Code\plugins\rhinolabs-claude"
$ConfigDir  = "$env:APPDATA\rhinolabs-ai"
$SkillsDir  = "$env:USERPROFILE\.claude\skills"
$AssetName  = "rhinolabs-ai-windows-x64.exe"
$RlaiAsset  = "rlai-windows-x64.exe"

# ── Phase 1: Clean ──────────────────────────────────────────────────
Write-Header "======================================================"
Write-Header "  Rhinolabs AI - Clean Install (Windows)"
Write-Header "======================================================"

Write-Header "Phase 1: Removing existing installation"

# Remove CLI binaries
foreach ($bin in @("rhinolabs-ai.exe", "rlai.exe")) {
    $binPath = (Get-Command $bin -ErrorAction SilentlyContinue).Source
    if ($binPath) {
        Remove-Item -Path $binPath -Force
        Write-Ok "Removed binary: $binPath"
    } else {
        Write-Info "Binary not in PATH (skipped): $bin"
    }
}

# Remove plugin
Remove-IfExists $PluginDir

# Remove config
Remove-IfExists $ConfigDir

# Remove user-level skills
Remove-IfExists $SkillsDir

Write-Ok "Clean complete"

if ($CleanOnly) {
    Write-Host ""
    Write-Ok "Clean-only mode. Done."
    exit 0
}

# ── Phase 2: Download & Install CLI ─────────────────────────────────
Write-Header "Phase 2: Installing CLI from latest release"

$InstallDir = "$env:USERPROFILE\.local\bin"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Check if InstallDir is in PATH
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$InstallDir*") {
    Write-Warn "$InstallDir is not in your PATH"
    $addToPath = Read-Host "Add it to PATH? (Y/n)"
    if ($addToPath -ne "n" -and $addToPath -ne "N") {
        [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallDir", "User")
        $env:PATH = "$env:PATH;$InstallDir"
        Write-Ok "Added $InstallDir to PATH"
    }
}

Write-Info "Fetching latest release from $Repo..."
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"

# Download rhinolabs-ai.exe
$cliAsset = $release.assets | Where-Object { $_.name -eq $AssetName }
if (-not $cliAsset) {
    Write-Host "Error: Asset $AssetName not found in release" -ForegroundColor Red
    exit 1
}

Write-Info "Downloading $AssetName..."
Invoke-WebRequest -Uri $cliAsset.browser_download_url -OutFile "$InstallDir\rhinolabs-ai.exe"
Write-Ok "Installed: $InstallDir\rhinolabs-ai.exe"

# Download rlai.exe alias
$rlaiAssetObj = $release.assets | Where-Object { $_.name -eq $RlaiAsset }
if ($rlaiAssetObj) {
    Invoke-WebRequest -Uri $rlaiAssetObj.browser_download_url -OutFile "$InstallDir\rlai.exe"
    Write-Ok "Installed: $InstallDir\rlai.exe"
}

# ── Phase 3: Install plugin ─────────────────────────────────────────
Write-Header "Phase 3: Installing plugin"

Write-Info "Running: rhinolabs-ai install"
& "$InstallDir\rhinolabs-ai.exe" install

# ── Done ─────────────────────────────────────────────────────────────
Write-Header "======================================================"
Write-Ok "Clean install complete!"
Write-Host ""
Write-Info "Restart Claude Code to activate the plugin."
Write-Host ""
