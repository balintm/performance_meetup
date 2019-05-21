#!/bin/bash

cd "/home/martina/prezentacia/performance_meetup/performance_meetup/"
MEMCHECK_PATH="src/example_memcheck";
OUT_PATH="$MEMCHECK_PATH/xtree_memcheck"
LOG_PATH="$MEMCHECK_PATH/log"
rm "$OUT_PATH" "$LOG_PATH"

cargo run --bin example_memcheck
valgrind --leak-check=full --xtree-memory=full --log-file="$LOG_PATH" --xtree-memory-file="$OUT_PATH"   ./target/debug/example_memcheck
kcachegrind "$OUT_PATH"
