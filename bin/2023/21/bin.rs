#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use std::time::Instant;

use aoc_rs::prelude::*;

fn solve() {
    let s = parser!((| id)[LE]);
    // let s = parser!((| ~" " > id " " id)[LE]);
    let pi = pi!(s);
    let start = pi
        .iter()
        .enumerate()
        .find_map(|(i, r)| r.chars().position(|x| x == 'S').map(|x| (i, x)))
        .unwrap();
    let map = pi
        .into_iter()
        .map(|c| c.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    const C: usize = 26501365;
    assert_eq!(map[0].len(), map.len());
    assert_eq!(start.0, map.len() / 2);
    assert_eq!(start.0, start.1);
    let mut todo: HashMap<(isize, isize), u16> =
        HashMap::from([((start.0 as isize, start.1 as isize), 0b_000_010_000)]);
    let mut possible = vec![];
    let mut startt = Instant::now();
    for i in 1..=start.0 - 1 + map.len() {
        let mut new_todo: HashMap<(isize, isize), u16> = HashMap::new();
        for ((nextx, nexty), count) in todo {
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let nx = nextx as isize + dx;
                let ny = nexty as isize + dy;
                let blockx = if nx < 0 {
                    -1
                } else {
                    nx / map[0].len() as isize
                };
                let blocky = if ny < 0 { -1 } else { ny / map.len() as isize };
                let nx = (nx + map[0].len() as isize) % map[0].len() as isize;
                let ny = (ny + map.len() as isize) % map.len() as isize;
                if !map[ny as usize][nx as usize] {
                    let mut cur;
                    if blockx == -1 {
                        cur = 0;
                        if count & 0b_000_000_001 != 0 {
                            cur |= 0b_000_000_010;
                        }
                        if count & 0b_000_000_010 != 0 {
                            cur |= 0b_000_000_100;
                        }
                        if count & 0b_000_001_000 != 0 {
                            cur |= 0b_000_010_000;
                        }
                        if count & 0b_000_010_000 != 0 {
                            cur |= 0b_000_100_000;
                        }
                        if count & 0b_001_000_000 != 0 {
                            cur |= 0b_010_000_000;
                        }
                        if count & 0b_010_000_000 != 0 {
                            cur |= 0b_100_000_000;
                        }
                        if cur == 0 {
                            panic!("{count:b}");
                        }
                    } else if blockx == 1 {
                        cur = 0;
                        if count & 0b_100_000_000 != 0 {
                            cur |= 0b_010_000_000;
                        }
                        if count & 0b_010_000_000 != 0 {
                            cur |= 0b_001_000_000;
                        }
                        if count & 0b_000_100_000 != 0 {
                            cur |= 0b_000_010_000;
                        }
                        if count & 0b_000_010_000 != 0 {
                            cur |= 0b_000_001_000;
                        }
                        if count & 0b_000_000_100 != 0 {
                            cur |= 0b_000_000_010;
                        }
                        if count & 0b_000_000_010 != 0 {
                            cur |= 0b_000_000_001;
                        }
                        if cur == 0 {
                            panic!("{count:b}");
                        }
                    } else if blocky == 1 {
                        cur = 0;
                        if count & 0b_100_000_000 != 0 {
                            cur |= 0b_000_100_000;
                        }
                        if count & 0b_010_000_000 != 0 {
                            cur |= 0b_000_010_000;
                        }
                        if count & 0b_001_000_000 != 0 {
                            cur |= 0b_000_001_000;
                        }
                        if count & 0b_000_100_000 != 0 {
                            cur |= 0b_000_000_100;
                        }
                        if count & 0b_000_010_000 != 0 {
                            cur |= 0b_000_000_010;
                        }
                        if count & 0b_000_001_000 != 0 {
                            cur |= 0b_000_000_001;
                        }
                        if cur == 0 {
                            panic!("{count:b}");
                        }
                    } else if blocky == -1 {
                        cur = 0;
                        if count & 0b_000_000_001 != 0 {
                            cur |= 0b_000_001_000;
                        }
                        if count & 0b_000_000_010 != 0 {
                            cur |= 0b_000_010_000;
                        }
                        if count & 0b_000_000_100 != 0 {
                            cur |= 0b_000_100_000;
                        }
                        if count & 0b_000_001_000 != 0 {
                            cur |= 0b_001_000_000;
                        }
                        if count & 0b_000_010_000 != 0 {
                            cur |= 0b_010_000_000;
                        }
                        if count & 0b_000_100_000 != 0 {
                            cur |= 0b_100_000_000;
                        }
                        if cur == 0 {
                            panic!("{count:b}");
                        }
                    } else {
                        cur = count;
                    }
                    *new_todo.entry((nx, ny)).or_default() |= cur;
                }
            }
        }
        todo = new_todo;
        if i == 64 {
            let p1 = todo.iter().map(|(_, c)| c.count_ones() as usize).s();
            println!("Part 1: {}", p1);
        }
        if i == start.0 || (i + 1) % map.len() == start.0 {
            println!("{i}|{:?}", startt.elapsed());
            startt = Instant::now();
            possible.push(todo.iter().map(|(_, c)| c.count_ones() as usize).s());
        }
    }
    let c = possible[1];
    let a = (possible[2] - 2 * possible[0] + c) / 2;
    let b = a + c - possible[0];
    let x = (C - start.0) / map.len();
    println!("{possible:?}");
    println!("Part 2: {}", a * x * x + b * x + c);
}

fn main() {
    solve();
}
