#![feature(iter_map_windows)]

use std::vec;

use aoc_rs::prelude::*;

fn one() {
    let p = sb(
        LE,
        alt((
            map(preceded(t!("addx "), pn), |n| (2, n)),
            map(t!("noop"), |_| (1, 0)),
        )),
    );
    let s = pi!(p);
    let mut strength = vec![0];
    let mut n = 1;
    for (c, x) in s {
        for _ in 0..c {
            strength.push(n);
        }
        n += x;
    }
    strength
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 40 == 20)
        .inspect(|(i, x)| println!("{}: {}", i, x))
        .map(|(i, x)| i as isize * x)
        .s()
        .save();
}

fn two() {
    let p = sb(
        LE,
        alt((
            map(preceded(tag("addx "), pn), |n| (2, n)),
            map(tag("noop"), |_| (1, 0)),
        )),
    );
    let s = pi!(p);
    let mut img = Vec::new();
    let mut n = 1;
    let mut t = 0;
    for (c, x) in s {
        for _ in 0..c {
            img.push(((t as isize) % 40 - n).abs() <= 1);
            t += 1;
        }
        n += x;
    }
    println!();
    img.db(40, 6);
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
