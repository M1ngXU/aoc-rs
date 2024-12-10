#![feature(iter_map_windows, iter_from_coroutine, never_type)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| pns)[LE]);
    let s = pi!(p);
    s.into_iter()
        .filter(|v| {
            let d = v.d();
            d.iter().all(|d| [1, 2, 3].contains(d)) || d.iter().all(|d| [-1, -2, -3].contains(d))
        })
        .count()
        .save();
}

fn two() {
    let p = parser!((| pns)[LE]);
    let s = pi!(p);
    s.into_iter()
        .filter(|v| {
            for i in 0..v.len() {
                let mut vv = v.clone();
                vv.remove(i);
                let d = vv.d();
                if d.iter().all(|d| [1, 2, 3].contains(d))
                    || d.iter().all(|d| [-1, -2, -3].contains(d))
                {
                    return true;
                }
            }
            false
        })
        .count()
        .save();
}

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
