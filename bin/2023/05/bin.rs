#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!("seeds: " << (pn[" "]) LELE << ((| ((~"\n" > id) >> le) << ((| (pn >> " " pn >> " " pn))[LE]))[LELE]));
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
    let p = parser!("seeds: " << (pn[" "]) LELE << ((| ((~"\n" > id) >> le) << ((| (pn >> " " pn >> " " pn))[LE]))[LELE]));
    let s = pi!(p);

    let (seeds, other): (Vec<isize>, Vec<Vec<(isize, (isize, isize))>>) = s;
    let other = other
        .into_iter()
        .map(|o| {
            o.into_iter()
                .map(|(to, (from, size))| {
                    (HypercuboidSet::new(vec![[from..from + size]]), to - from)
                })
                .collect_vec()
        })
        .collect_vec();

    let mut locations = seeds
        .chunks_exact(2)
        .map(|c| HypercuboidSet::new(vec![[c[0]..c[0] + c[1]]]))
        .collect_vec();
    for o in other {
        for location in &mut locations {
            let mut new_location = HypercuboidSet::default();
            for (map, to) in &o {
                let mut tmp = &*location & map;
                *location -= &tmp;
                for r in &mut tmp.ranges {
                    r[0] = r[0].starts_at() + to..=r[0].ends_at() + to;
                }
                new_location += tmp;
            }
            *location += new_location;
        }
    }
    locations
        .into_iter()
        .flat_map(|r| r.ranges.into_iter().map(|r| r[0].starts_at()))
        .mn()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
