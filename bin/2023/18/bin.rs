#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| ~" " > id >> " " pn >> " (#" (~")" > id) >> ")")[LE]);
    let s = pi!(p);
    let mut posx = 0i64;
    let mut posy = 0i64;
    let mut fill = 0;
    let mut counts = 0;
    for (id, (count, hex)) in s {
        let code = &hex[..5];
        let id = ["R", "D", "L", "U"][hex
            .chars()
            .nth(5)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap()];
        let count = u32::from_str_radix(code, 16).unwrap() as i64;

        let count = count as i64;
        let oldx = posx;
        let oldy = posy;
        match id {
            "R" => {
                posx += count;
            }
            "L" => {
                posx -= count;
            }
            "U" => {
                posy -= count;
            }
            "D" => {
                posy += count;
            }
            _ => unreachable!(),
        }
        fill += oldx * posy - oldy * posx;
        counts += count;
    }
    (fill / 2 + counts / 2 + 1).save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
