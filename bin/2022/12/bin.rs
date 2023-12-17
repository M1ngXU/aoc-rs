#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn replace(c: char) -> i8 {
    ((match c {
        'S' => 'a',
        'E' => 'z',
        c => c,
    }) as u8
        - b'a') as i8
}

fn one() {
    let p = _sb("\n", ch);
    let s = pi!(p);
    let start_y = s.iter().position(|r| r.contains(&'S')).unwrap();
    let start_x = s[start_y].iter().position(|&c| c == 'S').unwrap();
    dijkstrao(
        (0u32, (start_x, start_y)),
        |c, (x, y)| {
            let mut adj = vec![];
            if x > &0 && replace(s[*y][x - 1]) - replace(s[*y][*x]) <= 1 {
                adj.push((c + 1, (x - 1, *y)));
            }
            if *x < s[0].len() - 1 && replace(s[*y][x + 1]) - replace(s[*y][*x]) <= 1 {
                adj.push((c + 1, (x + 1, *y)));
            }
            if y > &0 && replace(s[y - 1][*x]) - replace(s[*y][*x]) <= 1 {
                adj.push((c + 1, (*x, y - 1)));
            }
            if *y < s.len() - 1 && replace(s[y + 1][*x]) - replace(s[*y][*x]) <= 1 {
                adj.push((c + 1, (*x, y + 1)));
            }

            adj
        },
        |_, (x, y)| s[*y][*x] == 'E',
    )
    .unwrap()
    .1
    .len()
    .save();
}

fn two() {
    let p = _sb("\n", ch);
    let s = pi!(p);
    let mut min = usize::MAX;
    for y in 0..s.len() {
        for x in 0..s[0].len() {
            if replace(s[y][x]) != replace('a') {
                continue;
            }
            min = min.min(
                dijkstrao(
                    (0u32, (x, y)),
                    |c, (x, y)| {
                        let mut adj = vec![];
                        if x > &0 && replace(s[*y][x - 1]) - replace(s[*y][*x]) <= 1 {
                            adj.push((c + 1, (x - 1, *y)));
                        }
                        if *x < s[0].len() - 1 && replace(s[*y][x + 1]) - replace(s[*y][*x]) <= 1 {
                            adj.push((c + 1, (x + 1, *y)));
                        }
                        if y > &0 && replace(s[y - 1][*x]) - replace(s[*y][*x]) <= 1 {
                            adj.push((c + 1, (*x, y - 1)));
                        }
                        if *y < s.len() - 1 && replace(s[y + 1][*x]) - replace(s[*y][*x]) <= 1 {
                            adj.push((c + 1, (*x, y + 1)));
                        }

                        adj
                    },
                    |_, (x, y)| s[*y][*x] == 'E',
                )
                .map(|(_, v)| v.len())
                .unwrap_or(usize::MAX),
            );
        }
    }
    min.save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
