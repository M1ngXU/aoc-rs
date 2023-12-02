// See history for another version with regex

use aoc_rs::prelude::*;

fn one() {
    let p = sble(pds);
    let s = pi!("example1.txt": p);
    s.into_iter().map(|r| 10 * r.f() + r.l()).s().save()
}

fn two() {
    let r = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let rp = fsa::<9>().zp(r).map(|(n, s)| format!("{s}{}{s}", n + 1));
    let p = sble(mp(
        rpl(r.map(|n| r!(n)), rp.iter().map(|s| s.as_str()).cfsa()),
        pds,
    ));
    let s = pi!("example2.txt": p);
    s.into_iter().map(|r| 10 * r.f() + r.l()).s().save()
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
