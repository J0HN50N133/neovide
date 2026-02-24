#!/bin/bash
# Build Windows executable using cargo-zigbuild
# Usage: ./build-windows-zig.sh

set -e

echo "Installing cargo-zigbuild..."
cargo binstall -y cargo-zigbuild || cargo install cargo-zigbuild

echo "Building for Windows target..."
cargo zigbuild --target x86_64-pc-windows-gnu --release

echo "Output location: target/x86_64-pc-windows-gnu/release/neovide.exe"
