#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
    let p =
        parser!(("Game " << pn >> ": " ((pn >> " " {"red"}{"green"}{"blue"})[", "])["; "])["\n"]);

    let s: Vec<(isize, Vec<Vec<(isize, &str)>>)> = pi!(p);
    // let p = sble(id);
    // let red = r!("(\\d+) red");
    // let green = r!("(\\d+) green");
    // let blue = r!("(\\d+) blue");
    s.into_iter()
        .map(|(id, rgb)| {
            let r = rgb
                .iter()
                .filter_map(|r| r.iter().find(|(_, c)| c == &"red").map(|(x, _)| *x))
                .collect_vec();
            let g = rgb
                .iter()
                .filter_map(|r| r.iter().find(|(_, c)| c == &"green").map(|(x, _)| *x))
                .collect_vec();
            let b = rgb
                .iter()
                .filter_map(|r| r.iter().find(|(_, c)| c == &"blue").map(|(x, _)| *x))
                .collect_vec();
            (id, (r, g, b))
        })
        .filter(|(_, (r, g, b))| {
            r.iter().all(|r| r <= &12) && g.iter().all(|g| g <= &13) && b.iter().all(|b| b <= &14)
        })
        .map(|(s, _)| s)
        .s()
        .save()
}

fn two() {
    let p = sble(id);
    let s = pi!(p);
    let red = r!("(\\d+) red");
    let green = r!("(\\d+) green");
    let blue = r!("(\\d+) blue");
    s.into_iter()
        .map(|s| {
            let id = s.split_once("Game ").unwrap().1.split_once(":").unwrap().0;
            let r = red
                .captures_iter(s)
                .map(|c| c.get(1).unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            let g = green
                .captures_iter(s)
                .map(|c| c.get(1).unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            let b = blue
                .captures_iter(s)
                .map(|c| c.get(1).unwrap().as_str().parse::<usize>().unwrap())
                .collect_vec();
            (id.parse::<usize>().unwrap(), (r, g, b))
        })
        .map(|(_, (r, g, b))| {
            r.iter().copied().mx() * g.iter().copied().mx() * b.iter().copied().mx()
        })
        .s()
        .save()
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
