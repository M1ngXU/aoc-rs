#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
    let p = sb(le, chp(u8));
    let s = pi!(p);
    let mut sum = 0;
    for y in 0..s.len() {
        for x in 0..s[0].len() {
            if s[y][..x].iter().all(|t| t < &s[y][x])
                || s[y][x + 1..].iter().all(|t| t < &s[y][x])
                || s[..y].iter().all(|r| r[x] < s[y][x])
                || s[y + 1..].iter().all(|r| r[x] < s[y][x])
            {
                sum += 1;
            }
        }
    }
    sum.save();
}

fn two() {
    let p = sb(
        le,
        map(ch, |c| {
            c.into_iter()
                .map(|c| u8::from_str_radix(&c.to_string(), 10).unwrap())
                .collect_vec()
        }),
    );
    let s = pi!(p);
    let mut sc = vec![];
    for y in 0..s.len() {
        for x in 0..s[0].len() {
            sc.push(
                s[y][..x]
                    .iter()
                    .rev()
                    .position(|t| t >= &s[y][x])
                    .map(|n| n + 1)
                    .unwrap_or(x)
                    * s[y][x + 1..]
                        .iter()
                        .position(|t| t >= &s[y][x])
                        .map(|n| n + 1)
                        .unwrap_or(s[0].len() - x - 1)
                    * s[..y]
                        .iter()
                        .rev()
                        .position(|r| r[x] >= s[y][x])
                        .map(|n| n + 1)
                        .unwrap_or(y)
                    * s[y + 1..]
                        .iter()
                        .position(|r| r[x] >= s[y][x])
                        .map(|n| n + 1)
                        .unwrap_or(s.len() - y - 1),
            );
        }
    }
    sc.into_iter().mx().save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
