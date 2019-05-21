#!/bin/bash

cd "/home/martina/prezentacia/performance_meetup/performance_meetup/"
SAN_PATH="src/example_san/";
LOG_PATH="$SAN_PATH/log_leak"
rm "$LOG_PATH"

rustup default nightly
cargo build
RUSTFLAGS='-Z sanitizer=leak' cargo run --target x86_64-unknown-linux-gnu --bin example_massif &> "$LOG_PATH"
rustup default stable
less -S "$LOG_PATH"
