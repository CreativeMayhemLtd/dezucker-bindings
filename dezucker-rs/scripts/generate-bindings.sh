#!/bin/bash
set -e

# Ensure we are in the crate root
cd "$(dirname "$0")/.."

echo "Building dezucker Rust library and generating C headers..."

# Build in release mode
cargo build --release

# Check if header exists
if [ ! -f "dezucker.h" ]; then
    echo "Error: dezucker.h was not generated."
    exit 1
fi

echo "-------------------------------------------------------"
echo "Success! C bindings generated successfully."
echo ""
echo "Header location:  $(pwd)/dezucker.h"
echo "Library location: $(pwd)/target/release/"
echo ""
echo "Artifacts generated:"
if [ -f "target/release/libdezucker.so" ]; then echo " - libdezucker.so"; fi
if [ -f "target/release/libdezucker.dylib" ]; then echo " - libdezucker.dylib"; fi
if [ -f "target/release/libdezucker.a" ]; then echo " - libdezucker.a"; fi
echo "-------------------------------------------------------"
