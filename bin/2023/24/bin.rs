#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;
use std::ops::RangeInclusive;

fn one() {
    let p = parser!((| pns)[LE]);
    let s = pi!(p);
    #[cfg(feature = "ex")]
    const RANGE: RangeInclusive<f64> = 7.0..=27.0;
    #[cfg(not(feature = "ex"))]
    const RANGE: RangeInclusive<f64> = 200_000_000_000_000.0..=400_000_000_000_000.0;
    s.clone()
        .into_iter()
        .enumerate()
        .flat_map(|(i, v1)| {
            s.clone()
                .into_iter()
                .take(i)
                .map(move |v2| (v1.clone(), v2))
        })
        .par_bridge()
        .map(|(v1, v2)| {
            let v1 = v1.clone().into_iter().cf64().collect_vec();
            let v2 = v2.into_iter().cf64().collect_vec();
            let (x1, y1, _, dx1, dy1, _) = (v1[0], v1[1], v1[2], v1[3], v1[4], v1[5]);
            let (x2, y2, _, dx2, dy2, _) = (v2[0], v2[1], v2[2], v2[3], v2[4], v2[5]);
            let lhs = Matrix2::new(dx2, -dx1, dy2, -dy1);
            let rhs = Vector2::new(x1 - x2, y1 - y2);
            if let Some(sol) = lhs.full_piv_lu().solve(&rhs) {
                let t2 = sol[0];
                let t1 = sol[1];
                let (x, y) = (x1 + dx1 * t1, y1 + dy1 * t1);
                if t1 >= 0.0 && t2 >= 0.0 && RANGE.contains(&x) && RANGE.contains(&y) {
                    return true;
                }
            }
            false
        })
        .filter(|x| *x)
        .count()
        .save();
}

fn two() {
    let p = parser!((| pns)[LE]);
    let s = pi!(p);
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
