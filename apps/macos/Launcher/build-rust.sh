#!/bin/zsh
# Script that is called from within Xcode to compile Rust sources.
# Adapted from https://chinedufn.github.io/swift-bridge/building/xcode-and-cargo/index.html

set -e

if [[ -n "$PROJECT_DIR" ]]; then
  cd "$PROJECT_DIR"
else
  cd "$(dirname -- "$0")"
fi

# Without this we can't compile on MacOS Big Sur
# https://github.com/TimNN/cargo-lipo/issues/41#issuecomment-774793892
if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

TARGETS="aarch64-apple-darwin,x86_64-apple-darwin"

# if [ $ENABLE_PREVIEWS == "NO" ]; then
  echo "[Rust] Building using cargo-lipo... ($TARGETS)"
  if [ "$CONFIGURATION" = "Debug" ]; then
    cargo lipo --package launcher-macos --targets $TARGETS --manifest-path ../Cargo.toml
  else
    cargo lipo --package launcher-macos --targets $TARGETS --release --manifest-path ../Cargo.toml
  fi
# else
#   echo "[Rust] Skipping the script because of preview mode"
# fi
