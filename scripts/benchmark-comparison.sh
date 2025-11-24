#!/bin/bash
# Hyperfine comparison script - compare different binary configurations
# Useful for comparing performance with/without features

set -e

echo "Building binaries for comparison..."

# Build standard release
cargo build --release -p rust-hp-minitoring-cookie-cutter --bin testing
mv target/release/testing target/release/testing-standard

# Build with dhat (if needed for comparison)
# Note: dhat adds overhead, so this is mainly for demonstration
cargo build --release -p rust-hp-minitoring-cookie-cutter --bin testing --features dhat-heap
mv target/release/testing target/release/testing-dhat

echo "Running hyperfine comparison..."
hyperfine \
  --warmup 3 \
  --min-runs 10 \
  --export-json hyperfine-comparison.json \
  --export-markdown hyperfine-comparison.md \
  "target/release/testing-standard" \
  "target/release/testing-dhat"

echo "Comparison results saved to:"
echo "  - hyperfine-comparison.json"
echo "  - hyperfine-comparison.md"

