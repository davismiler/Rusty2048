# Rusty2048 Desktop

Desktop application for the Rusty2048 game built with Tauri.

## Features

- **Native Performance**: Built with Rust backend and web frontend
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Theme System**: 5 beautiful themes (Classic, Dark, Neon, Retro, Pastel)
- **Full Game Logic**: Complete 2048 game implementation
- **Undo Support**: Undo your last move
- **Statistics**: Track your score, best score, and moves
- **Responsive UI**: Modern and intuitive interface

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Node.js (for Tauri CLI)
- Platform-specific dependencies:
  - **Windows**: Microsoft Visual Studio C++ Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `build-essential`, `libwebkit2gtk-4.0-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`

### Installation

1. **Install Tauri CLI**:
   ```bash
   cargo install tauri-cli
   ```

2. **Build the application**:
   ```bash
   ./build.sh
   ```

3. **Run in development mode**:
   ```bash
   cargo tauri dev
   ```

## Controls

- **Arrow Keys** or **WASD**: Move tiles
- **Mouse**: Click buttons for New Game, Undo
- **Theme Buttons**: Click to switch themes

## Project Structure

```
desktop/
├── src/
│   └── main.rs          # Tauri backend (Rust)
├── frontend/
│   └── index.html       # Frontend UI (HTML/CSS/JS)
├── tauri.conf.json      # Tauri configuration
├── build.sh             # Build script
└── README.md           # This file
```

## Technical Details

### Backend (Rust)
- Uses `rusty2048-core` for game logic
- Uses `rusty2048-shared` for theme system
- Tauri commands for game operations
- State management with Tauri's state system

### Frontend (Web)
- Pure HTML/CSS/JavaScript
- No framework dependencies
- Responsive design
- Theme switching
- Keyboard and mouse controls

### Build Process
1. Rust code compiles to native binary
2. Frontend assets are bundled
3. Tauri creates platform-specific packages
4. Final application is self-contained

## Development

### Adding New Features

1. **Backend**: Add new Tauri commands in `src/main.rs`
2. **Frontend**: Update UI in `frontend/index.html`
3. **Testing**: Use `cargo tauri dev` for development

### Debugging

- Use browser dev tools for frontend debugging
- Check console for JavaScript errors
- Use `cargo tauri dev` for hot reloading

## Distribution

### Building for Distribution

```bash
cargo tauri build --release
```

This creates platform-specific packages in `target/release/bundle/`:
- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image
- **Linux**: `.deb` and `.AppImage` packages

### Code Signing

For production releases, you'll need to configure code signing in `tauri.conf.json`.

## Performance

- **Startup Time**: < 1 second
- **Memory Usage**: ~50MB
- **CPU Usage**: Minimal during gameplay
- **File Size**: ~10-20MB (depending on platform)

## Browser Support

The frontend uses modern web APIs but maintains compatibility with:
- Chrome/Chromium 80+
- Firefox 75+
- Safari 13+
- Edge 80+

## Troubleshooting

### Common Issues

1. **Build fails on Linux**:
   ```bash
   sudo apt install build-essential libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
   ```

2. **Build fails on macOS**:
   ```bash
   xcode-select --install
   ```

3. **Build fails on Windows**:
   Install Microsoft Visual Studio C++ Build Tools

### Getting Help

- Check the [Tauri documentation](https://tauri.app/docs/)
- Review the [Rusty2048 core documentation](../core/README.md)
- Open an issue in the project repository

## License

This project is licensed under the MIT License - see the main [README.md](../README.md) for details.
