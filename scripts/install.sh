#!/usr/bin/env bash
# Install script for statline (Linux)
#
# What this script does:
# 1. Builds statline in release mode
# 2. Installs the binary to ~/.local/bin when possible
# 3. Falls back to /usr/local/bin (uses sudo when required)
# 4. Checks whether the install directory is in PATH
# 5. Validates installation by running: statline --help

set -e

BIN_NAME="statline"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RELEASE_BIN="$PROJECT_ROOT/target/release/$BIN_NAME"
USER_INSTALL_DIR="$HOME/.local/bin"
SYSTEM_INSTALL_DIR="/usr/local/bin"
INSTALLED_PATH=""
PATH_OK=0

info() {
    printf '[INFO] %s\n' "$1"
}

success() {
    printf '[OK] %s\n' "$1"
}

warn() {
    printf '[WARN] %s\n' "$1"
}

fail() {
    printf '[ERROR] %s\n' "$1" >&2
    exit 1
}

# Step 1: Basic checks
info "Checking prerequisites..."
command -v cargo >/dev/null 2>&1 || fail "cargo is required but not found in PATH."

# Step 2: Build in release mode
info "Building $BIN_NAME in release mode..."
cd "$PROJECT_ROOT"
cargo build --release
[ -x "$RELEASE_BIN" ] || fail "Build completed but binary was not found at $RELEASE_BIN."
success "Release build complete."

# Step 3: Try installing to ~/.local/bin (preferred)
info "Attempting install to $USER_INSTALL_DIR..."
if mkdir -p "$USER_INSTALL_DIR" && cp "$RELEASE_BIN" "$USER_INSTALL_DIR/$BIN_NAME"; then
    INSTALLED_PATH="$USER_INSTALL_DIR/$BIN_NAME"
    success "Installed to $INSTALLED_PATH"
else
    warn "Could not install to $USER_INSTALL_DIR. Will try $SYSTEM_INSTALL_DIR."
fi

# Step 4: Fallback install to /usr/local/bin if needed
if [ -z "$INSTALLED_PATH" ]; then
    info "Attempting install to $SYSTEM_INSTALL_DIR..."

    if cp "$RELEASE_BIN" "$SYSTEM_INSTALL_DIR/$BIN_NAME" 2>/dev/null; then
        INSTALLED_PATH="$SYSTEM_INSTALL_DIR/$BIN_NAME"
        success "Installed to $INSTALLED_PATH"
    else
        command -v sudo >/dev/null 2>&1 || fail "Install to $SYSTEM_INSTALL_DIR requires elevated privileges, and sudo is not available."
        info "Using sudo to install to $SYSTEM_INSTALL_DIR..."
        sudo cp "$RELEASE_BIN" "$SYSTEM_INSTALL_DIR/$BIN_NAME" || fail "Failed to install to $SYSTEM_INSTALL_DIR with sudo."
        INSTALLED_PATH="$SYSTEM_INSTALL_DIR/$BIN_NAME"
        success "Installed to $INSTALLED_PATH"
    fi
fi

# Step 5: Check if install directory is in PATH
INSTALL_DIR="$(dirname "$INSTALLED_PATH")"
case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        PATH_OK=1
        success "$INSTALL_DIR is already in PATH."
        ;;
    *)
        PATH_OK=0
        warn "$INSTALL_DIR is not in PATH."
        warn "Add this line to your shell profile (for example ~/.bashrc):"
        printf '       export PATH="%s:$PATH"\n' "$INSTALL_DIR"
        ;;
esac

# Step 6: Validate installation using the installed binary path
info "Validating installation with '$BIN_NAME --help'..."
"$INSTALLED_PATH" --help >/dev/null 2>&1 || fail "Validation failed: '$INSTALLED_PATH --help' did not run successfully."
success "Validation passed."

# Step 7: Final status summary
printf '\n'
success "$BIN_NAME installation complete."
printf 'Installed binary: %s\n' "$INSTALLED_PATH"

if [ "$PATH_OK" -eq 1 ]; then
    success "You can run '$BIN_NAME --help' directly."
else
    warn "Current shell may not find '$BIN_NAME' until PATH is updated."
    printf "Temporary fix for this shell:\n"
    printf '    export PATH="%s:$PATH"\n' "$INSTALL_DIR"
fi
