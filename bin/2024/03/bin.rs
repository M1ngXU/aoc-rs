#![feature(iter_map_windows, iter_from_coroutine, never_type)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!(((~"mul" ((("(" << pn >> "," pn >> ")" >> id)?) >> id))["mul"]) >> "mul");
    let s = pi!((|s| format!("{s}mul")): p);
    s.into_iter().flatten().map(|(a, b)| a * b).s().save();
}

fn two() {
    let s = inp!("example.txt");
    let reg = r!(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))");
    let mut on = true;
    let mut cnt = 0;
    for x in reg.captures_iter(s) {
        if x.get(4).is_some_and(|x| x.as_str() == "do()") {
            on = true;
        } else if x.get(5).is_some_and(|x| x.as_str() == "don't()") {
            on = false;
        } else if on {
            let a = x.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let b = x.get(3).unwrap().as_str().parse::<i64>().unwrap();
            cnt += a * b;
        }
    }
    cnt.save();
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
