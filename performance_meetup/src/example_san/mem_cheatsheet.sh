#!/bin/bash
SAN_PATH="src/example_san/";
LOG_PATH="$SAN_PATH/mem_leak"
rm "$LOG_PATH"

rustup default nightly
cargo build
RUSTFLAGS='-Z sanitizer=memory' cargo run --target x86_64-unknown-linux-gnu --bin example_massif &> "$LOG_PATH"
rustup default stable
less -S "$LOG_PATH"
