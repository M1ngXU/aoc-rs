#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
    let p = _sb(
        LLE,
        tuple((
            delimited(
                tag("Monkey "),
                map_parser(take_until(":"), pn),
                pair(tag(":"), tag(LE)),
            ),
            delimited(
                tag("  Starting items: "),
                map_parser(take_until(LE), _sb(", ", pn)),
                tag(LE),
            ),
            delimited(
                tag("  Operation: new = old "),
                pair(
                    alt((tag("*"), tag("+"))),
                    preceded(
                        tag(" "),
                        alt((map(tag("old"), |_| -1), map_parser(take_until(LE), pn))),
                    ),
                ),
                tag(LE),
            ),
            delimited(
                tag("  Test: divisible by "),
                map_parser(take_until(LE), pn),
                tag(LE),
            ),
            delimited(
                tag("    If true: throw to monkey "),
                map_parser(take_until(LE), pn),
                tag(LE),
            ),
            preceded(
                tag("    If false: throw to monkey "),
                map_parser(take_while(|_| true), pn),
            ),
        )),
    );
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
    let p = _sb(
        LLE,
        tpl((
            dlt(t!("Monkey "), mp(take_until(":"), pn), t!(":" @)),
            pcd(t!("  Starting items: "), pu(LE, _sb(", ", pn))),
            dlt(
                t!("  Operation: new = old "),
                pair(
                    alt((t!("*"), t!("+"))),
                    pcd(
                        t!(" "),
                        alt((map(t!("old"), |_| -1), mp(take_until(LE), pn))),
                    ),
                ),
                t!(),
            ),
            pcd(tag("  Test: divisible by "), pu(LE, pn)),
            pcd(tag("    If true: throw to monkey "), pu(LE, pn)),
            pcd(tag("    If false: throw to monkey "), pn),
        )),
    );
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
