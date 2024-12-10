#![feature(iter_map_windows, iter_from_coroutine, never_type)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| id)[LE]);
    let s = pi!(p);
}

fn two() {}

#[cfg(not(feature = "benchmarking"))]
fn main() {
    one();
    two();
}

#[cfg(feature = "benchmarking")]
fn criterion_benchmark(c: &mut criterion::Criterion) {
    use std::hint::black_box;
    c.bench_function("one", |b| b.iter(|| black_box(one())));
    c.bench_function("two", |b| b.iter(|| black_box(two())));
}

#[cfg(feature = "benchmarking")]
criterion::criterion_group!(benches, criterion_benchmark);

#[cfg(feature = "benchmarking")]
criterion::criterion_main!(benches);
