#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]
// #![feature(trace_macros)]
// trace_macros!(true);

use aoc_rs::prelude::*;

fn one() {
    let p2 = parser!((| ("Card" ms0) << pn ~"|" > pns (ms0 "|" ms0) <<| pns)[LE]);
    let s: Vec<(isize, (Vec<isize>, Vec<isize>))> = pi!(p2);
    s.into_iter()
        .map(|(_, (b, c))| (1 << c.into_iter().filter(|c| b.contains(&c)).count()) >> 1)
        .s()
        .save();
}

fn two() {
    let p2 = parser!((| ("Card" ms0) << pn ~"|" > pns (ms0 "|" ms0) <<| pns)[LE]);
    let s = pi!(p2);
    let mut cards = HashMap::<isize, isize>::new();
    for (i, (b, c)) in &s {
        let wc = c.iter().filter(|c| b.contains(c)).count() as isize;
        let decks = *cards.get(i).unwrap_or(&0) + 1;
        for i2 in 0..wc {
            *cards.entry(i2 + i + 1).or_insert(0) += decks;
        }
    }
    (cards.values().copied().s() + s.len() as isize).save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
