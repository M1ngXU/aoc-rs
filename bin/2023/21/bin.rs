#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

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
    let mut todo: HashMap<(isize, isize), u8> =
        HashMap::from([((start.0 as isize, start.1 as isize), 0b_000_0_0_000)]);
    let mut possible = vec![];
    for i in 1..=start.0 - 1 + map.len() {
        let mut new_todo: HashMap<(isize, isize), u8> = HashMap::new();
        for ((nextx, nexty), count) in todo {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx != dy && (dx == 0 || dy == 0) {
                        let nx = nextx as isize + dx;
                        let ny = nexty as isize + dy;
                        let blockx = nx.div_euclid(map[0].len() as isize);
                        let blocky = ny.div_euclid(map.len() as isize);
                        let nx = (nx % map[0].len() as isize + map[0].len() as isize)
                            % map[0].len() as isize;
                        let ny =
                            (ny % map.len() as isize + map.len() as isize) % map.len() as isize;
                        if !map[ny as usize][nx as usize] {
                            let mut cur = count;
                            if blockx == -1 {
                                if cur == 0 {
                                    cur |= 0b_000_1_0_000;
                                } else if cur & 0b_010_0_0_000 != 0 {
                                    cur |= 0b_100_0_0_000;
                                } else if cur & 0b_000_0_0_010 != 0 {
                                    cur |= 0b_000_0_0_100;
                                } else {
                                    panic!();
                                }
                            } else if blockx == 1 {
                                cur |= 0b0100;
                                // cur = cur.map(|x| x << 1);
                            } else if blocky == 1 {
                                cur |= 0b0010;
                                // cur.rotate_right(1); // rightmost is 0
                            } else if blocky == -1 {
                                cur |= 0b0001;
                                // cur.rotate_left(1); // leftmost is 0
                            }
                            let x = new_todo.entry((nx, ny)).or_default();
                            *x = x.zp(cur).map(|(a, b)| a | b);
                        }
                    }
                }
            }
        }
        todo = new_todo;
        if i == 64 {
            let p1 = todo.iter().map(|(_, c)| c.count_ones() as usize + 1).s();
            println!("Part 1: {}", p1);
        }
        if i == start.0 || (i + 1) % map.len() == start.0 {
            possible.push(todo.iter().map(|(_, c)| c.count_ones() as usize + 1).s());
        }
    }
    let c = possible[1];
    let a = (possible[2] - 2 * possible[0] + c) / 2;
    let b = a + c - possible[0];
    let x = (C - start.0) / map.len();
    println!("Part 2: {}", a * x * x + b * x + c);
}

fn main() {
    solve();
}
