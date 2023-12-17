#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn djkstra(dx: isize, dy: isize, s: &Vec<Vec<isize>>) -> isize {
    let h = s.len();
    let w = s[0].len();
    dijkstra(
        (0, (0, 0, 9, 3, dx, dy)),
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
        |_, _| 0,
        |_, (x, y, _, _, _, _)| *x as usize == w - 1 && *y as usize == h - 1,
    )
    .unwrap()
    .0
}

fn one() {
    let p = parser!((|pds)[LE]);
    let mut s = pi!(p);
    s[0][0] = 0;
    const PADDING: usize = 10;
    for _ in 0..PADDING + 1 {
        s.insert(0, vec![isize::MAX / 3; s[0].len()]);
    }
    for (y, row) in s.iter_mut().enumerate() {
        let fill = if y < PADDING + 2 { 0 } else { isize::MAX / 3 };
        if y < PADDING + 1 {
            row[0] = fill;
        }
        for _ in 0..PADDING + 1 {
            row.insert(0, fill);
        }
    }

    let h = s.len();
    let w = s[0].len();
    dijkstra(
        (0, (0, 0, 9, 3, 0, 1)),
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
        |_, _| 0,
        |_, (x, y, _, _, _, _)| *x as usize == w - 1 && *y as usize == h - 1,
    )
    .unwrap()
    .0
    .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
