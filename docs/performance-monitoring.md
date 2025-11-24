# Performance Monitoring Guide

This document describes the performance monitoring tools and setup available in this project.

## Overview

This project is configured with multiple performance monitoring and benchmarking tools:

1. **dhat** - Heap profiler for tracking memory allocations
2. **tokio-console** - Runtime monitoring for async Tokio applications
3. **Criterion.rs** - Statistical micro-benchmarking framework
4. **Hyperfine** - Command-line benchmarking tool for binary performance testing

## Tools

### dhat

[dhat](https://github.com/nnethercote/dhat-rs) is a heap profiler that helps identify memory allocations and potential memory leaks in Rust applications.

#### Features

- Tracks all heap allocations
- Provides detailed reports on memory usage
- Helps identify memory leaks and excessive allocations
- Generates HTML reports for analysis

#### Usage

To run the testing binary with dhat:

```bash
# Using custom cargo alias (recommended)
cargo run-dhat

# Or using cargo feature directly from workspace root
cargo run -p rust-hp-minitoring-cookie-cutter --bin testing --features dhat-heap
```

The profiler will generate a report file (typically `dhat-heap.json`) that can be analyzed.

#### Configuration

dhat is configured in `rust-hp-minitoring-cookie-cutter/src/bin/testing.rs` with:

- Global allocator wrapper
- Profiler initialization
- Automatic report generation on program exit

### Tokio Console

[Tokio Console](https://github.com/tokio-rs/console) is a debugging and monitoring tool for async Rust applications using Tokio.

#### Features

- Real-time monitoring of async tasks
- Task scheduling visualization
- Resource usage tracking
- Performance bottleneck identification
- Interactive debugging interface

#### Usage

To run the testing binary with tokio console support:

```bash
# Using custom cargo alias with RUSTFLAGS
RUSTFLAGS="--cfg tokio_unstable" cargo run-tokio

# Or using the full command from workspace root
RUSTFLAGS="--cfg tokio_unstable" cargo run -p rust-hp-minitoring-cookie-cutter --bin testing
```

Then, in a separate terminal, run the tokio console:

```bash
tokio-console
```

The console will connect to your running application and display real-time metrics.

#### Prerequisites

Install tokio-console tool:

```bash
cargo install tokio-console
```

#### Configuration

Tokio console is configured using:

- `console-subscriber` crate for runtime instrumentation
- Tokio unstable features enabled via `RUSTFLAGS`
- Automatic initialization in the testing binary

### Criterion.rs

[Criterion.rs](https://github.com/bheisler/criterion.rs) is a statistical micro-benchmarking framework for Rust that provides precise performance measurements with HTML reports.

#### Features

- Precise performance measurements with statistical analysis
- HTML reports with graphs and visualizations
- Automatic detection of performance regressions
- Statistical significance testing
- Comparison between benchmark runs
- Beautiful, interactive HTML reports

#### Usage

To run benchmarks using Criterion:

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench --bench benchmark

# View HTML reports (generated in target/criterion/)
# Open target/criterion/<benchmark_name>/report/index.html in a browser
```

Criterion generates detailed HTML reports with statistical analysis, graphs, and performance comparisons. Reports are saved in `target/criterion/`.

#### Configuration

Criterion is configured in `rust-hp-minitoring-cookie-cutter/benches/benchmark.rs`:

- Uses `criterion_group!` and `criterion_main!` macros
- Benchmarks are defined using `c.bench_function()`
- Uses `black_box` to prevent compiler optimizations
- Automatically handles warmup runs and statistical analysis

#### Example Benchmark

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_hp_minitoring_cookie_cutter::add;

fn benchmark_add(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| add(black_box(10), black_box(20))));
}

criterion_group!(benches, benchmark_add);
criterion_main!(benches);
```

### Hyperfine

[Hyperfine](https://github.com/sharkdp/hyperfine) is a command-line benchmarking tool that benchmarks entire binary execution time, making it perfect for end-to-end performance testing.

#### Features

- Benchmarks entire binary execution time
- Statistical analysis with confidence intervals
- Automatic warmup runs
- Comparison between different commands/configurations
- Export results to JSON and Markdown formats
- Perfect for end-to-end performance testing

#### Prerequisites

Install Hyperfine:

```bash
# Using cargo
cargo install hyperfine

# Or on macOS with Homebrew:
brew install hyperfine

# Or on Ubuntu/Debian:
sudo apt install hyperfine
```

#### Usage

Hyperfine benchmarks entire binary execution time:

```bash
# Using the provided script (recommended)
./scripts/benchmark-binary.sh

# Or manually:
# 1. Build release binary
cargo build --release -p rust-hp-minitoring-cookie-cutter --bin testing

# 2. Run hyperfine
hyperfine --warmup 3 --min-runs 10 "target/release/testing"

# Compare different configurations
./scripts/benchmark-comparison.sh

# Export results
hyperfine --export-json results.json --export-markdown results.md "target/release/testing"
```

#### Configuration

Hyperfine scripts are provided in the `scripts/` directory:

- `benchmark-binary.sh` - Simple binary benchmarking
- `benchmark-comparison.sh` - Compare different binary configurations

### Using Both Tools Together

You can use both dhat and tokio console simultaneously:

```bash
# Using custom cargo alias (recommended)
RUSTFLAGS="--cfg tokio_unstable" cargo run-monitor

# Or using the full command from workspace root
RUSTFLAGS="--cfg tokio_unstable" cargo run -p rust-hp-minitoring-cookie-cutter --bin testing --features dhat-heap
```

In one terminal, run the application. In another terminal, run `tokio-console` to monitor async tasks while dhat tracks memory allocations.

### When to Use Criterion vs Hyperfine

- **Criterion**: For micro-benchmarking specific functions/code paths within your Rust code
  - Best for: Function-level performance, algorithm comparisons, code path optimization
  - Provides: Detailed statistical analysis, HTML reports, regression detection
  
- **Hyperfine**: For benchmarking entire binary execution time and comparing different builds/configurations
  - Best for: End-to-end performance, binary comparison, real-world execution time
  - Provides: Command-line statistics, comparison tables, export formats

## Custom Cargo Aliases

This project includes convenient cargo aliases for running the testing binary with different monitoring configurations:

### Available Aliases

- **`cargo run-dhat`** - Run the testing binary with dhat heap profiler enabled
- **`cargo run-tokio`** - Run the testing binary (set `RUSTFLAGS="--cfg tokio_unstable"` for tokio console)
- **`cargo run-monitor`** - Run the testing binary with dhat enabled (set `RUSTFLAGS="--cfg tokio_unstable"` for tokio console)
- **`cargo build-dhat`** - Build the testing binary with dhat heap profiler enabled
- **`cargo build-tokio`** - Build the testing binary

### Quick Reference

```bash
# Run with dhat only
cargo run-dhat

# Run with tokio console only
RUSTFLAGS="--cfg tokio_unstable" cargo run-tokio

# Run with both dhat and tokio console
RUSTFLAGS="--cfg tokio_unstable" cargo run-monitor
```

## Testing Binary

The `testing` binary (`src/bin/testing.rs`) is configured to support both monitoring tools:

- Automatically initializes dhat when the `dhat-heap` feature is enabled
- Automatically initializes tokio console subscriber when `tokio_unstable` is enabled
- Provides example async work to demonstrate monitoring capabilities
- Can be extended with your own test scenarios

## Best Practices

1. **Memory Profiling**: Use dhat during development to identify memory issues early
2. **Async Debugging**: Use tokio console when debugging complex async code
3. **Performance Testing**: Run both tools together for comprehensive performance analysis
4. **CI/CD**: Consider running performance tests in CI to catch regressions

## Benchmarking Commands

### Cargo Benchmarks

| Command | Description |
|---------|-------------|
| `cargo bench` | Run all benchmarks using Criterion.rs |

### Hyperfine Scripts

| Script | Description |
|--------|-------------|
| `./scripts/benchmark-binary.sh` | Run Hyperfine benchmark on the testing binary |
| `./scripts/benchmark-comparison.sh` | Compare different binary configurations with Hyperfine |

## Additional Resources

- [dhat-rs Documentation](https://docs.rs/dhat/)
- [Tokio Console Documentation](https://github.com/tokio-rs/console)
- [Criterion.rs Documentation](https://github.com/bheisler/criterion.rs)
- [Hyperfine Documentation](https://github.com/sharkdp/hyperfine)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
