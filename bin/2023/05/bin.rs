#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!("seeds: " << (pn[" "]) LELE << (((~ "\n" le) << (((pn " " << pn " " <<| pn))[le]))[LELE]));
    let s = pi!(p);

    let (seeds, other) = s;

    let location = seeds
        .into_iter()
        .tuple_windows()
        .enumerate()
        .par_bridge()
        .into_par_iter()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, (a, b))| (a, b))
        .flat_map(|(seedf, _seedt)| seedf..seedf + 1)
        .map(|mut pos| {
            for other in &other {
                for (t, (f, r)) in other.iter() {
                    if (*f..f + r).contains(&pos) {
                        pos += t - f;
                        break;
                    }
                }
            }
            pos
        })
        .min()
        .unwrap();
    location.save();
}

fn two() {
    let p = parser!("seeds: " << (pn[" "]) LELE << (((~ "\n" le) << (((pn " " << pn " " <<| pn))[le]))[LELE]));
    let s = pi!(p);

    let (seeds, other) = s;

    let location = seeds
        .into_iter()
        .tuple_windows()
        .enumerate()
        .par_bridge()
        .into_par_iter()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, (a, b))| (a, b))
        .flat_map(|(seedf, seedt)| seedf..seedf + seedt)
        .map(|mut pos| {
            for other in &other {
                for (t, (f, r)) in other.iter() {
                    if (*f..f + r).contains(&pos) {
                        pos += t - f;
                        break;
                    }
                }
            }
            pos
        })
        .min()
        .unwrap();
    location.save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
