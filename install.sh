#!/usr/bin/env bash
set -e

# Colors for terminal output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

APP_NAME="myvault"
REPO_URL="https://github.com/madonnaLeonardoDev/myvault.git"

# --- SMART PATH DETECTION ---
FIND_INSTALL_DIR() {
    if [[ ":$PATH:" == *":${HOME}/.local/bin:"* ]]; then
        echo "${HOME}/.local/bin"
        return
    fi

    IFS=':' read -ra PATH_DIRS <<< "$PATH"
    for dir in "${PATH_DIRS[@]}"; do
        if [ -n "$dir" ] && [ -d "$dir" ] && [ -w "$dir" ]; then
            echo "$dir"
            return
        fi
    done

    echo "${HOME}/.local/bin"
}

INSTALL_DIR="$(FIND_INSTALL_DIR)"
# ----------------------------

echo -e "${BLUE}==>${NC} Starting installation for ${APP_NAME}..."

# 1. Verify Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[!] Cargo is not installed. Please install Rust (https://rustup.rs) first.${NC}"
    exit 1
fi

# 2. Create a temporary build directory
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

echo -e "${BLUE}==>${NC} Downloading source code from GitHub..."
git clone "${REPO_URL}" "${TMP_DIR}/repo"

# 3. Locate Cargo.toml and Compile
echo -e "${BLUE}==>${NC} Compiling ${APP_NAME} (Release Mode)..."
cd "${TMP_DIR}/repo"

if [ ! -f "Cargo.toml" ]; then
    if [ -f "${APP_NAME}/Cargo.toml" ]; then
        cd "${APP_NAME}"
    else
        echo -e "${RED}[!] Error: Could not find Cargo.toml anywhere in the repository!${NC}"
        exit 1
    fi
fi

# Build without --quiet to display full output
cargo build --release

# 4. Install the binary dynamically
echo -e "${BLUE}==>${NC} Installing binary to ${INSTALL_DIR}..."
mkdir -p "${INSTALL_DIR}"
cp "target/release/${APP_NAME}" "${INSTALL_DIR}/${APP_NAME}"
chmod +x "${INSTALL_DIR}/${APP_NAME}"

# 5. Generate and install shell completions
USER_SHELL="$(basename "${SHELL}")"
echo -e "${BLUE}==>${NC} Detected shell: ${USER_SHELL}"

WARN_ZSH_FPATH=false
COMP_DIR_USED=""

case "${USER_SHELL}" in
    zsh)
        echo -e "${BLUE}==>${NC} Scanning Zsh fpath for writable directories..."
        COMP_DIR=""
        
        while IFS= read -r dir; do
            if [ -n "$dir" ] && [ -d "$dir" ] && [ -w "$dir" ]; then
                COMP_DIR="$dir"
                continue
            fi
        done < <(zsh -c 'print -l $fpath' 2>/dev/null || true)

        if [ -z "$COMP_DIR" ]; then
            COMP_DIR="${HOME}/.local/share/zsh/site-functions"
            mkdir -p "${COMP_DIR}"
            WARN_ZSH_FPATH=true
        fi
        
        COMP_DIR_USED="$COMP_DIR"
        echo -e "${BLUE}==>${NC} Generating Zsh completions -> ${COMP_DIR}/_${APP_NAME}"
        "${INSTALL_DIR}/${APP_NAME}" completions zsh > "${COMP_DIR}/_${APP_NAME}"
        
        rm -f "${HOME}/.zcompdump*" 2>/dev/null || true
        ;;
    bash)
        COMP_DIR="${HOME}/.local/share/bash-completion/completions"
        mkdir -p "${COMP_DIR}"
        echo -e "${BLUE}==>${NC} Generating Bash completions -> ${COMP_DIR}/${APP_NAME}"
        "${INSTALL_DIR}/${APP_NAME}" completions bash > "${COMP_DIR}/${APP_NAME}"
        ;;
    fish)
        COMP_DIR="${HOME}/.config/fish/completions"
        mkdir -p "${COMP_DIR}"
        echo -e "${BLUE}==>${NC} Generating Fish completions -> ${COMP_DIR}/${APP_NAME}.fish"
        "${INSTALL_DIR}/${APP_NAME}" completions fish > "${COMP_DIR}/${APP_NAME}.fish"
        ;;
    *)
        echo -e "${YELLOW}[!] Shell '${USER_SHELL}' not explicitly supported for auto-completions.${NC}"
        ;;
esac

echo -e "${GREEN}==> Installation Complete!${NC}"

# 6. Final Path & Completion Checks
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: ${INSTALL_DIR} is not in your \$PATH.${NC}"
    echo "Add this line to your ~/.${USER_SHELL}rc file:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
else
    echo -e "${GREEN}Success! ${APP_NAME} was installed to ${INSTALL_DIR}.${NC}"
fi

if [ "$WARN_ZSH_FPATH" = true ]; then
    echo -e "${YELLOW}Warning: Auto-completions were saved to ${COMP_DIR_USED}${NC}"
    echo "This directory is likely not in your Zsh \$fpath because standard system folders (/usr/...) require root privileges."
    echo "To enable completions, add the following line to your ~/.zshrc BEFORE the 'compinit' command:"
    echo "  fpath=(\"${COMP_DIR_USED}\" \$fpath)"
else
    echo "Restart your terminal or run: exec ${USER_SHELL}"
fi