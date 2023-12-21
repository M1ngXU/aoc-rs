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
    let mut todo: HashMap<(isize, isize), [u8; 5]> =
        HashMap::from([((start.0 as isize, start.1 as isize), [0, 0, 0b00100, 0, 0])]);
    let mut possible = vec![];
    for i in 1..=usize::MAX {
        let mut new_todo: HashMap<(isize, isize), [u8; 5]> = HashMap::new();
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
                                cur = cur.map(|x| x >> 1);
                            } else if blockx == 1 {
                                cur = cur.map(|x| x << 1);
                            }
                            if blocky == 1 {
                                cur[4] = 0;
                                cur.rotate_right(1);
                            } else if blocky == -1 {
                                cur[0] = 0;
                                cur.rotate_left(1);
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
            println!(
                "Part 1: {}",
                todo.iter()
                    .map(|(_, c)| c.into_iter().map(|x| x.count_ones() as usize).s())
                    .s()
            );
        }
        if i % map.len() == start.0 {
            possible.push(
                todo.iter()
                    .map(|(_, c)| c.into_iter().map(|x| x.count_ones() as usize).s())
                    .s(),
            );
            if possible.len() == 3 {
                break;
            }
        }
    }
    // c = possible[0]
    // ax^2 + bx + c
    // a + b + c = possible[1]
    // a = possible[1] - b - c
    // 4a + 2b + c = possible[2]
    // 2b = possible[2] - 4a - c
    // 2b = possible[2] - 4(possible[1] - b - c) - c
    // 2b = possible[2] - 4possible[1] + 4b + 4c - c
    // 2b = possible[2] - 4possible[1] + 4b + 3c
    // b = (possible[2] - 4possible[1] + 4b + 3c) / 2
    // b = (possible[2] - 4possible[1] + 3c) / 2 + 2b
    // b = -(possible[2] - 4possible[1] + 3c) / 2
    // b = (-possible[2] + 4possible[1] - 3c) / 2
    // b = (4possible[1] - possible[2] - 3c) / 2
    // a = possible[1] - b - c
    let c = possible[0];
    let b = (4 * possible[1] - possible[2] - 3 * c) / 2;
    let a = possible[1] - b - c;
    let x = (C - start.0) / map.len();
    (a * x * x + b * x + c).save();
}

fn main() {
    solve();
}
