#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn findrref(field: &Vec<Vec<char>>) -> impl Iterator<Item = usize> {
    field
        .i()
        .takei(-1)
        .filter(|i| {
            field[..=*i]
                .iter()
                .rev()
                .zip(&field[i + 1..])
                .all(|(f1, f2)| f1 == f2)
        })
        .collect_vec()
        .into_iter()
}

fn solve() {
    let p = parser!((| (| ch)[LE])[LELE]);
    let s = pi!(p);

    let (p1, p2) = s
        .into_iter()
        .zip_eq([
            14, 900, 1200, 1100, 9, 4, 900, 500, 200, 4, 6, 1, 8, 2, 9, 9, 6, 400, 1000, 1100, 400,
            14, 3, 100, 1500, 400, 2, 300, 10, 10, 400, 5, 500, 14, 2, 200, 500, 8, 4, 900, 2, 2,
            200, 2, 1, 7, 300, 3, 10, 9, 5, 100, 3, 1, 800, 400, 1400, 12, 400, 1, 100, 14, 200,
            100, 6, 7, 200, 6, 600, 1100, 5, 10, 8, 2, 4, 4, 11, 900, 1, 2, 2, 10, 700, 9, 5, 8,
            1100, 8, 2, 200, 5, 5, 500, 4, 800, 700, 4, 600, 6, 900,
        ])
        .map(|(field, e)| {
            let valid = findrref(&field)
                .map(|x| 100 * (x + 1))
                .next()
                .unwrap_or_else(|| findrref(&field.clone().t()).next().unwrap() + 1);
            let nvalid = field
                .ii()
                .find_map(|(x, y)| {
                    let mut fieldcopy = field.clone();
                    fieldcopy[y][x] = if fieldcopy[y][x] == '#' { '.' } else { '#' };
                    findrref(&fieldcopy)
                        .map(|x| 100 * (x + 1))
                        .filter(|s| s != &valid)
                        .next()
                        .or_else(|| {
                            findrref(&fieldcopy.clone().t())
                                .map(|x| x + 1)
                                .filter(|x| x != &valid)
                                .next()
                        })
                })
                .unwrap();
            if nvalid != e {
                println!("{nvalid}|{e}|{:?}", field);
            }
            (valid, nvalid)
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn main() {
    solve();
}
