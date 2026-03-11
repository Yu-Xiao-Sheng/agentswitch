#!/bin/sh
#
# AgentSwitch Installer
# Usage: curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
#
# Environment variables:
#   INSTALL_DIR - Installation directory (default: /usr/local/bin)
#   NO_MODIFY_PATH - Don't modify PATH (default: false)
#   ASW_VERSION - Specific version to install (default: latest)
#
# Copyright 2026, Yu-Xiao-Sheng
# License: MIT

set -e

# ============================================================================
# Script Metadata
# ============================================================================

VERSION="1.0.0"
REPO_OWNER="Yu-Xiao-Sheng"
REPO_NAME="agentswitch"
GITHUB_BASE_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}"

# ============================================================================
# Global Variables
# ============================================================================

# Default values
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
NO_MODIFY_PATH="${NO_MODIFY_PATH:-false}"
ASW_VERSION="${ASW_VERSION:-latest}"
FORCE_INSTALL="${FORCE_INSTALL:-false}"
LOCAL_FILE=""

# Temporary directory
TMP_DIR="$(mktemp -d -t asw-install.XXXXXX)"

# Detect system
OS="$(uname -s)"
ARCH="$(uname -m)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ============================================================================
# Cleanup Handler
# ============================================================================

cleanup() {
    # Remove temporary directory
    if [ -d "$TMP_DIR" ]; then
        rm -rf "$TMP_DIR"
    fi
}

# Trap cleanup on exit
trap cleanup EXIT

# ============================================================================
# Utility Functions
# ============================================================================

# Print colored message
print_success() {
    printf "${GREEN}✓${NC} %s\n" "$1"
}

print_error() {
    printf "${RED}✗${NC} Error: %s\n" "$1" >&2
}

print_warning() {
    printf "${YELLOW}⚠${NC} Warning: %s\n" "$1"
}

print_info() {
    printf "${BLUE}ℹ${NC} %s\n" "$1"
}

# Print header
print_header() {
    printf "\n${BLUE}AgentSwitch Installer v${VERSION}${NC}\n\n"
}

# ============================================================================
# System Detection Functions
# ============================================================================

detect_system() {
    print_info "Detecting system..."
    
    # Map architecture
    case "$ARCH" in
        x86_64|amd64|x64)
            ARCH="x86_64"
            ;;
        aarch64|arm64|armv8*)
            ARCH="aarch64"
            ;;
        armv7*|armv6*)
            ARCH="armv7"
            ;;
        *)
            print_error "Unsupported architecture: $ARCH"
            echo ""
            echo "Supported architectures:"
            echo "  - x86_64 (AMD64/Intel 64-bit)"
            echo "  - aarch64 (ARM64/Apple Silicon)"
            echo ""
            echo "Please install from source:"
            echo "  https://github.com/${REPO_OWNER}/${REPO_NAME}#building-from-source"
            exit 1
            ;;
    esac
    
    # Map OS
    case "$OS" in
        Linux)
            OS="unknown-linux-gnu"
            ;;
        Darwin)
            OS="apple-darwin"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="pc-windows-msvc"
            ;;
        *)
            print_error "Unsupported operating system: $OS"
            echo ""
            echo "Supported operating systems:"
            echo "  - Linux"
            echo "  - macOS"
            echo ""
            exit 1
            ;;
    esac
    
    print_success "Detected system: $OS $ARCH"
}

# Detect shell type for completion
detect_shell() {
    if [ -n "$ZSH_VERSION" ]; then
        echo "zsh"
    elif [ -n "$BASH_VERSION" ]; then
        echo "bash"
    elif [ "$(basename "$SHELL")" = "fish" ]; then
        echo "fish"
    else
        # Default to bash
        echo "bash"
    fi
}

# ============================================================================
# Version Functions
# ============================================================================

get_latest_version() {
    if [ "$ASW_VERSION" = "latest" ]; then
        # Fetch latest version from GitHub API
        print_info "Fetching latest version from GitHub..."
        
        API_URL="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"
        VERSION_INFO=$(curl -sSf "$API_URL" 2>/dev/null || echo "")
        
        if [ -z "$VERSION_INFO" ]; then
            print_error "Failed to fetch version information from GitHub"
            exit 1
        fi
        
        ASW_VERSION=$(echo "$VERSION_INFO" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/' | sed 's/^v//')
        
        if [ -z "$ASW_VERSION" ]; then
            print_error "Failed to parse version from GitHub response"
            exit 1
        fi
    fi
    
    print_success "Version: $ASW_VERSION"
}

# ============================================================================
# Download Functions
# ============================================================================

download_binary() {
    # If using local file, copy it
    if [ -n "$LOCAL_FILE" ]; then
        print_info "Using local binary: $LOCAL_FILE"

        # Check if local file exists
        if [ ! -f "$LOCAL_FILE" ]; then
            print_error "Local file not found: $LOCAL_FILE"
            exit 1
        fi

        # Copy local binary to temp directory
        cp "$LOCAL_FILE" "${TMP_DIR}/asw"
        chmod +x "${TMP_DIR}/asw"

        # Verify binary
        if ! "${TMP_DIR}/asw" --version >/dev/null 2>&1; then
            print_error "Local binary is not valid or corrupted"
            exit 1
        fi

        print_success "Local binary copied and verified"
        return 0
    fi

    # Otherwise, download from GitHub
    BINARY_NAME="agentswitch-${ARCH}-${OS}"
    DOWNLOAD_URL="${GITHUB_BASE_URL}/releases/download/v${ASW_VERSION}/${BINARY_NAME}.tar.gz"

    print_info "Downloading AgentSwitch ${ASW_VERSION} for ${ARCH}..."

    # Download with retry
    MAX_RETRIES=3
    RETRY_COUNT=0

    while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
        if curl -fSLO --retry 3 --retry-delay 2 "$DOWNLOAD_URL" -o "${TMP_DIR}/${BINARY_NAME}.tar.gz" 2>/dev/null; then
            print_success "Download completed"
            return 0
        fi

        RETRY_COUNT=$((RETRY_COUNT + 1))

        if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
            print_warning "Download failed, retrying... ($RETRY_COUNT/$MAX_RETRIES)"
            sleep 2
        fi
    done

    print_error "Failed to download after $MAX_RETRIES attempts"
    echo ""
    echo "You can download manually from:"
    echo "  ${GITHUB_BASE_URL}/releases/v${ASW_VERSION}"
    exit 1
}

extract_binary() {
    # If using local file, skip extraction
    if [ -n "$LOCAL_FILE" ]; then
        print_info "Using local binary (skipping extraction)"
        return 0
    fi

    BINARY_NAME="agentswitch-${ARCH}-${OS}"

    print_info "Extracting binary..."

    # Extract tarball
    tar -xzf "${TMP_DIR}/${BINARY_NAME}.tar.gz" -C "$TMP_DIR"

    # Find the binary (it might be in a subdirectory)
    EXTRACTED_BINARY=$(find "$TMP_DIR" -type f -name "asw" | head -n 1)

    if [ -z "$EXTRACTED_BINARY" ]; then
        print_error "Failed to find binary in archive"
        exit 1
    fi

    # Move to tmp root
    mv "$EXTRACTED_BINARY" "${TMP_DIR}/asw"
    chmod +x "${TMP_DIR}/asw"

    # Verify binary
    if ! "${TMP_DIR}/asw" --version >/dev/null 2>&1; then
        print_error "Downloaded binary is not valid or corrupted"
        exit 1
    fi

    print_success "Binary extracted and verified"
}

# ============================================================================
# Installation Functions
# ============================================================================

check_install_dir() {
    print_info "Checking installation directory..."
    
    # Check if INSTALL_DIR exists
    if [ ! -d "$INSTALL_DIR" ]; then
        print_info "Creating directory: $INSTALL_DIR"
        mkdir -p "$INSTALL_DIR" || {
            print_error "Failed to create directory: $INSTALL_DIR"
            echo ""
            echo "You may need to:"
            echo "  1. Run with sudo: sudo bash $0"
            echo "  2. Or set INSTALL_DIR to your home directory:"
            echo "     curl -sSL ... | bash -s -- --install-dir ~/bin"
            exit 1
        }
    fi
    
    # Check write permission
    if [ ! -w "$INSTALL_DIR" ]; then
        print_error "No write permission for: $INSTALL_DIR"
        echo ""
        echo "You may need to:"
        echo "  1. Run with sudo: sudo bash $0"
        echo "  2. Or set INSTALL_DIR to your home directory:"
        echo "     curl -sSL ... | INSTALL_DIR=~/bin bash"
        exit 1
    fi
    
    print_success "Installation directory OK: $INSTALL_DIR"
}

check_existing_installation() {
    if [ -x "${INSTALL_DIR}/asw" ]; then
        EXISTING_VERSION=$("${INSTALL_DIR}/asw" --version 2>/dev/null | head -n 1 || echo "unknown")
        
        if [ "$FORCE_INSTALL" != "true" ]; then
            echo ""
            echo "AgentSwitch $EXISTING_VERSION is already installed at ${INSTALL_DIR}/asw"
            echo ""
            printf "Do you want to upgrade to v${ASW_VERSION}? [Y/n] "
            read -r response
            
            case "$response" in
                [nN][oO]|[nN])
                    print_info "Installation cancelled"
                    exit 0
                    ;;
            esac
        fi
        
        print_info "Upgrading existing installation..."
    fi
}

install_binary() {
    print_info "Installing binary to $INSTALL_DIR..."
    
    # Copy binary
    cp "${TMP_DIR}/asw" "${INSTALL_DIR}/asw"
    chmod 0755 "${INSTALL_DIR}/asw"
    
    print_success "Binary installed successfully"
}

install_completion() {
    SHELL_TYPE=$(detect_shell)
    print_info "Installing completion for $SHELL_TYPE..."
    
    COMPLETION_DIR="${HOME}/.agentswitch"
    mkdir -p "$COMPLETION_DIR"
    
    # Download completion scripts
    case "$SHELL_TYPE" in
        bash)
            # Generate completion
            if command -v asw >/dev/null 2>&1; then
                asw completion generate bash > "${COMPLETION_DIR}/completion.bash" 2>/dev/null || true
                
                # Add to .bashrc if not already present
                if [ "$NO_MODIFY_PATH" != "true" ]; then
                    if ! grep -q "agentswitch/completion.bash" "${HOME}/.bashrc" 2>/dev/null; then
                        echo ""
                        echo "# AgentSwitch bash completion" >> "${HOME}/.bashrc"
                        echo "source ${COMPLETION_DIR}/completion.bash 2>/dev/null || true" >> "${HOME}/.bashrc"
                        print_info "Added completion to ~/.bashrc"
                    fi
                fi
            fi
            ;;
        zsh)
            if command -v asw >/dev/null 2>&1; then
                asw completion generate zsh > "${COMPLETION_DIR}/completion.zsh" 2>/dev/null || true
                
                # Add to .zshrc if not already present
                if [ "$NO_MODIFY_PATH" != "true" ]; then
                    if ! grep -q "agentswitch/completion.zsh" "${HOME}/.zshrc" 2>/dev/null; then
                        echo "" >> "${HOME}/.zshrc"
                        echo "# AgentSwitch zsh completion" >> "${HOME}/.zshrc"
                        echo "fpath=(\"${COMPLETION_DIR}/completion.zsh\" \$fpath)" >> "${HOME}/.zshrc"
                        echo "autoload -U compinit && compinit" >> "${HOME}/.zshrc"
                        print_info "Added completion to ~/.zshrc"
                    fi
                fi
            fi
            ;;
        fish)
            if command -v asw >/dev/null 2>&1; then
                mkdir -p "${HOME}/.config/fish/completions"
                asw completion generate fish > "${HOME}/.config/fish/completions/asw.fish" 2>/dev/null || true
                print_info "Fish completion installed (will be auto-loaded)"
            fi
            ;;
    esac
    
    print_success "Shell completion configured"
}

# ============================================================================
# Uninstall Functions
# ============================================================================

uninstall() {
    print_info "Uninstalling AgentSwitch..."
    
    # Check if installed
    if [ ! -x "${INSTALL_DIR}/asw" ]; then
        print_warning "AgentSwitch is not installed"
        exit 0
    fi
    
    # Show what will be removed
    echo ""
    echo "This will remove:"
    echo "  - Binary: ${INSTALL_DIR}/asw"
    echo ""
    
    printf "Do you want to continue? [y/N] "
    read -r response
    
    case "$response" in
        [yY][eE][sS]|[yY])
            ;;
        *)
            print_info "Uninstall cancelled"
            exit 0
            ;;
    esac
    
    # Remove binary
    rm -f "${INSTALL_DIR}/asw"
    print_success "Binary removed"
    
    # Remove completion scripts
    rm -rf "${HOME}/.agentswitch"
    print_success "Completion scripts removed"
    
    # Ask about config directory
    echo ""
    printf "Do you want to remove your configuration directory (~/.agentswitch/)? [y/N] "
    read -r response
    
    case "$response" in
        [yY][eE][sS]|[yY])
            rm -rf "${HOME}/.agentswitch/config.toml"
            rm -rf "${HOME}/.agentswitch/presets"
            rm -rf "${HOME}/.agentswitch/backups"
            print_success "Configuration removed"
            ;;
        *)
            print_info "Configuration directory preserved"
            ;;
    esac
    
    echo ""
    print_success "Uninstall complete"
}

# ============================================================================
# Help Functions
# ============================================================================

show_help() {
    cat << EOF
AgentSwitch Installer v${VERSION}

USAGE:
    curl -sSL ${GITHUB_BASE_URL}/raw/main/scripts/install.sh | bash [OPTIONS]

OPTIONS:
    -h, --help          Show this help message
    -V, --version       Show installer version
    --uninstall         Uninstall AgentSwitch
    --force             Force installation without confirmation
    --dry-run           Show what would be installed without actually installing
    -y, --yes           Auto-confirm all prompts
    -v, --verbose       Show verbose output
    --install-dir DIR   Installation directory (default: /usr/local/bin)
    --local-file PATH   Install from local binary file instead of downloading

ENVIRONMENT VARIABLES:
    INSTALL_DIR         Installation directory (default: /usr/local/bin)
    NO_MODIFY_PATH      Don't modify PATH configuration (default: false)
    ASW_VERSION         Specific version to install (default: latest)

EXAMPLES:
    # Basic installation
    curl -sSL ${GITHUB_BASE_URL}/raw/main/scripts/install.sh | bash

    # Install to custom directory
    curl -sSL ${GITHUB_BASE_URL}/raw/main/scripts/install.sh | bash -s -- --install-dir ~/bin

    # Uninstall
    bash $0 --uninstall

For more information, visit: ${GITHUB_BASE_URL}
EOF
}

# ============================================================================
# Main Installation Flow
# ============================================================================

main() {
    print_header
    
    # Parse command line arguments
    while [ $# -gt 0 ]; do
        case $1 in
            -h|--help)
                show_help
                exit 0
                ;;
            -V|--version)
                echo "AgentSwitch Installer v${VERSION}"
                exit 0
                ;;
            --uninstall)
                uninstall
                exit 0
                ;;
            --force)
                FORCE_INSTALL=true
                shift
                ;;
            --dry-run)
                print_info "Dry run mode - showing installation plan"
                echo ""
                detect_system
                get_latest_version
                echo ""
                echo "Would install to: $INSTALL_DIR"
                exit 0
                ;;
            --install-dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --local-file)
                LOCAL_FILE="$2"
                shift 2
                ;;
            -y|--yes)
                # Auto-confirm prompts
                shift
                ;;
            -v|--verbose)
                set -x
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # Run installation steps
    detect_system
    get_latest_version
    check_install_dir
    check_existing_installation
    download_binary
    extract_binary
    install_binary
    install_completion
    
    # Success message
    echo ""
    print_success "Installation complete!"
    echo ""
    echo "AgentSwitch v${ASW_VERSION} has been installed successfully."
    echo ""
    echo "Binary: ${INSTALL_DIR}/asw"
    echo "Config: ~/.agentswitch/"
    echo ""
    echo "Quick start:"
    echo "  asw wizard init       # Start the initialization wizard"
    echo "  asw --help            # Show all commands"
    echo ""
    echo "Documentation:"
    echo "  man asw               # View manual page"
    echo "  ${GITHUB_BASE_URL}"
    echo ""
    
    # Remind about PATH
    case ":$PATH:" in
        *":${INSTALL_DIR}:"*)
            # Already in PATH
            ;;
        *)
            print_warning "Installation directory not in PATH: ${INSTALL_DIR}"
            echo ""
            echo "Add to your PATH:"
            case "$(detect_shell)" in
                bash)
                    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
                    echo "Then run: source ~/.bashrc"
                    ;;
                zsh)
                    echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
                    echo "Then run: source ~/.zshrc"
                    ;;
            esac
            echo ""
            ;;
    esac
}

# Run main function
main "$@"
