#!/bin/bash

# Homebrew Tap Setup Script
# For publishing Rusty2048 CLI to Homebrew

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}🚀${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

# Check required tools
check_requirements() {
    if ! command -v git &> /dev/null; then
        print_error "Git not installed"
        exit 1
    fi
    
    if ! command -v curl &> /dev/null; then
        print_error "curl not installed"
        exit 1
    fi
}

# Create Homebrew formula
create_formula() {
    local version=$1
    local sha256=$2
    
    cat > Formula/rusty2048-cli.rb << EOF
class Rusty2048Cli < Formula
  desc "Command-line interface for Rusty2048 - A modern 2048 game with AI, themes, and statistics"
  homepage "https://github.com/honkinglin/rusty2048"
  version "${version}"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/honkinglin/rusty2048/releases/download/v${version}/rusty2048-cli-macos-aarch64.tar.gz"
      sha256 "${sha256}"
    else
      url "https://github.com/honkinglin/rusty2048/releases/download/v${version}/rusty2048-cli-macos-x86_64.tar.gz"
      sha256 "${sha256}"
    end
  end
  
  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/honkinglin/rusty2048/releases/download/v${version}/rusty2048-cli-linux-aarch64.tar.gz"
      sha256 "${sha256}"
    else
      url "https://github.com/honkinglin/rusty2048/releases/download/v${version}/rusty2048-cli-linux-x86_64.tar.gz"
      sha256 "${sha256}"
    end
  end
  
  def install
    bin.install "rusty2048"
  end
  
  test do
    system "#{bin}/rusty2048", "--help"
  end
end
EOF
}

# Calculate SHA256 of file
calculate_sha256() {
    local file=$1
    if command -v shasum &> /dev/null; then
        shasum -a 256 "$file" | cut -d' ' -f1
    elif command -v sha256sum &> /dev/null; then
        sha256sum "$file" | cut -d' ' -f1
    else
        print_error "SHA256 calculation tool not found"
        exit 1
    fi
}

# Main function
main() {
    print_status "Setting up Homebrew Tap..."
    
    check_requirements
    
    # Get version number
    read -p "Enter version number (e.g., 1.0.0): " version
    if [ -z "$version" ]; then
        print_error "Version number cannot be empty"
        exit 1
    fi
    
    # Detect system architecture
    local arch=$(uname -m)
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    
    print_status "Detected system: ${os} ${arch}"
    
    # Download corresponding binary file
    local download_url="https://github.com/honkinglin/rusty2048/releases/download/v${version}/rusty2048-cli-${os}-${arch}.tar.gz"
    local temp_file="rusty2048-cli-${version}.tar.gz"
    
    print_status "Downloading binary file..."
    if ! curl -L -o "$temp_file" "$download_url"; then
        print_error "Download failed. Please check version number and network connection"
        exit 1
    fi
    
    # Calculate SHA256
    print_status "Calculating SHA256..."
    local sha256=$(calculate_sha256 "$temp_file")
    print_success "SHA256: ${sha256}"
    
    # Create Formula directory
    mkdir -p Formula
    
    # Create formula file
    create_formula "$version" "$sha256"
    
    print_success "Homebrew formula created: Formula/rusty2048-cli.rb"
    print_status "Next steps:"
    echo "1. Commit Formula/rusty2048-cli.rb to your Homebrew tap repository"
    echo "2. Users can install with:"
    echo "   brew install honkinglin/tap/rusty2048-cli"
    
    # Clean up temporary file
    rm -f "$temp_file"
}

main "$@"
