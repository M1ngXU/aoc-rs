use std::time::Instant;

use aoc_rs::util::Save;
use itertools::Itertools;

const TABLE: [((isize, isize), (isize, isize), bool); 13] = [
    ((0, 0), (0, 0), false),
    ((0, 0), (0, 0), false),
    ((0, 0), (0, 0), false),
    ((0, 1), (1, 0), false), // F
    ((0, 0), (0, 0), false),
    ((0, 1), (-1, 0), false), // 7
    ((1, 0), (-1, 0), false), // -
    ((0, 0), (0, 0), false),
    ((0, 0), (0, 0), false),
    ((0, -1), (0, 1), true), // |
    ((0, -1), (1, 0), true), // L
    ((0, 0), (0, 0), false),
    ((0, -1), (-1, 0), true), // J
];

fn solve() {
    let mut s = include_str!("input.txt")
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| match c {
                    // top left right bottom
                    'F' => 0b0011,
                    '7' => 0b0101,
                    '-' => 0b0110,
                    '|' => 0b1001,
                    'L' => 0b1010,
                    'J' => 0b1100,
                    'S' => 0b1111, // might connect to anywhere
                    _ => 0b0000,   // otherwise, '.', connects nowhere
                })
                .collect_vec()
        })
        .collect_vec();
    let (x, y) = s
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|x| x == &0b1111).map(|x| (x, y)))
        .unwrap();

    s[y][x] = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(dy, dx)| (dx == &0 || dy == &0) && dx != dy)
        .zip([0b0001_u8, 0b0010, 0b0100, 0b1000])
        .filter_map(|((dy, dx), mask)| {
            let xy = [(x as isize + dx) as usize, (y as isize + dy) as usize];
            xy.iter()
                .all(|x| (0..s.len()).contains(x))
                .then(|| (xy[0], xy[1], mask))
        })
        .map(|(x, y, mask)| s[y][x] & mask)
        .reduce(|a, b| a | b)
        .unwrap();

    let (mut cx @ mut ox, mut cy @ mut oy) = (x as isize, y as isize);
    let mut closed_walk = vec![vec![0; s.len()]; s.len()];
    let mut len: usize = 0;
    while {
        let c = s[cy as usize][cx as usize];
        let ((dx1, dy1), (dx2, dy2), i) = TABLE[c as usize];
        let (oox, ooy) = (cx, cy);
        if cx + dx1 == ox && cy + dy1 == oy {
            cx += dx2;
            cy += dy2;
        } else {
            cx += dx1;
            cy += dy1;
        }
        (ox, oy) = (oox, ooy);
        closed_walk[oy as usize][ox as usize] = i as usize + 1;
        len += 1;
        cx != x as isize || cy != y as isize
    } {}

    println!("Part 1: {}", len.div_ceil(2));

    print!("Part 2: ");
    (0..s.len())
        .map(|y| {
            let mut inside = 0;
            let mut outside = true;
            for x in 0..s.len() {
                match closed_walk[y as usize][x as usize] {
                    2 => outside = !outside,
                    0 if !outside => inside += 1,
                    _ => {}
                }
            }
            inside
        })
        .sum::<usize>()
        .save();
}

fn main() {
    let start = Instant::now();
    solve();
    println!("Time: {:?}", start.elapsed());
}
