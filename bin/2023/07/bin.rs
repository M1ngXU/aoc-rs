#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((~" " > ch ms1 << pn)[LE]);
    let s = pi!(p);
    s.into_iter()
        .map(|(h, b)| {
            let c = h.iter().copied().counts();
            let v = c.values().copied().collect_vec();
            let xx = if v.mx() > 3 {
                v.mx() + 1
            } else if v.mx() == 3 && v.mn() == 2 {
                4
            } else if v.mx() == 3 {
                3
            } else if v.mx() == 2 && v.iter().copied().filter(|x| *x == 2).count() == 2 {
                2
            } else if v.mx() == 2 {
                1
            } else {
                0
            };
            (
                h.into_iter()
                    .map(|c| {
                        [
                            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
                        ]
                        .into_iter()
                        .rev()
                        .position(|x| x == c)
                        .unwrap()
                    })
                    .collect_vec(),
                xx,
                b,
            )
        })
        .sorted_by(|(c, a, _), (c2, b, _)| a.cmp(b).then(c.cmp(c2)))
        .enumerate()
        .map(|(r, (_, _, b))| (r as isize + 1) * b)
        .s()
        .save();
}

fn two() {
    let p = parser!((~" " > ch ms1 << pn)[LE]);
    let s = pi!(p);
    s.into_iter()
        .map(|(h, b)| {
            let mut c = h.iter().copied().counts();
            let jokers = *c.get(&'J').unwrap_or(&0);
            c.remove(&'J');
            let v = c.values().copied().collect_vec();
            let xx = if v.iter().copied().max().unwrap_or(0) + jokers > 3 {
                v.iter().copied().max().unwrap_or(0) + 1 + jokers
            } else if v.iter().copied().max().unwrap_or(0) + jokers == 3
                && v.iter().copied().min().unwrap_or(0) == 2
            {
                4
            } else if v.iter().copied().max().unwrap_or(0) + jokers == 3 {
                3
            } else if v.iter().copied().max().unwrap_or(0) + jokers == 2
                && v.iter().copied().filter(|x| *x == 2).count() == 2
            {
                2
            } else if v.into_iter().max().unwrap_or(0) + jokers == 2 {
                1
            } else {
                0
            };
            (
                h.into_iter()
                    .map(|c| {
                        [
                            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
                        ]
                        .into_iter()
                        .rev()
                        .position(|x| x == c)
                        .unwrap()
                    })
                    .collect_vec(),
                xx,
                b,
            )
        })
        .sorted_by(|(c, a, _), (c2, b, _)| a.cmp(b).then(c.cmp(c2)))
        .enumerate()
        .map(|(r, (_, _, b))| (r as isize + 1) * b)
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
