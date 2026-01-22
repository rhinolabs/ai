# Rhinolabs AI GUI

Desktop application for managing the Rhinolabs Claude plugin.

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Tech Stack

- **Backend**: Rust + Tauri
- **Frontend**: React + TypeScript + Vite
- **Shared Logic**: rhinolabs-core library

## Features

- Install/Update/Uninstall plugin
- Sync MCP configuration
- View status and diagnostics
- Interactive UI with real-time feedback

## Building

The GUI will be built as part of the release workflow and distributed as:
- **macOS**: .dmg
- **Windows**: .exe installer
- **Linux**: .AppImage
