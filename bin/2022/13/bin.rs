#![feature(iter_map_windows)]

use std::cmp::Ordering;

use aoc_rs::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pkg {
    N(isize),
    P(Vec<Pkg>),
}

fn p2<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Pkg> {
    alt((
        map(pn, Pkg::N),
        map(dlt(t!("["), sb(t!(","), |i| p2()(i)), t!("]")), Pkg::P),
    ))
}

fn cmp(a: &Pkg, b: &Pkg) -> Ordering {
    match (a, b) {
        (Pkg::N(a), Pkg::N(b)) => a.cmp(b),
        (Pkg::P(a), Pkg::P(b)) => {
            for (a, b) in a.iter().zip(b) {
                let c = cmp(a, b);
                if c.is_ne() {
                    return c;
                }
            }
            a.len().cmp(&b.len())
        }
        (Pkg::P(_), Pkg::N(b)) => cmp(a, &Pkg::P(vec![Pkg::N(*b)])),
        (Pkg::N(a), Pkg::P(_)) => cmp(&Pkg::P(vec![Pkg::N(*a)]), b),
    }
}

fn one() {
    let p = _sb("\n\n", pair(pu("\n", p2()), p2()));
    let s = pi!(p);
    s.into_iter()
        .enumerate()
        .filter(|(_, (a, b))| cmp(a, b).is_lt())
        .map(|(i, _)| i + 1)
        .s()
        .save();
}

fn two() {
    let p = _sb("\n\n", pair(pu("\n", p2()), p2()));
    let s = pi!(p);
    let d1 = Pkg::P(vec![Pkg::P(vec![Pkg::N(2)])]);
    let d2 = Pkg::P(vec![Pkg::P(vec![Pkg::N(6)])]);
    let s = s
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .chain([d1.clone(), d2.clone()])
        .sorted_by(cmp)
        .collect::<Vec<_>>();
    s.into_iter()
        .enumerate()
        .filter(|(_, a)| a == &d1 || a == &d2)
        .map(|(i, _)| i + 1)
        .p()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
