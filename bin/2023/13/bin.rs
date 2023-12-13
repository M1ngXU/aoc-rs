#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn solve() {
    let p = parser!((| (| ch)[LE])[LELE]);
    let s = pi!(p);

    let (p1, p2) = s
        .into_iter()
        .map(|field| {
            let mut valid = 0;
            for i in 0..field.len() - 1 {
                if (0..=i)
                    .rev()
                    .zip(i + 1..field.len())
                    .all(|(i1, i2)| field[i1] == field[i2])
                {
                    valid = 100 * (i + 1);
                    break;
                }
            }
            for i in 0..field[0].len() - 1 {
                if (0..=i)
                    .rev()
                    .zip(i + 1..field[0].len())
                    .all(|(i1, i2)| field.iter().all(|r| r[i1] == r[i2]))
                {
                    valid = i + 1;
                    break;
                }
            }
            for y in 0..field.len() {
                for x in 0..field[0].len() {
                    let mut fieldcopy = field.clone();
                    fieldcopy[y][x] = if fieldcopy[y][x] == '#' { '.' } else { '#' };
                    let mut nvalid = 0;
                    for i in 0..fieldcopy.len() - 1 {
                        if (0..=i)
                            .rev()
                            .zip(i + 1..fieldcopy.len())
                            .all(|(i1, i2)| fieldcopy[i1] == fieldcopy[i2])
                        {
                            nvalid = 100 * (i + 1);
                            if nvalid != valid {
                                break;
                            }
                        }
                    }
                    if nvalid == valid || nvalid == 0 {
                        for i in 0..fieldcopy[0].len() - 1 {
                            if (0..=i)
                                .rev()
                                .zip(i + 1..fieldcopy[0].len())
                                .all(|(i1, i2)| fieldcopy.iter().all(|r| r[i1] == r[i2]))
                            {
                                nvalid = i + 1;
                                if nvalid != valid {
                                    break;
                                }
                            }
                        }
                    }
                    if nvalid != valid && nvalid > 0 {
                        return (valid, nvalid);
                    }
                }
            }

            unreachable!()
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn main() {
    solve();
}
