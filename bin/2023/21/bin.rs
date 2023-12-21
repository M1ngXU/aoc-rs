#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use std::time::Instant;

use aoc_rs::prelude::*;

fn solve() {
    let s = parser!((| id)[LE]);
    let pi = pi!(s);
    let start = pi
        .iter()
        .enumerate()
        .find_map(|(i, r)| r.chars().position(|x| x == 'S').map(|x| (i, x)))
        .unwrap();
    let map = pi
        .clone()
        .into_iter()
        .enumerate()
        .map(|(y, c)| {
            c.char_indices()
                .map(|(x, c)| (if c == '#' { 99999 } else { 1 }, (x as isize, y as isize)))
                .collect_vec()
        })
        .collect_vec();
    let d = dijkstraa2(&map, (start.0 as isize, start.1 as isize), 0);
    let ([d1, d0, d22], d21) = rayon::join(
        || [64, 65, 195].map(|x| d.iter().filter(|(_, &c)| c % 2 == x % 2 && c <= x).count()),
        || {
            [0, 130]
                .into_iter()
                .cartesian_product([0, 130])
                .map(|(x, y)| ((x, y), 63))
                .chain(
                    [(0, start.1), (130, start.1), (start.0, 0), (start.1, 130)]
                        .map(|(x, y)| ((x as isize, y as isize), 129)),
                )
                .collect_vec()
                .into_par_iter()
                .map(|((x, y), max)| {
                    dijkstraa2(&map, (x, y), 0)
                        .into_iter()
                        .filter(|(_, c)| c % 2 == max % 2 && c <= &max)
                        .count()
                })
                .sum::<usize>()
        },
    );
    println!("Part 1: {d0}");

    let c = d0;
    let a = (d21 + d22 - 2 * d1 + c) / 2;
    let b = a + c - d1;
    let x = (26501365 - start.0) / map.len();
    println!("Part 2: {}", a * x * x + b * x + c);
}

fn main() {
    solve();
}
