#!/bin/bash

# Use direct cargo path to avoid rustup proxy issues
CARGO_BIN="$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo"

# Check if direct cargo exists, otherwise use system cargo
if [ ! -f "$CARGO_BIN" ]; then
    CARGO_BIN="cargo"
fi

echo "=========================================="
echo "Testing Parser with Valid Code"
echo "=========================================="
echo ""

# Check if the executable exists
if [ -f "target/debug/cc-compiler" ]; then
    echo "Running parser on comprehensive_valid.c..."
    ./target/debug/cc-compiler comprehensive_valid.c 2>&1 | tail -50
elif [ -f "target/release/cc-compiler" ]; then
    echo "Running parser on comprehensive_valid.c..."
    ./target/release/cc-compiler comprehensive_valid.c 2>&1 | tail -50
else
    echo "Building and running parser on comprehensive_valid.c..."
    $CARGO_BIN run -- comprehensive_valid.c 2>&1 | tail -50
fi

echo ""
echo "=========================================="
echo "Testing Parser with Buggy Code"
echo "=========================================="
echo ""

if [ -f "target/debug/cc-compiler" ]; then
    echo "Running parser on buggy_code.c..."
    ./target/debug/cc-compiler buggy_code.c 2>&1 | tail -100
elif [ -f "target/release/cc-compiler" ]; then
    echo "Running parser on buggy_code.c..."
    ./target/release/cc-compiler buggy_code.c 2>&1 | tail -100
else
    echo "Building and running parser on buggy_code.c..."
    $CARGO_BIN run -- buggy_code.c 2>&1 | tail -100
fi

