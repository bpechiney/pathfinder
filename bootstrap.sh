#!/bin/bash

set -o errexit
set -o nounset
set -o

readonly BREW_URL="https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
readonly RUSTUP_URL="https://sh.rustup.rs"

function main() {
  install_rustup
  install_just
}

install_just() {
  if ! command -v just &> /dev/null; then
    log_info "just is not installed. Installing just using Homebrew..."
    brew install just
  else
    log_info "just is already installed."
  fi
}

install_rustup() {
  if ! command -v brew &> /dev/null; then
    log_error "brew is not installed."
    exit 1
  fi

  if ! command -v rustup &> /dev/null; then
    log_info "rustup is not installed. Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf "$RUSTUP_URL" | sh
  else
    log_info "rustup is already installed."
  fi
}

log_error() {
  local RED='\033[0;31m'
  local NC='\033[0m' # No Color
  echo -e "${RED}[ERROR] $1${NC}"
}

# Function to log informational messages in the default text color
log_info() {
  echo -e "[INFO] $1"
}

main "$@"
