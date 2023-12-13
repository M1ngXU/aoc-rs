#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

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
        .map(|field| {
            let valid = findrref(&field)
                .map(|x| 100 * (x + 1))
                .next()
                .unwrap_or_else(|| findrref(&field.clone().t()).next().unwrap() + 1);
            (
                valid,
                field
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
                    .unwrap(),
            )
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn main() {
    solve();
}
