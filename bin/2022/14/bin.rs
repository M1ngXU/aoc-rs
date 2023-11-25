#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
    let p = sb("\n", sb(" -> ", spair(mp(tu(","), pn), t!(","), pn)));
    let s = pi!(p);
    let max_x = s.iter().flatten().map(|(x, _)| x).copied().mx().max(500);
    let min_x = s.iter().flatten().map(|(x, _)| x).copied().mn().min(500);
    let min_y = s.iter().flatten().map(|(_, y)| y).copied().mn().min(0);
    let max_y = s.iter().flatten().map(|(_, y)| y).copied().mx().max(0);
    let mut grid = HashSet::new();
    for block in s.into_iter().filter(|b| !b.is_empty()) {
        let (mut ox, mut oy) = block[0];
        grid.insert((ox, oy));
        for (x, y) in &block[1..] {
            if x == &ox {
                let r = if y > &oy { oy..=*y } else { *y..=oy };
                for y in r {
                    grid.insert((*x, y));
                }
            } else {
                let r = if x > &ox { ox..=*x } else { *x..=ox };
                for x in r {
                    grid.insert((x, *y));
                }
            }
            ox = *x;
            oy = *y;
        }
    }
    // let mut grid_b = vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    // for (x, y) in &grid {
    //     grid_b[(y - min_y) as usize][(x - min_x) as usize] = true;
    // }
    // grid_b
    //     .into_iter()
    //     .flatten()
    //     .collect_vec()
    //     .db((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize);
    let mut count = 0;
    loop {
        let mut sand = (500_isize, 0_isize);
        while (min_x..=max_x).contains(&sand.0) && (min_y..=max_y).contains(&sand.1) {
            if !grid.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !grid.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !grid.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                count += 1;
                break;
            }
        }
        if !((min_x..=max_x).contains(&sand.0) && (min_y..=max_y).contains(&sand.1)) {
            break;
        }
        grid.insert(sand);
    }
    count.save();
}

fn two() {
    let p = sb("\n", sb(" -> ", spair(mp(tu(","), pn), t!(","), pn)));
    let s = pi!(p);
    let max_x = s.iter().flatten().map(|(x, _)| x).copied().mx().max(500);
    let min_x = s.iter().flatten().map(|(x, _)| x).copied().mn().min(500);
    let min_y = s.iter().flatten().map(|(_, y)| y).copied().mn().min(0);
    let max_y = s.iter().flatten().map(|(_, y)| y).copied().mx().max(0);
    let mut grid = HashSet::new();
    for block in s.into_iter().filter(|b| !b.is_empty()) {
        let (mut ox, mut oy) = block[0];
        grid.insert((ox, oy));
        for (x, y) in &block[1..] {
            if x == &ox {
                let r = if y > &oy { oy..=*y } else { *y..=oy };
                for y in r {
                    grid.insert((*x, y));
                }
            } else {
                let r = if x > &ox { ox..=*x } else { *x..=ox };
                for x in r {
                    grid.insert((x, *y));
                }
            }
            ox = *x;
            oy = *y;
        }
    }
    // let mut grid_b = vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    // for (x, y) in &grid {
    //     grid_b[(y - min_y) as usize][(x - min_x) as usize] = true;
    // }
    // grid_b
    //     .into_iter()
    //     .flatten()
    //     .collect_vec()
    //     .db((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize);
    let mut count = 0;
    loop {
        let mut sand = (500_isize, 0_isize);
        if grid.contains(&sand) {
            break;
        }
        while sand.1 != max_y + 2 {
            if !grid.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !grid.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
            } else if !grid.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
            } else {
                count += 1;
                break;
            }
        }
        grid.insert(sand);
    }
    count.save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
