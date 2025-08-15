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
- ✅ Code quality check (`cargo clippy`)
- ✅ Run all tests (`cargo test`)
- ✅ Build core projects (`core`, `shared`, `cli`)
- ✅ Build Web WASM module

**Features**:
- Fast execution (about 2-3 minutes)
- Parallel execution
- Cache dependencies to accelerate builds

### 2. Full CI (`ci.yml`)
**Purpose**: Complete build and test
**Trigger Conditions**: 
- Push to `main` branch
- Pull Request to `main` branch

**Check Content**:
- ✅ Code formatting and quality checks
- ✅ Run all tests
- ✅ Build all projects (CLI, Core, Shared)
- ✅ Build Web version (WASM + Vite build)
- ✅ Build Desktop version (Tauri)
- ✅ Cross-platform build testing (Linux, macOS, Windows)

**Features**:
- Comprehensive detection
- Cross-platform verification
- Build all release versions

## Status Badges

You can add the following badges to README.md to display CI status:

```markdown
[![Quick Check](https://github.com/{username}/rusty2048/workflows/Quick%20Check/badge.svg)](https://github.com/{username}/rusty2048/actions?query=workflow%3A%22Quick+Check%22)
[![CI](https://github.com/{username}/rusty2048/workflows/CI/badge.svg)](https://github.com/{username}/rusty2048/actions?query=workflow%3ACI)
```

## Local Testing

Before pushing code, it's recommended to run the following commands locally:

```bash
# Check code formatting
cargo fmt --all -- --check

# Run clippy check
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test --all

# Build all projects
cargo build --release -p rusty2048-core
cargo build --release -p rusty2048-shared
cargo build --release -p rusty2048-cli

# Build Web version
cd web
wasm-pack build --target web --out-dir public/pkg
npm run build
```

## Troubleshooting

### Common Issues

1. **Formatting check failed**
   ```bash
   cargo fmt --all
   ```

2. **Clippy warnings**
   ```bash
   cargo clippy --all-targets --all-features --fix
   ```

3. **Test failures**
   - Check test code
   - Ensure all dependencies are correctly installed

4. **Build failures**
   - Check dependency version compatibility
   - Ensure all necessary tools are installed

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
