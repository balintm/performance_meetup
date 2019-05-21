#!/bin/bash

cd "/home/martina/prezentacia/performance_meetup/performance_meetup/"
PERF_PATH="src/example_perf";
sudo rm "perf.data*" "*.svg"

cargo build --release
sudo perf record -F 800 --call-graph dwarf  ./target/release/example_cachegrind
sudo perf script | /home/martina/work/FlameGraph/stackcollapse-perf.pl |  /home/martina/work/FlameGraph/flamegraph.pl > flame.svg

