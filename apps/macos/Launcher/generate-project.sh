#!/bin/zsh
# Sets up env vars for xcodegen. Keeps version in sync with Cargo.

set -e
cd "$(dirname -- "$0")"

BUILD=$(git rev-list --count master)
VERSION=$(grep ^version ../Cargo.toml | cut -d " " -f3 | cut -d \" -f2)

export BUILD VERSION
xcodegen

if [ -z "$CI" ]; then
  echo "Remember to regenerate the project when changing version in Cargo.toml."
fi
