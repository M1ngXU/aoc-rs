#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
    let p = sb(
        "\n",
        spair(
            spair(
                pcd(t!("Sensor at x="), mp(tu(","), pn)),
                t!(", y="),
                mp(tu(":"), pn),
            ),
            t!(": closest beacon is at x="),
            spair(mp(tu(","), pn), t!(", y="), pn),
        ),
    );
    let s = pi!(p);
    let manhatten_distances = s
        .iter()
        .map(|((x, y), (bx, by))| ((*x, *y), (x - bx).abs() + (y - by).abs()))
        .collect_vec();
    let yi = if s[0].0 .0 == 2 { 10 } else { 2_000_000 };
    let mut d = manhatten_distances
        .par_iter()
        .flat_map(|((x, y), d)| {
            if (*y..=y + d).contains(&yi) {
                x - (y + d - yi).abs()..=x + (y + d - yi).abs()
            } else if (y - d..=*y).contains(&yi) {
                x - (y - d - yi).abs()..=x + (y - d - yi).abs()
            } else {
                0..=-1
            }
        })
        .collect::<HashSet<isize>>();
    for (_, (x, y)) in &s {
        if y == &yi {
            d.remove(x);
        }
    }
    d.len().save();
}

fn two() {
    let p = sb(
        "\n",
        spair(
            spair(
                pcd(t!("Sensor at x="), mp(tu(","), pn)),
                t!(", y="),
                mp(tu(":"), pn),
            ),
            t!(": closest beacon is at x="),
            spair(mp(tu(","), pn), t!(", y="), pn),
        ),
    );
    let s = pi!(p);
    let manhatten_distances = s
        .iter()
        .map(|((x, y), (bx, by))| ((*x, *y), (x - bx).abs() + (y - by).abs()))
        .collect_vec();
    let mx = if s[0].0 .0 == 2 { 20 } else { 4_000_000 };
    let (y, x) = (0..=mx)
        .into_par_iter()
        .map(|yi| {
            (
                yi,
                manhatten_distances
                    .par_iter()
                    .map(|((x, y), d)| {
                        if (*y..=y + d).contains(&yi) {
                            vec![
                                0..=(x - (y + d - yi).abs()).max(0).min(mx) - 1,
                                (x + (y + d - yi).abs()).max(0).min(mx) + 1..=mx,
                            ]
                        } else if (y - d..=*y).contains(&yi) {
                            vec![
                                0..=(x - (y - d - yi).abs()).max(0).min(mx) - 1,
                                (x + (y - d - yi).abs()).max(0).min(mx) + 1..=mx,
                            ]
                        } else {
                            vec![0..=mx]
                        }
                    })
                    .reduce(
                        || vec![0..=mx],
                        |acc, r| {
                            acc.into_iter()
                                .cartesian_product(r)
                                .filter_map(|(a, b)| a.intersection(&b))
                                .filter(|a| !a.is_empty())
                                .collect_vec()
                        },
                    ),
            )
        })
        .find_any(|(_, x)| !x.is_empty())
        .unwrap();

    assert_eq!(x.len(), 1);
    assert_eq!(x[0].start(), x[0].end());
    (x[0].start() * 4000000 + y).save()
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
