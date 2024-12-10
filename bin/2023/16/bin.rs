#![feature(
    
    
    vec_push_within_capacity,
    iter_map_windows,
    iter_from_coroutine
)]

use std::{mem::swap, time::Instant};

use aoc_rs::prelude::*;

#[inline(always)]
fn check_valid(s: &mut u8, dx: u8, dy: u8) -> bool {
    // // (0, 1): 0b0001 => 1
    // // (1, 0): 0b0CAPACITY => 4
    // // (1, 2): 0b0110 => 6
    // // (2, 1): 0bCAPACITY1 => 9
    const INDECES: [u8; 10] = [0xF, 0b0001, 0xF, 0xF, 0b0010, 0xF, 0b0100, 0xF, 0xF, 0b1000];
    let index = unsafe { INDECES.get_unchecked((dx << 2 | dy) as usize) };

    let not_seen = *s & index == 0;
    *s |= index * not_seen as u8;
    not_seen
}

fn one() {
    let p = parser!((|ch)[LE]);
    let mut s = pi!(p);
    for r in &mut s {
        r.insert(0, '?');
    }
    let width = s[2].len() as u8;
    s.insert(0, vec!['?'; width as usize]);
    let height = s.len() as u8;
    let s = s
        .into_iter()
        .flat_map(|r| {
            r.into_iter()
                .map(|c| match c {
                    '|' => 0,
                    '-' => 1,
                    '/' => 2,
                    '\\' => 3,
                    '.' => 4,
                    '?' => 255,
                    _ => unimplemented!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut total = 0;
    const RUNS: usize = 5_000;
    const WARMUP: usize = 5_000;
    for i in 0..WARMUP + RUNS {
        const THREADS: usize = 16;
        assert_eq!(THREADS, rayon::current_num_threads());
        const CAPACITY: usize = 100;
        let beams = Box::leak(Box::new([[(0, 0, 0, 0); CAPACITY]; THREADS])).as_mut_ptr() as usize;
        let new = Box::leak(Box::new([[(0, 0, 0, 0); CAPACITY]; THREADS])).as_mut_ptr() as usize;
        const WH: u8 = 111;
        assert_eq!(WH, width);
        assert_eq!(WH, height);
        assert_eq!(WH as usize * WH as usize, s.len());
        let seen =
            Box::leak(Box::new([[0; WH as usize * WH as usize]; THREADS])).as_mut_ptr() as usize;
        let start = Instant::now();
        let res = (1..WH)
            .map(|y| (y, 0, 2, 1)) // dx = 1, dy = 0
            .chain((1..WH).map(|y| (y, WH, 0, 1))) // dx = -1, dy = 0
            .chain((1..WH).map(|x| (0, x, 1, 2))) // dx = 0, dy = 1
            .chain((1..WH).map(|x| (WH, x, 1, 0))) // dx = 0, dy = -1
            .par_bridge()
            .into_par_iter()
            .map(|(y, x, dx, dy)| {
                let index = rayon::current_thread_index().unwrap();
                let mut beams =
                    unsafe { &mut *(beams as *mut [(u8, u8, u8, u8); CAPACITY]).add(index) };
                let mut beams_index = 1;
                beams[0] = (x, y, dx, dy);
                let mut new =
                    unsafe { &mut *(new as *mut [(u8, u8, u8, u8); CAPACITY]).add(index) };
                let mut new_index;
                let seen =
                    unsafe { &mut *(seen as *mut [u8; WH as usize * WH as usize]).add(index) };
                unsafe { seen.get_unchecked_mut(WH as usize + 1..WH as usize * WH as usize - 1) }
                    .fill(0);
                while beams_index > 0 {
                    new_index = 0;
                    for i in 0..beams_index {
                        let (bx, by, mut dx, mut dy) = unsafe { *beams.get_unchecked(i) };
                        let (nx, ny) = (bx + dx - 1, by + dy - 1);
                        if (1..WH).contains(&ny) && (1..WH).contains(&nx) {
                            let i = WH as usize * ny as usize + nx as usize;
                            let se = unsafe { seen.get_unchecked_mut(i) };
                            let s = *unsafe { s.get_unchecked(i) };
                            if s < 2 {
                                if s == 0 {
                                    if dy == 1 {
                                        if check_valid(se, 1, 0) {
                                            unsafe {
                                                *new.get_unchecked_mut(new_index) = (nx, ny, 1, 0)
                                            };
                                            new_index += 1;
                                        }
                                        if check_valid(se, 1, 2) {
                                            unsafe {
                                                *new.get_unchecked_mut(new_index) = (nx, ny, 1, 2);
                                            }
                                            new_index += 1;
                                        }
                                    } else {
                                        if check_valid(se, dx, dy) {
                                            unsafe {
                                                *new.get_unchecked_mut(new_index) =
                                                    (nx, ny, dx, dy);
                                            }
                                            new_index += 1;
                                        }
                                    }
                                } else {
                                    if dx == 1 {
                                        if check_valid(se, 0, 1) {
                                            unsafe {
                                                *new.get_unchecked_mut(new_index) = (nx, ny, 0, 1);
                                            }
                                            new_index += 1;
                                        }
                                        if check_valid(se, 2, 1) {
                                            unsafe {
                                                *new.get_unchecked_mut(new_index) = (nx, ny, 2, 1);
                                            }
                                            new_index += 1;
                                        }
                                    } else {
                                        if check_valid(se, dx, dy) {
                                            unsafe {
                                                *new.get_unchecked_mut(new_index) =
                                                    (nx, ny, dx, dy);
                                            }
                                            new_index += 1;
                                        }
                                    }
                                }
                            } else {
                                if s <= 3 {
                                    if s == 2 {
                                        dx = 2 - dx;
                                        dy = 2 - dy;
                                    }
                                    if check_valid(se, dy, dx) {
                                        unsafe {
                                            *new.get_unchecked_mut(new_index) = (nx, ny, dy, dx);
                                        }
                                        new_index += 1;
                                    }
                                } else {
                                    if check_valid(se, dx, dy) {
                                        unsafe {
                                            *new.get_unchecked_mut(new_index) = (nx, ny, dx, dy);
                                        }
                                        new_index += 1;
                                    }
                                }
                            }
                        }
                    }
                    swap(&mut beams, &mut new);
                    swap(&mut beams_index, &mut new_index);
                }

                let mut total = 0;
                for i in WH as usize + 1..WH as usize * WH as usize - 1 {
                    if unsafe { seen.get_unchecked_mut(i) } != &0 {
                        total += 1;
                    }
                }
                total
            })
            .max()
            .unwrap();
        let elapsed = start.elapsed();
        assert_eq!(res, 8564);
        if i > WARMUP {
            total += elapsed.as_micros();
        }
    }
    println!("{}us", total / RUNS as u128);
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
