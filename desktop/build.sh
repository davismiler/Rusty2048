#!/bin/bash
set -e

echo "🚀 Building Rusty2048 Desktop version..."

# Check if Tauri CLI is installed
if ! command -v tauri &> /dev/null; then
    echo "❌ Tauri CLI is not installed. Installing..."
    cargo install tauri-cli
fi

# Build the desktop application
echo "📦 Building desktop application..."
cargo tauri build

echo "✅ Build complete! Desktop application is ready."
echo "📁 The built application can be found in:"
echo "   target/release/bundle/"
echo ""
echo "🎮 To run the desktop version in development mode:"
echo "   cargo tauri dev"
