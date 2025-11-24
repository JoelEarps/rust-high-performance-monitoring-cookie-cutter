# Rust High-Performance Monitoring Cookie Cutter

A Rust project template (cookie cutter) for high-performance monitoring and profiling. This workspace provides a ready-to-use setup for performance monitoring tools including **dhat** (heap profiler) and **tokio-console** (async runtime monitoring).

## What is This?

This is a Cargo workspace template designed to help you quickly set up performance monitoring in your Rust projects. It includes:

- **Library crate** with example code
- **Testing binary** configured for performance monitoring
- **Micro-benchmarking** with Criterion.rs
- **Binary benchmarking** with Hyperfine
- **Custom Cargo aliases** for easy command execution
- **Pre-configured tools** for memory and async runtime profiling
- **Documentation** on how to use the monitoring tools

## Project Structure

```bash

.
├── Cargo.toml                 # Workspace root configuration
├── .cargo/
│   └── config.toml            # Custom Cargo aliases
├── scripts/
│   ├── benchmark-binary.sh    # Hyperfine binary benchmarking script
│   └── benchmark-comparison.sh # Hyperfine comparison script
├── rust-hp-minitoring-cookie-cutter/
│   ├── Cargo.toml             # Package configuration
│   ├── src/
│   │   ├── lib.rs             # Library code
│   │   └── bin/
│   │       └── testing.rs     # Testing binary with monitoring support
│   ├── tests/
│   │   └── unit_test.rs       # Unit tests
│   ├── benches/
│   │   └── benchmark.rs       # Criterion benchmarks
│   └── docs/
│       └── performance-monitoring.md  # Detailed monitoring documentation
└── README.md                  # This file
```

## Features

### Performance Monitoring Tools

1. Tokio Console.
2. Dhat.
3. Hyperfine.
4. Criteron.

## Prerequisites

- Rust toolchain (latest stable recommended)
- For tokio-console: `cargo install tokio-console`
- For Hyperfine: `cargo install hyperfine` (or use Homebrew/apt)

  ```bash
  cargo install tokio-console
  ```

- For Hyperfine: Install the benchmarking tool:

  ```bash
  cargo install hyperfine
  # Or on macOS with Homebrew:
  brew install hyperfine
  # Or on Ubuntu/Debian:
  sudo apt install hyperfine
  ```

## Custom Cargo Commands

This project includes convenient Cargo aliases defined in `.cargo/config.toml`. All commands should be run from the workspace root.

### Run Commands

| Command | Description |
|---------|-------------|
| `cargo run-dhat` | Run the testing binary with dhat heap profiler enabled |
| `cargo run-tokio` | Run the testing binary (set `RUSTFLAGS="--cfg tokio_unstable"` for tokio console) |
| `cargo run-monitor` | Run with both dhat and tokio console (set `RUSTFLAGS="--cfg tokio_unstable"`) |

### Build Commands

| Command | Description |
|---------|-------------|
| `cargo build-dhat` | Build the testing binary with dhat heap profiler enabled |
| `cargo build-tokio` | Build the testing binary for tokio console support |

### Benchmark Commands

| Command | Description |
|---------|-------------|
| `cargo bench` | Run all benchmarks using Criterion.rs |

## Usage

### Running with dhat (Memory Profiling)

```bash

# Simple usage
cargo run-dhat

# The profiler will generate a report (dhat-heap.json) on exit
```

### Running with tokio-console (Async Monitoring)

```bash
# Terminal 1: Run the application
RUSTFLAGS="--cfg tokio_unstable" cargo run-tokio

# Terminal 2: Connect with tokio-console
tokio-console
```

### Running with Both Tools

```bash
# Terminal 1: Run with both monitoring tools
RUSTFLAGS="--cfg tokio_unstable" cargo run-monitor

# Terminal 2: Connect with tokio-console (optional)
tokio-console
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for the package
cargo test -p rust-hp-minitoring-cookie-cutter
```

### Building

```bash
# Standard build
cargo build

# Build with dhat support
cargo build-dhat

# Build with tokio console support
cargo build-tokio
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run a specific benchmark
cargo bench --bench benchmark

# View HTML reports (generated in target/criterion/)
# Open target/criterion/<benchmark_name>/report/index.html in a browser
```

Criterion generates detailed HTML reports with statistical analysis, graphs, and performance comparisons. Reports are saved in `target/criterion/`.

### Running Binary Benchmarks with Hyperfine

Hyperfine benchmarks entire binary execution time, making it perfect for end-to-end performance testing:

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

## Quick Start

1. **Clone or use this template** for your project
2. **Customize the code** in `rust-hp-minitoring-cookie-cutter/src/`
3. **Run with monitoring**:

   ```bash
   # For memory profiling
   cargo run-dhat
   
   # For async monitoring
   RUSTFLAGS="--cfg tokio_unstable" cargo run-tokio
   ```

## How It Works

### dhat Integration

The testing binary automatically initializes dhat when the `dhat-heap` feature is enabled:

- Uses a global allocator wrapper
- Tracks all heap allocations
- Generates reports on program exit

### tokio-console Integration

The testing binary initializes the console subscriber when `tokio_unstable` is enabled via `RUSTFLAGS`:

- Instruments async tasks
- Provides runtime metrics
- Connects to the tokio-console tool

## Documentation

For detailed information about the performance monitoring tools, see:

- [`rust-hp-minitoring-cookie-cutter/docs/performance-monitoring.md`](rust-hp-minitoring-cookie-cutter/docs/performance-monitoring.md)

## Customization

### Adding Your Own Code

1. Edit `rust-hp-minitoring-cookie-cutter/src/lib.rs` for library code
2. Edit `rust-hp-minitoring-cookie-cutter/src/bin/testing.rs` for the testing binary
3. Add tests in `rust-hp-minitoring-cookie-cutter/tests/`
4. Add benchmarks in `rust-hp-minitoring-cookie-cutter/benches/`

### Modifying Aliases

Edit `.cargo/config.toml` to add or modify custom commands.

## Example Workflow

```bash
# 1. Run micro-benchmarks to establish baseline
cargo bench

# 2. View benchmark reports in target/criterion/

# 3. Run binary benchmarks with Hyperfine
./scripts/benchmark-binary.sh

# 4. Build with dhat support
cargo build-dhat

# 5. Run with memory profiling
cargo run-dhat

# 6. Check the generated dhat-heap.json report

# 7. For async debugging, run with tokio console
RUSTFLAGS="--cfg tokio_unstable" cargo run-tokio

# 8. In another terminal, connect with tokio-console
tokio-console

# 9. After making changes, re-run benchmarks to detect regressions
cargo bench
./scripts/benchmark-binary.sh

# 10. Compare performance between configurations
./scripts/benchmark-comparison.sh
```

## License

This is a template project. Customize the license as needed for your use case.

## Contributing

This is a cookie cutter template. Feel free to fork and customize for your needs!
