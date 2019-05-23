#!/bin/bash

PERF_PATH="src/example_perf";
sudo rm "perf.data*" "*.svg"

cargo build --release
sudo perf record -F 800 --call-graph dwarf  ./target/release/example_cachegrind
sudo perf script | ~/FlameGraph/stackcollapse-perf.pl |  ~/FlameGraph/flamegraph.pl > flame.svg

