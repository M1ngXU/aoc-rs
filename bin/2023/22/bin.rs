#![feature(iter_map_windows, iter_from_coroutine)]

use std::{ops::RangeInclusive, time::Instant};

use aoc_rs::prelude::*;

// DOESNT WORK ANYMORE

fn fall_fast(
    cubes: BTreeMap<usize, Vec<[RangeInclusive<isize>; 3]>>,
) -> (BTreeMap<usize, Vec<[RangeInclusive<isize>; 3]>>, usize) {
    let mut fell = 0;
    let mut safe: BTreeMap<usize, Vec<[RangeInclusive<isize>; 3]>> = Default::default();
    for (s, cubes) in cubes {
        for mut n in cubes {
            let z = safe
                .iter()
                .rev()
                .find_map(|c| {
                    c.1.iter()
                        .find(|c| c[0].intersects(&n[0]) && c[1].intersects(&n[1]))
                        .map(|h| h[2].ends_at() as usize + 1)
                })
                .unwrap_or(0);
            if s != z {
                n[2] = z as isize..=z as isize + n[2].ends_at() - n[2].starts_at();
                fell += 1;
            }
            safe.entry(z).or_default().push(n);
        }
    }
    (safe, fell)
}

fn solve() {
    let p = parser!((| ~"~" > pns >> "~" pns)[LE]);
    let s = pi!(p);

    let mut cubes = vec![];
    for (start, end) in s {
        let (sx, sy, sz) = (start[0], start[1], start[2]);
        let (ex, ey, ez) = (end[0], end[1], end[2]);
        cubes.push([sx..=ex, sy..=ey, sz..=ez]);
    }
    let c = cubes.len();
    let cubes = cubes
        .into_iter()
        .into_group_map_by(|c| *c[2].start() as usize)
        .into_iter()
        .collect();

    let start = Instant::now();
    let cubes = fall_fast(cubes).0;

    let prefixes: Vec<usize> = cubes.iter().fold(vec![0], |mut a, (i, v)| {
        if a.len() < *i + 1 {
            a.extend(vec![a.l(); i + 1 - a.len()]);
        }
        a.push(v.len() + a.l());
        a
    });
    let (p1, p2) = (0..c)
        .par_bridge()
        .into_par_iter()
        .map(|i| {
            let mut cc = cubes.clone();
            let j = prefixes.iter().position(|&x| x > i).unwrap() - 1;
            let p = prefixes[j];
            cc.get_mut(&j).unwrap().remove(i - p);
            let f = fall_fast(cc).1;
            if f == 0 {
                (1, 0)
            } else {
                (0, f)
            }
        })
        .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d));
    let elapsed = start.elapsed();
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{elapsed:?}");
}

fn main() {
    solve();
}
