# GitHub Actions CI/CD

This project uses GitHub Actions for continuous integration and deployment.

## Workflow Files

### 1. Quick Check (`quick-check.yml`)
**Purpose**: Quick project status check
**Trigger Conditions**: 
- Push to `main` branch
- Pull Request to `main` branch

**Check Content**:
- ✅ Code formatting check (`cargo fmt`)
- ✅ Code quality check (`cargo clippy`) - Core modules only
- ✅ Run tests - Core modules only (`core`, `shared`, `cli`, `web`)
- ✅ Build core projects (`core`, `shared`, `cli`)
- ✅ Build Web WASM module

**Features**:
- Fast execution (about 2-3 minutes)
- Parallel execution
- Cache dependencies to accelerate builds
- Excludes desktop module to avoid GTK dependency issues

### 2. Full CI (`ci.yml`)
**Purpose**: Complete build and test
**Trigger Conditions**: 
- Push to `main` branch
- Pull Request to `main` branch

**Check Content**:
- ✅ Code formatting and quality checks - Core modules only
- ✅ Run tests - Core modules only (`core`, `shared`, `cli`, `web`)
- ✅ Build core projects (CLI, Core, Shared)
- ✅ Build Web version (WASM + Vite build)
- ✅ Build Desktop version (Tauri) - with system dependencies
- ✅ Cross-platform build testing (Linux, macOS, Windows)

**Features**:
- Comprehensive detection
- Cross-platform verification
- Build all release versions
- Desktop build includes GTK system dependencies

## Module Separation Strategy

To avoid GTK dependency issues in CI, we've implemented a module separation strategy:

### Core Modules (No GTK Dependencies)
- `rusty2048-core`: Game logic and core functionality
- `rusty2048-shared`: Shared utilities and internationalization
- `rusty2048-cli`: Terminal-based interface
- `rusty2048-web`: Web/WASM version

### Desktop Module (GTK Dependencies)
- `rusty2048-desktop`: Tauri-based desktop application
- Requires GTK system libraries (glib, gtk, fontconfig, etc.)
- Built separately with proper system dependencies

## Status Badges

You can add the following badges to README.md to display CI status:

```markdown
[![Quick Check](https://github.com/{username}/rusty2048/workflows/Quick%20Check/badge.svg)](https://github.com/{username}/rusty2048/actions?query=workflow%3A%22Quick+Check%22)
[![CI](https://github.com/{username}/rusty2048/workflows/CI/badge.svg)](https://github.com/{username}/rusty2048/actions?query=workflow%3ACI)
```

## Local Testing

Before pushing code, it's recommended to run the following commands locally:

```bash
# Check code formatting (core modules only)
cargo fmt -p rusty2048-core -- --check
cargo fmt -p rusty2048-shared -- --check
cargo fmt -p rusty2048-cli -- --check
cargo fmt -p rusty2048-web -- --check

# Run clippy check (core modules only)
cargo clippy -p rusty2048-core --all-targets --all-features -- -D warnings
cargo clippy -p rusty2048-shared --all-targets --all-features -- -D warnings
cargo clippy -p rusty2048-cli --all-targets --all-features -- -D warnings
cargo clippy -p rusty2048-web --all-targets --all-features -- -D warnings

# Run tests (core modules only)
cargo test -p rusty2048-core
cargo test -p rusty2048-shared
cargo test -p rusty2048-cli
cargo test -p rusty2048-web

# Build core projects
cargo build --release -p rusty2048-core
cargo build --release -p rusty2048-shared
cargo build --release -p rusty2048-cli

# Build Web version
cd web
export FONTCONFIG_NO_PKG_CONFIG=1
export RUST_FONTCONFIG_DLOPEN=1
wasm-pack build --target web --out-dir public/pkg
npm run build

# Build Desktop version (requires GTK dependencies)
cd ../desktop
cargo tauri build --ci
```

## System Dependencies for Desktop Build

The desktop module requires the following system dependencies on Linux:

```bash
sudo apt-get update

# Install essential build tools
sudo apt-get install -y build-essential pkg-config

# Try to install WebKit2GTK 4.1 first, fallback to 4.0 if not available
if sudo apt-get install -y libwebkit2gtk-4.1-dev; then
  echo "WebKit2GTK 4.1 installed successfully"
else
  echo "WebKit2GTK 4.1 not available, trying 4.0..."
  sudo apt-get install -y libwebkit2gtk-4.0-dev
fi

# Install other GTK dependencies
sudo apt-get install -y \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libfontconfig1-dev \
  libfontconfig-dev \
  libglib2.0-dev \
  libgirepository1.0-dev \
  libcairo2-dev \
  libpango1.0-dev \
  libatk1.0-dev \
  libgdk-pixbuf2.0-dev \
  fontconfig \
  libfreetype6-dev

# Verify installation
pkg-config --exists gtk+-3.0 && echo "GTK+ 3.0 found"
pkg-config --exists fontconfig && echo "FontConfig found"
if pkg-config --exists webkit2gtk-4.1; then
  echo "WebKit2GTK 4.1 found"
elif pkg-config --exists webkit2gtk-4.0; then
  echo "WebKit2GTK 4.0 found"
else
  echo "WebKit2GTK not found - this may cause build issues"
  exit 1
fi

# Show pkg-config info for debugging
echo "FontConfig pkg-config info:"
pkg-config --libs --cflags fontconfig || echo "FontConfig pkg-config failed"

# Set environment variables for fontconfig (if needed)
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig
export FONTCONFIG_PATH=/etc/fonts
```

## Troubleshooting

### Common Issues

1. **Formatting check failed**
   ```bash
   # For core modules only
   cargo fmt -p rusty2048-core
   cargo fmt -p rusty2048-shared
   cargo fmt -p rusty2048-cli
   cargo fmt -p rusty2048-web
   
   # Or format all (including desktop)
   cargo fmt --all
   ```

2. **Clippy warnings**
   ```bash
   # For core modules
   cargo clippy -p rusty2048-core --all-targets --all-features --fix
   cargo clippy -p rusty2048-shared --all-targets --all-features --fix
   cargo clippy -p rusty2048-cli --all-targets --all-features --fix
   cargo clippy -p rusty2048-web --all-targets --all-features --fix
   ```

3. **Test failures**
   - Check test code
   - Ensure all dependencies are correctly installed

4. **Build failures**
   - Check dependency version compatibility
   - Ensure all necessary tools are installed

5. **GTK dependency errors**
   - Ensure system dependencies are installed (for desktop builds)
   - Use module-specific commands to avoid GTK dependencies in core builds
   - If WebKit2GTK packages are not found, try:
     ```bash
     # For Ubuntu 20.04 or older
     sudo apt-get install -y libwebkit2gtk-4.0-dev
     
     # For Ubuntu 22.04 or newer
     sudo apt-get install -y libwebkit2gtk-4.1-dev
     
     # Alternative: install from source
     sudo apt-get install -y libwebkit2gtk-4.0-dev libwebkit2gtk-4.1-dev
     ```
   
6. **FontConfig errors**
   - Ensure fontconfig is properly installed:
     ```bash
     sudo apt-get install -y libfontconfig1-dev libfontconfig-dev fontconfig
     ```
   - Set environment variables:
     ```bash
     export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig
     export FONTCONFIG_PATH=/etc/fonts
     ```
   - Verify installation:
     ```bash
     pkg-config --exists fontconfig && echo "FontConfig found"
     pkg-config --libs --cflags fontconfig
     ```
   - For WASM builds, disable fontconfig pkg-config:
     ```bash
     export FONTCONFIG_NO_PKG_CONFIG=1
     export RUST_FONTCONFIG_DLOPEN=1
     ```

### Cache Issues

If you encounter cache-related issues, you can manually clear the cache in GitHub Actions:
1. Go to the Actions page
2. Select the failed workflow
3. Click "Re-run jobs" and select "Re-run all jobs"

## Performance Optimization

- Use dependency caching to accelerate builds
- Run independent tasks in parallel
- Use the latest GitHub Actions versions
- Regularly update dependency versions
- Module separation reduces build time and dependency conflicts
