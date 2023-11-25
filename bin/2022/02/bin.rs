use aoc_rs::prelude::*;

fn one() {
    let p = sb(
        LE,
        map(
            sb(
                " ",
                to_p(|s| match s {
                    "A" | "X" => 1, // r
                    "B" | "Y" => 2, // p
                    "C" | "Z" => 3, // s
                    _ => unreachable!(),
                }),
            ),
            |v| -> (_, _) { v.into_iter().collect_tuple().unwrap() },
        ),
    );
    let s = pi!(p);
    s.into_iter().map(|(e, s)| match s - e{1 | -2=>6,0=>3,_=>0}+s).s().save();
}

fn two() {
    let p = sb(
        LE,
        map(
            sb(
                " ",
                to_p(|s| match s {
                    "A" | "X" => 0, // r
                    "B" | "Y" => 1, // p
                    "C" | "Z" => 2, // s
                    _ => unreachable!(),
                }),
            ),
            |v| -> (_, _) { v.into_iter().collect_tuple().unwrap() },
        ),
    );
    let s = pi!(p);
    s.into_iter()
        .map(|(e, r)| {
            (
                match r {
                    2 => {
                        if e == 2 {
                            0
                        } else {
                            e + 1
                        }
                    }
                    1 => e,
                    _ => {
                        if e == 0 {
                            2
                        } else {
                            e - 1
                        }
                    }
                },
                3 * r,
            )
        })
        .map(|(s, r)| s + r + 1)
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
