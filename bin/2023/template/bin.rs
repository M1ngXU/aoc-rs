#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = sble(id);
    let s = pi!(p);
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
