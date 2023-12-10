#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| pns)[LE]);
    let s = pi!(p);
    s.iter()
        .cloned()
        .map(|row| {
            let mut rows = successors(Some(row), |pred| Some(pred.d()))
                .take_while(|r| r.iter().any(|x| x != &0))
                .collect_vec();
            rows.last_mut().unwrap().push(0);
            for r in (0..rows.len() - 1).rev() {
                let x = rows[r].l() + rows[r + 1].l();
                rows[r].push(x);
            }
            rows[0].l()
        })
        .s()
        .save();
}

fn two() {
    let p = parser!((| pns)[LE]);
    let s = pi!(p);
    s.iter()
        .cloned()
        .map(|row| {
            let mut rows = successors(Some(row), |pred| Some(pred.d()))
                .take_while(|r| r.iter().any(|x| x != &0))
                .collect_vec();
            rows.last_mut().unwrap().insert(0, 0);
            for r in (0..rows.len() - 1).rev() {
                let x = rows[r][0] - rows[r + 1][0];
                rows[r].insert(0, x);
            }
            rows[0][0]
        })
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
