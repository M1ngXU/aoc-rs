#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| ~" " > id >> " " pn >> " (#" (~")" > id) >> ")")[LE]);
    let s = pi!(p);
    let mut posx = 0;
    let mut posy = 0;
    let mut pos = HashSet::new();
    for (id, (count, hex)) in s {
        let code = &hex[..5];
        let id = ["R", "D", "L", "U"][hex
            .chars()
            .nth(5)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap()];
        let count = u32::from_str_radix(code, 16).unwrap();
        let mut poss = vec![];
        match id {
            "R" => {
                for c in 0..count {
                    posx += 1;
                    poss.push((posx, posy))
                }
            }
            "L" => {
                for c in 0..count {
                    posx -= 1;
                    poss.push((posx, posy))
                }
            }
            "U" => {
                for c in 0..count {
                    posy -= 1;
                    poss.push((posx, posy))
                }
            }
            "D" => {
                for c in 0..count {
                    posy += 1;
                    poss.push((posx, posy))
                }
            }
            _ => unreachable!(),
        }
        // minx = posx.min(minx);
        // maxx = posx.min(maxx);
        // miny = posy.min(miny);
        // maxy = posy.min(maxy);
        for p in poss {
            pos.insert((p.0, p.1));
        }
    }
    println!("{pos:?}");
    let mut s = vec![(5, 1)];
    while let Some(next) = s.pop() {
        if pos.insert(next) {
            s.push((next.0 - 1, next.1));
            s.push((next.0 + 1, next.1));
            s.push((next.0, next.1 - 1));
            s.push((next.0, next.1 + 1));
        }
    }
    pos.len().save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
