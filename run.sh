#!/bin/bash
# Run the lexer project with system Rust
export PATH=/usr/bin:$PATH
cargo run -- "$@"
