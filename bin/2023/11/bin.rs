#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    const C: isize = 1_000_000;

    let p = parser!((| ch)[LE]);
    let s = pi!(p);
    let mut emptiesr = vec![0; s.len()];
    let mut xx: isize = 0;
    for (i, s) in s.iter().enumerate() {
        if s.iter().all(|c| c == &'.') {
            xx += C;
        } else {
            xx += 1;
        }
        emptiesr[i] = xx;
    }

    let mut emptiesc = vec![0; s.len()];
    let mut xx: isize = 0;
    for i in 0..s[0].len() {
        if s.iter().all(|c| c[i] == '.') {
            xx += C;
        } else {
            xx += 1;
        }
        emptiesc[i] = xx;
    }
    let fltten = s
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.clone()
                .into_iter()
                .enumerate()
                .map(|(x, c)| (y, x, c == '#'))
                .collect_vec()
        })
        .collect_vec();
    (fltten
        .iter()
        .copied()
        .filter(|(_, _, c)| *c)
        .permutations(2)
        .map(|v| {
            emptiesr[v[0].0].abs_diff(emptiesr[v[1].0])
                + emptiesc[v[0].1].abs_diff(emptiesc[v[1].1])
        })
        .s()
        / 2)
    .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
