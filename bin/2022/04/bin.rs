use aoc_rs::prelude::*;

fn one() {
    let p = sb(LE, sb(",", sb("-", pn)));
    let s = pi!(p);
    s.into_iter()
        .map(|l| ((l[0][0], l[0][1]), (l[1][0], l[1][1])))
        .filter(|((a, b), (c, d))| a >= c && b <= d || a <= c && b >= d)
        .count()
        .save()
}

fn two() {
    let p = sb(LE, sb(",", sb("-", pn)));
    let s = pi!(p);
    s.into_iter()
        .map(|l| ((l[0][0], l[0][1]), (l[1][0], l[1][1])))
        .filter(|((a, b), (c, d))| {
            a >= c && a <= d || b >= c && b <= d || c >= a && c <= b || d >= a && d <= b
        })
        .count()
        .save()
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
