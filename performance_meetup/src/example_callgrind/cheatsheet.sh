#!/bin/bash

cd "/home/martina/prezentacia/performance_meetup/performance_meetup/"
CALLGRIND_PATH="src/example_callgrind";
OUT_PATH="$CALLGRIND_PATH/out_callgrind"
LOG_PATH="$CALLGRIND_PATH/log"
PROF_PATH="$CALLGRIND_PATH/prof"

rm "$OUT_PATH" "$LOG_PATH"

cargo build
valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --cache-sim=yes --branch-sim=yes --trace-children=yes --callgrind-out-file="$OUT_PATH"   ./target/release/example_callgrind 15
#valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --cache-sim=yes --branch-sim=yes --trace-children=yes --callgrind-out-file="$OUT_PATH"   ./target/release/example_cachegrind

cargo profiler callgrind --bin ./target/release/example_cachegrind  | tee "$PROF_PATH"

kcachegrind "$OUT_PATH"


