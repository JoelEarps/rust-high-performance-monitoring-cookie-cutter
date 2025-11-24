//! Testing binary for performance monitoring with dhat and tokio console support
//!
//! Usage:
//!   - With dhat: cargo run --bin testing --features dhat-heap
//!   - With tokio console: RUSTFLAGS="--cfg tokio_unstable" cargo run --bin testing
//!   - With both: RUSTFLAGS="--cfg tokio_unstable" cargo run --bin testing --features dhat-heap

use rust_hp_minitoring_cookie_cutter::add;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::main]
async fn main() {
    // Initialize tokio console subscriber if tokio_unstable is enabled
    #[cfg(tokio_unstable)]
    {
        console_subscriber::init();
        println!("✓ Tokio console subscriber initialized");
    }

    // Initialize dhat heap profiler if dhat-heap feature is enabled
    #[cfg(feature = "dhat-heap")]
    let _profiler = {
        let profiler = dhat::Profiler::new_heap();
        println!("✓ Dhat heap profiler initialized");
        profiler
    };

    println!("Running performance monitoring test...");

    // Example async work
    let handle = tokio::spawn(async {
        let result = add(10, 20);
        println!("Calculation result: {}", result);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        result
    });

    let result = handle.await.unwrap();
    println!("Test completed with result: {}", result);

    // Give time for console subscriber to collect data
    #[cfg(tokio_unstable)]
    {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    #[cfg(feature = "dhat-heap")]
    {
        println!("Dhat profiler will generate report on exit");
    }
}
