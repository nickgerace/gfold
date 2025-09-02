#!/usr/bin/env bash
set -euo pipefail

# ======================================================================================================
# This file and format are inspired by Zed's platform setup.
# Link: https://github.com/zed-industries/zed/blob/024a5bbcd0f40dc7c9c762a207ef49964b0ec8b4/script/linux
# ======================================================================================================

if ! command -v cargo >/dev/null 2>&1; then
  echo "rust must be installed"
  exit 1
fi

sudo=''
if [ "$(id -u)" -ne 0 ]; then
  sudo="$(command -v sudo || command -v doas || true)"
fi

pacman=$(command -v pacman || true)
brew=$(command -v brew || true)

if [[ -n $pacman ]]; then
  deps=(
    cargo-bloat
    cargo-outdated
    cargo-udeps
    hyperfine
    just
    mold
    taplo-cli
  )
  "$sudo" "$pacman" -Sy --needed "${deps[@]}"
  exit 0
elif [[ -n $brew ]]; then
  deps=(
    cargo-bloat
    cargo-outdated
    cargo-udeps
    hyperfine
    just
    taplo
  )
  "$brew" install "${deps[@]}"
  exit 0
fi

echo "platform not yet supported"
exit 1
