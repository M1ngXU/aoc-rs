#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((|pds)[LE]);
    let s = pi!(p);

    let h = s.len();
    let w = s[0].len();
    dijkstra(
        vec![(0, (0, 0, 2, 0, 1)), (0, (0, 0, 2, 1, 0))],
        |cost, (x, y, max, dx, dy)| {
            let l90 = (-dy, -dx);
            let r90 = (*dy, *dx);
            let mut adj = vec![];
            for ((dx, dy), max) in [l90, r90]
                .map(|x| (x, 2))
                .into_iter()
                .chain(if max > &0 {
                    vec![((*dx, *dy), max - 1)]
                } else {
                    vec![]
                })
                .collect_vec()
            {
                let nx = x + dx;
                let ny = y + dy;
                if s.ibi(nx, ny) {
                    adj.push((cost + s[ny as usize][nx as usize], (nx, ny, max, dx, dy)));
                }
            }
            adj
        },
        |_, (x, y, _, _, _)| *x as usize == w - 1 && *y as usize == h - 1,
    )
    .unwrap()
    .0
    .save();
}

fn two() {
    let p = parser!((|pds)[LE]);
    let s = pi!(p);

    let h = s.len();
    let w = s[0].len();
    dijkstra(
        vec![(0, (0, 0, 9, 3, 0, 1)), (0, (0, 0, 9, 3, 1, 0))],
        |cost, (x, y, max, min, dx, dy)| {
            let l90 = (-dy, -dx);
            let r90 = (*dy, *dx);
            let mut adj = vec![];
            for ((dx, dy), max, min) in if min > &0 {
                vec![((*dx, *dy), max - 1, min - 1)]
            } else {
                [l90, r90]
                    .map(|x| (x, 9, 3))
                    .into_iter()
                    .chain(if max > &0 {
                        vec![((*dx, *dy), max - 1, 0)]
                    } else {
                        vec![]
                    })
                    .collect_vec()
            } {
                let nx = x + dx;
                let ny = y + dy;
                if s.ibi(nx, ny) {
                    adj.push((
                        cost + s[ny as usize][nx as usize],
                        (nx, ny, max, min, dx, dy),
                    ));
                }
            }
            adj
        },
        |_, (x, y, _, _, min, _)| min == &0 && *x as usize == w - 1 && *y as usize == h - 1,
    )
    .unwrap()
    .0
    .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
