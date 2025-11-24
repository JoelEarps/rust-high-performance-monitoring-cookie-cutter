#!/bin/bash
# Hyperfine benchmark script for the testing binary
# This benchmarks the entire binary execution time

set -e

# Build the binary first
echo "Building testing binary..."
cargo build --release -p rust-hp-minitoring-cookie-cutter --bin testing

# Run hyperfine benchmarks
echo "Running hyperfine benchmarks..."
hyperfine \
  --warmup 3 \
  --min-runs 10 \
  --max-runs 50 \
  --export-json hyperfine-results.json \
  --export-markdown hyperfine-results.md \
  "target/release/testing"

echo "Benchmark results saved to:"
echo "  - hyperfine-results.json"
echo "  - hyperfine-results.md"

