#!/bin/bash

cd "/home/martina/prezentacia/performance_meetup/performance_meetup/"
MASSIF_PATH="src/example_massif";
OUT_PATH="$MASSIF_PATH/out_massif"
LOG_PATH="$MASSIF_PATH/log"
rm "$LOG_PATH"

#cargo build --release
cargo build
valgrind --tool=massif --threshold=0.0001   --massif-out-file="$OUT_PATH" ./target/debug/example_massif
#valgrind --tool=massif --threshold=0.0001   --massif-out-file="$OUT_PATH" ./target/release/example_massif

massif-visualizer "$OUT_PATH"
