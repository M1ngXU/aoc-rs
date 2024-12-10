#![feature(iter_map_windows, iter_from_coroutine, never_type)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| pn >> ms1 pn)[LE]);
    let s = pi!(p);
    let (mut a, mut b) = (vec![], vec![]);
    for (c, d) in s {
        a.push(c);
        b.push(d);
    }
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b).map(|(a, b)| (a - b).abs()).s().save();
}

fn two() {
    let p = parser!((| pn >> ms1 pn)[LE]);
    let s = pi!(p);
    let (mut a, mut b) = (vec![], vec![]);
    for (c, d) in s {
        a.push(c);
        b.push(d);
    }
    let f = b.freq();
    a.iter().map(|a| a * f.get(a).unwrap_or(&0)).s().save();
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
