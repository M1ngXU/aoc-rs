#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn get_parser<'a>() -> impl FnMut(
    &'a str,
) -> IResult<
    &'a str,
    Vec<(isize, Vec<isize>, (&'a str, isize), isize, isize, isize)>,
> {
    sb(
        lle,
        tpl((
            dlt(t!("Monkey "), pn, pair(t!(":"), le)),
            dlt(t!("  Starting items: "), sb(t!(", "), pn), le),
            dlt(
                t!("  Operation: new = old "),
                pair(
                    alt((t!("*"), t!("+"))),
                    preceded(t!(" "), alt((map(t!("old"), |_| -1), pn))),
                ),
                le,
            ),
            dlt(t!("  Test: divisible by "), pn, le),
            dlt(t!("    If true: throw to monkey "), pn, le),
            preceded(t!("    If false: throw to monkey "), pn),
        )),
    )
}

fn one() {
    let p = get_parser();
    let mut s: Vec<(isize, Vec<isize>, (&str, isize), isize, isize, isize)> = pi!(p);
    let mut mi = vec![0; s.len()];
    for _ in 0..20 {
        for i in 0..s.len() {
            let m: &mut (isize, Vec<isize>, (&str, isize), isize, isize, isize) =
                unsafe { &mut *(&mut s[i] as *mut _) };
            for mut item in m.1.drain(..) {
                mi[i] += 1;
                match m.2 {
                    ("*", -1) => item *= item,
                    ("*", x) => item *= x,
                    ("+", x) => item += x,
                    _ => unreachable!(),
                }
                item /= 3;
                if item % m.3 == 0 {
                    s[m.4 as usize].1.push(item);
                } else {
                    s[m.5 as usize].1.push(item);
                }
            }
        }
    }
    mi.into_iter().sorted().rev().take(2).p().save();
}

fn two() {
    let p = get_parser();
    let mut s: Vec<(isize, Vec<isize>, (&str, isize), isize, isize, isize)> = pi!(p);
    let mut mi = vec![0; s.len()];
    let sm = s.iter().map(|s| s.3).p();
    for _ in 0..10000 {
        for i in 0..s.len() {
            let m: &mut (isize, Vec<isize>, (&str, isize), isize, isize, isize) =
                unsafe { &mut *(&mut s[i] as *mut _) };
            for mut item in m.1.drain(..) {
                mi[i] += 1;
                match m.2 {
                    ("*", -1) => item = (item * item) % sm,
                    ("*", x) => item = (item * x) % sm,
                    ("+", x) => item = (item + x) % sm,
                    _ => unreachable!(),
                }
                if item % m.3 == 0 {
                    s[m.4 as usize].1.push(item);
                } else {
                    s[m.5 as usize].1.push(item);
                }
            }
        }
    }
    mi.into_iter()
        .map(|s| s as u128)
        .sorted()
        .rev()
        .take(2)
        .p()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
