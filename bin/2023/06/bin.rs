#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    // delete all spaces in input first! (for part 2)
    let p = parser!("Time:" << ms0 << (~"\n" pns) >> LE >> "Distance:" ms0 << pns);
    let (a, b) = pi!(p);
    a.into_iter()
        .zip(b)
        .map(|(t, d)| {
            (0..t)
                .par_bridge()
                .into_par_iter()
                .filter(|i| (t - i) * i > d)
                .count()
        })
        .collect_vec()
        .p()
        .save();
}

fn main() {
    print!("Part 1/2: ");
    one();
}
