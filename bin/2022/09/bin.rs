#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
    let p = _sb(
        LE,
        pair(
            alt((tag("L"), tag("U"), tag("R"), tag("D"))),
            preceded(t!(" "), pn),
        ),
    );
    let s = pi!(p);
    let i = s
        .into_iter()
        .map(|(d, n)| match d {
            "L" => repeat((-1, 0)).take(n as usize).collect_vec(),
            "U" => repeat((0, 1)).take(n as usize).collect_vec(),
            "R" => repeat((1, 0)).take(n as usize).collect_vec(),
            "D" => repeat((0, -1)).take(n as usize).collect_vec(),
            _ => unreachable!(),
        })
        .collect_vec();
    let mut t = (0isize, 0isize);
    let mut h = (0, 0);
    let mut t_visited = HashSet::new();
    for i in i {
        for (x, y) in i {
            h = (h.0 + x, h.1 + y);
            if t.0 + 2 == h.0 && t.1 == h.1 {
                t.0 += 1;
            } else if t.0 - 2 == h.0 && t.1 == h.1 {
                t.0 -= 1;
            } else if t.1 + 2 == h.1 && t.0 == h.0 {
                t.1 += 1;
            } else if t.1 - 2 == h.1 && t.0 == h.0 {
                t.1 -= 1;
            } else if t.1.abs_diff(h.1) > 1 || t.0.abs_diff(h.0 as _) > 1 {
                let mut m = (0, 0);
                if t.0 > h.0 {
                    m.0 = 1;
                } else {
                    m.0 = -1;
                }
                if t.1 > h.1 {
                    m.1 = 1;
                } else {
                    m.1 = -1;
                }
                t.0 -= m.0;
                t.1 -= m.1
            }
            t_visited.insert(t);
        }
    }
    t_visited.len().save();
}

fn two() {
    let p = _sb(
        LE,
        pair(
            alt((tag("L"), tag("U"), tag("R"), tag("D"))),
            preceded(tag(" "), pn),
        ),
    );
    let s = pi!(p);
    let i = s
        .into_iter()
        .map(|(d, n)| match d {
            "L" => repeat((-1, 0)).take(n as usize).collect_vec(),
            "U" => repeat((0, 1)).take(n as usize).collect_vec(),
            "R" => repeat((1, 0)).take(n as usize).collect_vec(),
            "D" => repeat((0, -1)).take(n as usize).collect_vec(),
            _ => unreachable!(),
        })
        .collect_vec();
    let mut h = [(0isize, 0isize); 10];
    let mut t_visited = HashSet::new();
    for i in i {
        for (x, y) in i {
            h[0] = (h[0].0 + x, h[0].1 + y);
            for i in 1..10 {
                let h0 = h[i - 1];
                let tail = &mut h[i];
                let front = h0;
                if tail.0 + 2 == front.0 && tail.1 == front.1 {
                    tail.0 += 1;
                } else if tail.0 - 2 == front.0 && tail.1 == front.1 {
                    tail.0 -= 1;
                } else if tail.1 + 2 == front.1 && tail.0 == front.0 {
                    tail.1 += 1;
                } else if tail.1 - 2 == front.1 && tail.0 == front.0 {
                    tail.1 -= 1;
                } else if ((tail.1 - front.1) as isize).abs() > 1
                    || ((tail.0 - front.0) as isize).abs() > 1
                {
                    let mut m = (0, 0);
                    if tail.0 > front.0 {
                        m.0 = 1;
                    } else {
                        m.0 = -1;
                    }
                    if tail.1 > front.1 {
                        m.1 = 1;
                    } else {
                        m.1 = -1;
                    }
                    tail.0 -= m.0;
                    tail.1 -= m.1
                }
            }
            t_visited.insert(h[9]);
        }
    }
    t_visited.len().save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
