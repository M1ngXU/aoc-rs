#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use std::time::Instant;

use aoc_rs::prelude::*;

#[inline(always)]
fn check_valid(s: &mut u8, d: u8) -> bool {
    // (0, 1): 0b0001 => 1
    // (1, 0): 0b0100 => 4
    // (1, 2): 0b0110 => 6
    // (2, 1): 0b1001 => 9
    const INDECES: [u8; 10] = [0xF, 0b0001, 0xF, 0xF, 0x0010, 0xF, 0b0100, 0xF, 0xF, 0b1000];
    let index = INDECES[d as usize];

    if *s & index == 0 {
        *s |= index;
        true
    } else {
        false
    }
}

fn one() {
    let p = parser!((|ch)[LE]);
    let s = pi!(p);
    let start = Instant::now();
    (1..s.len() as isize)
        .map(|y| (y, -1, 0b1001_u8)) // dx = 1, dy = 0
        .chain((1..s.len() as isize).map(|y| (y, s[0].len() as isize, 0b0001))) // dx = -1, dy = 0
        .chain((1..s[0].len() as isize).map(|x| (-1, x, 0b0110))) // dx = 0, dy = 1
        .chain((1..s[0].len() as isize).map(|x| (s.len() as isize, x, 0b0100))) // dx = 0, dy = -1
        .par_bridge()
        .into_par_iter()
        .map(|(y, x, d)| {
            // let mut energized = vec![vec![false; s[0].len()]; s.len()];
            let mut beams = vec![(x, y, d)];
            let mut seen = vec![vec![0; s[0].len()]; s.len()];
            while !beams.is_empty() {
                let mut new = vec![];
                for (bx, by, d) in beams {
                    let (nx, ny) = (bx + (d >> 2) as isize - 1, by + (d & 0b11) as isize - 1);
                    if (0..s.len() as isize).contains(&ny) && (0..s[0].len() as isize).contains(&nx)
                    {
                        let se = &mut seen[ny as usize][nx as usize];
                        match s[ny as usize][nx as usize] {
                            '|' if d & 0b11 == 0b01 => {
                                if check_valid(se, 0b0100) {
                                    new.push((nx, ny, 0b0100));
                                }
                                if check_valid(se, 0b0110) {
                                    new.push((nx, ny, 0b0110));
                                }
                            }
                            '-' if d & 0b1100 == 0b0100 => {
                                if check_valid(se, 0b0100) {
                                    new.push((nx, ny, 0b0001));
                                }
                                if check_valid(se, 0b0110) {
                                    new.push((nx, ny, 0b1001));
                                }
                            }
                            '/' => {
                                let d = 0b1010 - (((d << 2) & 0b1100) | (d >> 2));
                                if check_valid(se, d) {
                                    new.push((nx, ny, d));
                                }
                            }
                            '\\' => {
                                let d = ((d << 2) & 0b1100) | (d >> 2);
                                if check_valid(se, d) {
                                    new.push((nx, ny, d));
                                }
                            }
                            _ => {
                                if check_valid(se, d) {
                                    new.push((nx, ny, d));
                                }
                            }
                        }
                    }
                }
                beams = new;
            }

            seen.into_iter().flatten().filter(|x| x != &0).count()
        })
        .max()
        .unwrap()
        .save();
    println!("{:?}", start.elapsed());
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
