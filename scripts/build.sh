#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: ./scripts/build.sh [--clean]

Options:
  --clean    Run cargo clean before building
  -h, --help Show this help message
USAGE
}

CLEAN=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --clean)
      CLEAN=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Error: Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

BIN_NAME="$({
  awk '
    /^\[package\]/ { in_pkg=1; next }
    /^\[/ && in_pkg { exit }
    in_pkg && $1 == "name" {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' Cargo.toml
} || true)"

if [[ -z "$BIN_NAME" ]]; then
  echo "Error: Could not determine package name from Cargo.toml" >&2
  exit 1
fi

if [[ "$CLEAN" == true ]]; then
  echo "==> Running cargo clean"
  cargo clean
fi

echo "==> Building release binary"
START_TIME=$(date +%s)
cargo build --release
END_TIME=$(date +%s)

BUILD_SECONDS=$((END_TIME - START_TIME))
BINARY_PATH="$PROJECT_ROOT/target/release/$BIN_NAME"

if [[ ! -f "$BINARY_PATH" ]]; then
  echo "Warning: Expected binary not found at $BINARY_PATH" >&2
  echo "Searching for executable files in target/release..." >&2
  mapfile -t CANDIDATES < <(find "$PROJECT_ROOT/target/release" -maxdepth 1 -type f -perm -u+x)

  if [[ ${#CANDIDATES[@]} -eq 0 ]]; then
    echo "Error: No executable binary found in target/release" >&2
    exit 1
  fi

  BINARY_PATH="${CANDIDATES[0]}"
fi

if stat -c%s "$BINARY_PATH" >/dev/null 2>&1; then
  BINARY_SIZE_BYTES=$(stat -c%s "$BINARY_PATH")
else
  BINARY_SIZE_BYTES=$(stat -f%z "$BINARY_PATH")
fi

BINARY_SIZE_HUMAN=$(du -h "$BINARY_PATH" | awk '{print $1}')

echo
echo "Build completed successfully"
echo "Binary path: $BINARY_PATH"
echo "Binary size: $BINARY_SIZE_HUMAN (${BINARY_SIZE_BYTES} bytes)"
echo "Build time: ${BUILD_SECONDS}s"
