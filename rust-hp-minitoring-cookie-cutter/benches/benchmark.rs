use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_hp_minitoring_cookie_cutter::add;

fn benchmark_add(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| add(black_box(10), black_box(20))));
}

fn benchmark_add_small_numbers(c: &mut Criterion) {
    c.bench_function("add_small", |b| b.iter(|| add(black_box(1), black_box(2))));
}

fn benchmark_add_large_numbers(c: &mut Criterion) {
    c.bench_function("add_large", |b| {
        b.iter(|| add(black_box(u64::MAX - 100), black_box(50)))
    });
}

criterion_group!(
    benches,
    benchmark_add,
    benchmark_add_small_numbers,
    benchmark_add_large_numbers
);
criterion_main!(benches);
