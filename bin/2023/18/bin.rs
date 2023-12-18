#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| ~" " > id >> " " pn >> " (#" (~")" > id) >> ")")[LE]);
    let s = pi!(p);
    let mut posx = 0i64;
    let mut posy = 0i64;
    let mut minx = i64::MAX;
    let mut maxx = i64::MIN;
    let mut miny = i64::MAX;
    let mut maxy = i64::MIN;
    let mut pos = HashSet::new();
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
        // let mut poss = vec![];
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
        fill += (oldx * posy - oldy * posx);
        counts += count;
        pos.insert((
            oldx.min(posx)..=posx.max(oldx),
            oldy.min(posy)..=posy.max(oldy),
        ));
        minx = minx.min(posx);
        maxx = maxx.max(posx);
        miny = miny.min(posy);
        maxy = maxy.max(posy);
    }
    (fill / 2 + counts / 2 + 1).save();
    ray_intersection(
        miny..=maxy,
        |y| {
            pos.iter()
                .filter(|(_, yy)| yy.contains(&y))
                .sorted_by(|(x, _), (x2, _)| {
                    ((*x).start())
                        .cmp((*x2).start())
                        .then(x.end().cmp(x2.end()).reverse())
                })
                .fold(vec![], |mut a, (x, _)| {
                    if a.is_empty() {
                        a.push(x.clone());
                    } else if x.start() == x.end() && a[a.len() - 1].end() >= x.start() {
                        let new = (*a[a.len() - 1].start())..=(*x.end()).max(*a[a.len() - 1].end());
                        let z = a.len() - 1;
                        a[z] = new;
                    } else {
                        a.push(x.clone());
                    }
                    a
                })
                .into_iter()
        },
        |x, y1, y2| {
            pos.iter()
                .find(|(xx, yy)| xx.contains(x) && yy.contains(y1) && yy.contains(y2))
                .is_some()
        },
    )
    .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
