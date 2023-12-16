#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((|ch)[LE]);
    let s = pi!(p);
    (-1..=s.len() as isize)
        .cartesian_product(-1..=s[0].len() as isize)
        .filter(|(y, x)| {
            [
                x == &-1,
                y == &-1,
                x == &(s[0].len() as isize),
                y == &(s.len() as isize),
            ]
            .into_iter()
            .trues()
                == 1
        })
        .par_bridge()
        .into_par_iter()
        .map(|(y, x)| {
            let mut energized = s
                .iter()
                .map(|x| x.iter().map(|_| false).collect_vec())
                .collect_vec();
            let dx: isize = if x == -1 {
                1
            } else if x == s[0].len() as isize {
                -1
            } else {
                0
            };
            let dy: isize = if y == -1 {
                1
            } else if y == s.len() as isize {
                -1
            } else {
                0
            };
            let mut beams = vec![(x as isize, y as isize, dx, dy)];
            let mut seen = HashSet::new();
            while !beams.is_empty() {
                let mut new = vec![];
                for (bx, by, dx, dy) in beams {
                    let (nx, ny) = (bx + dx, by + dy);
                    if (0..energized.len() as isize).contains(&ny)
                        && (0..energized[0].len() as isize).contains(&nx)
                    {
                        energized[ny as usize][nx as usize] = true;
                        match s[ny as usize][nx as usize] {
                            '|' if dy == 0 => {
                                new.push((nx, ny, 0, -1));
                                new.push((nx, ny, 0, 1));
                            }
                            '-' if dx == 0 => {
                                new.push((nx, ny, -1, 0));
                                new.push((nx, ny, 1, 0));
                            }
                            '/' => {
                                new.push((nx, ny, -dy, -dx));
                            }
                            '\\' => {
                                new.push((nx, ny, dy, dx));
                            }
                            _ => {
                                new.push((nx, ny, dx, dy));
                            }
                        }
                    }
                }
                beams = new.into_iter().filter(|&x| seen.insert(x)).collect();
            }
            energized.into_iter().flatten().trues()
        })
        .max()
        .unwrap()
        .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
