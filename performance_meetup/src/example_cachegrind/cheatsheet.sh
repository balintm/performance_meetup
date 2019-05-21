#!/bin/bash

cd "/home/martina/prezentacia/performance_meetup/performance_meetup/"
CACHEGRIND_PATH="src/example_cachegrind";
OUT_PATH="$CACHEGRIND_PATH/out_cachegrind"
LOG_PATH="$CACHEGRIND_PATH/log"
rm "$LOG_PATH"

cargo build 
valgrind --tool=cachegrind --branch-sim=yes --trace-children=yes  --cachegrind-out-file="$OUT_PATH"   ./target/debug/example_cachegrind
kcachegrind "$OUT_PATH"
