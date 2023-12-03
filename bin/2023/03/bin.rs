#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

const INPUT: &str = include_str!("input.txt");

fn one() {
    INPUT
        .lines()
        .map(|s| {
            s.char_indices()
                .fold(
                    (vec![], false),
                    |(mut blocks, num): (Vec<Vec<(usize, char)>>, bool), n| {
                        if n.1.is_numeric() && num {
                            blocks.last_mut().unwrap().push(n);
                        } else {
                            blocks.push(vec![n]);
                        }
                        (blocks, n.1.is_numeric())
                    },
                )
                .0
        })
        .enumerate()
        .map(|(y, lines)| {
            lines
                .iter()
                .filter(|b| {
                    !b.is_empty()
                        && b[0].1.is_numeric()
                        && b.iter().any(|(x, _)| {
                            adjd(
                                *x,
                                y,
                                INPUT.lines().next().unwrap().len(),
                                INPUT.lines().count(),
                            )
                            .any(|(x, y)| {
                                INPUT.lines().nth(y).unwrap().chars().nth(x).unwrap() != '.'
                                    && !INPUT
                                        .lines()
                                        .nth(y)
                                        .unwrap()
                                        .chars()
                                        .nth(x)
                                        .unwrap()
                                        .is_numeric()
                            })
                        })
                })
                .map(|b| {
                    b.iter()
                        .map(|(_, c)| *c)
                        .collect::<String>()
                        .parse::<isize>()
                        .unwrap()
                })
                .s()
        })
        .s()
        .save();
}

fn two() {
    let p = sble(id);
    let s = pi!(p);
    let chs = s.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let s = s
        .into_iter()
        .map(|s| {
            let mut c = s.char_indices();
            let mut blocks: Vec<Vec<(usize, char)>> = vec![];
            let mut num = false;
            while let Some(n) = c.next() {
                if n.1.is_numeric() && num {
                    blocks.last_mut().unwrap().push(n);
                } else {
                    blocks.push(vec![n]);
                    num = n.1.is_numeric();
                }
            }
            blocks
        })
        .collect_vec();
    let mut counter = 0;
    for (y, lines) in s.iter().enumerate() {
        'outer: for block in lines.iter().filter(|b| !b.is_empty()) {
            let mut gear = 1;
            for (_, c) in block {
                if *c != '*' {
                    continue 'outer;
                }
            }
            let x = block.first().unwrap().0;

            let mut parts = HashSet::new();
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let y = y as isize + dy;
                    let x = x as isize + dx;
                    if (0..chs.len()).contains(&(y as usize))
                        && (0..chs[0].len()).contains(&(x as usize))
                    {
                        if chs[y as usize][x as usize].is_numeric() {
                            let g = s[y as usize]
                                .iter()
                                .find(|b| b.contains(&(x as usize, chs[y as usize][x as usize])))
                                .unwrap();
                            if parts.insert(g) {
                                gear *= g
                                    .iter()
                                    .map(|(_, c)| format!("{c}"))
                                    .collect::<String>()
                                    .parse::<isize>()
                                    .unwrap();
                            }
                        }
                    }
                }
            }
            if parts.len() > 1 {
                counter += gear;
            }
        }
    }
    counter.save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
