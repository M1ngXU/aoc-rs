#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = grd;
    let s = pi!(p);
    let s2 = s.clone();

    let m: Matrix<Vec<(isize, isize)>, Dyn, Dyn, _> = s.map(|x| match x {
        '|' => vec![(0, 1), (0, -1)],
        '-' => vec![(1, 0), (-1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(0, 1), (-1, 0)],
        'F' => vec![(0, 1), (1, 0)],
        '.' => vec![],
        'S' => {
            vec![(-1, -1)]
        }
        _ => unreachable!(),
    });
    let m = m.pad(vec![]);
    let m2 = m.clone();

    let mm = m.map(|x| match x.first() {
        Some(&(-1, -1)) => {
            let (x, y, _) = s2.enumerate_iter().find(|(_, _, c)| c == &&'S').unwrap();
            let b = m2.adj(x + 1, y + 1, 1);
            println!("{:?}", b.iter().collect_vec());
            let mut iter = b.iter();
            iter.next(); // top left
            let mut out = vec![];
            if iter.next().unwrap().contains(&(0, 1)) {
                out.push((0, -1));
                println!("{out:?}");
            }
            iter.next(); // top right
            if iter.next().unwrap().contains(&(1, 0)) {
                out.push((-1, 0));
                println!("{out:?}");
            }
            iter.next(); // self
            if iter.next().unwrap().contains(&(-1, 0)) {
                out.push((1, 0));
                println!("{out:?}");
            }
            iter.next(); // bottom left
            if iter.next().unwrap().contains(&(0, -1)) {
                out.push((0, 1));
                println!("{out:?}");
            }
            iter.next(); // bottom right
            assert!(iter.next().is_none());
            println!("{out:?}");
            out
        }
        _ => x,
    });

    let (x, y, _) = s2.enumerate_iter().find(|(_, _, c)| c == &&'S').unwrap();
    let x = x + 1; // padding
    let y = y + 1;

    println!("{mm:?}");

    dijkstraa(
        (0, (x as isize, y as isize)),
        |c, (x, y)| {
            // println!("{x}|{y}|{c}|{xx}|{yy}");
            mm.get((*x as usize, *y as usize))
                .unwrap()
                .iter()
                .map(|(dx, dy)| (c + 1, (x + dx, y + dy)))
                .collect_vec()
        },
        |_, _| 0,
    )
    .into_iter()
    .map(|(c, _)| c)
    .mx()
    .save();
}

fn two() {
    let p = parser!((| ch)[LE]);
    let s = pi!(p);
    let s2 = s.clone();

    let mut m = s
        .clone()
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|x| match x {
                    '|' => vec![(0, 1), (0, -1)],
                    '-' => vec![(1, 0), (-1, 0)],
                    'L' => vec![(0, -1), (1, 0)],
                    'J' => vec![(0, -1), (-1, 0)],
                    '7' => vec![(0, 1), (-1, 0)],
                    'F' => vec![(0, 1), (1, 0)],
                    '.' => vec![],
                    'S' => {
                        vec![(-1, -1)]
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    println!("{m:?}");
    m.insert(0, vec![vec![]; m[0].len()]);
    m.push(vec![vec![]; m[0].len()]);
    for r in m.iter_mut() {
        r.insert(0, vec![]);
        r.push(vec![]);
    }
    println!("{m:?}");
    let m2 = m.clone();
    assert_eq!(m, m2);

    let mm = m
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|x| match x.first() {
                    Some(&(-1, -1)) => {
                        let (x, y) = s2
                            .clone()
                            .into_iter()
                            .enumerate()
                            .find_map(|(y, r)| r.into_iter().position(|c| c == 'S').map(|x| (x, y)))
                            .unwrap();

                        let mut out = vec![];
                        // println!(
                        //     "{x}|{y}|{}|{}|{:?}|{:?}|{:?}|{:?}|{}|{}|{}|{}",
                        //     x + 1,
                        //     y + 2,
                        //     m2.get((y + 0, x + 1)).unwrap(),
                        //     m2.get((y + 1, x + 0)).unwrap(),
                        //     m2.get((y + 1, x + 2)).unwrap(),
                        //     m2.get((y + 2, x + 1)).unwrap(),
                        //     s2.get((y - 1, x + 0)).unwrap(),
                        //     s2.get((y + 0, x - 1)).unwrap(),
                        //     s2.get((y + 0, x + 1)).unwrap(),
                        //     s2.get((y + 1, x + 0)).unwrap(),
                        // );
                        if m2[y][x + 1].contains(&(0, 1)) {
                            out.push((0, -1));
                            println!("{out:?}");
                        }

                        if m2[y + 1][x].contains(&(1, 0)) {
                            out.push((-1, 0));
                            println!("{out:?}");
                        }

                        if m2[y + 1][x + 2].contains(&(-1, 0)) {
                            out.push((1, 0));
                            println!("{out:?}");
                        }

                        if m2[y + 2][x + 1].contains(&(0, -1)) {
                            out.push((0, 1));
                            println!("{out:?}");
                        }

                        println!("out: {out:?}|{x}|{y}");
                        out
                    }
                    _ => x,
                })
                .collect_vec()
        })
        .collect_vec();

    let (x, y) = s2
        .into_iter()
        .enumerate()
        .find_map(|(y, r)| r.into_iter().position(|c| c == 'S').map(|x| (x, y)))
        .unwrap();
    let x = x + 1; // padding
    let y = y + 1;

    println!("{mm:?}");

    let reachable = dijkstraa(
        (0, (x as isize, y as isize)),
        |c, (x, y)| {
            // println!("{x}|{y}|{c}|{xx}|{yy}");
            mm[*y as usize][*x as usize]
                .iter()
                .map(|(dx, dy)| (c + 1, (x + dx, y + dy)))
                .collect_vec()
        },
        |_, _| 0,
    );
    println!("fff {reachable:?}");
    let mut mmm = mm
        .iter()
        .map(|r| r.iter().map(|_| vec![]).collect_vec())
        .collect_vec();
    for (_, (x, y)) in reachable {
        mmm[y as usize][x as usize] = mm[y as usize][x as usize].clone();
    }
    println!("{mmm:?}");

    let mut inside = 0;
    for (x, y) in (1..mmm[0].len() - 1).cartesian_product(1..mmm.len() - 1) {
        if mmm[y][x] != vec![] || s[y - 1][x - 1] != '.' {
            continue;
        }
        // println!("{x}|{y}");
        let mut ins: bool = true;
        let mut inv = 0;

        let mut counter = 0;
        for xx in (0..x).rev() {
            if mmm[y][xx].contains(&(1, 0)) || mmm[y][xx].contains(&(-1, 0)) {
                counter = 1;
                if xx == x - 1 {
                    inv += 1;
                }
                break;
            }
            if mmm[y][xx].contains(&(0, 1)) || mmm[y][xx].contains(&(0, -1)) {
                counter += 1;
            }
        }
        if counter % 2 == 0 {
            // println!("x|{counter}");
            ins = false;
        }
        let mut counter = 0;
        for xx in x + 1..mmm[0].len() - 1 {
            if mmm[y][xx].contains(&(1, 0)) || mmm[y][xx].contains(&(-1, 0)) {
                counter = 1;
                if xx == x + 1 {
                    inv += 1;
                }
                break;
            }
            if mmm[y][xx].contains(&(0, 1)) || mmm[y][xx].contains(&(0, -1)) {
                counter += 1;
            }
        }
        if counter % 2 == 0 {
            // println!("xx");
            ins = false;
        }
        let mut counter = 0;
        for yy in (0..y).rev() {
            if mmm[yy][x].contains(&(0, 1)) || mmm[yy][x].contains(&(0, -1)) {
                counter = 1;
                if yy == y - 1 {
                    inv += 1;
                }
                break;
            }
            if mmm[yy][x].contains(&(1, 0)) || mmm[yy][x].contains(&(-1, 0)) {
                counter += 1;
            }
        }
        if counter % 2 == 0 {
            // println!("y");
            ins = false;
        }
        let mut counter = 0;
        for yy in y + 1..mmm.len() - 1 {
            if mmm[yy][x].contains(&(0, 1)) || mmm[yy][x].contains(&(0, -1)) {
                counter = 1;
                if yy == y + 1 {
                    inv += 1;
                }
                break;
            }
            if mmm[yy][x].contains(&(1, 0)) || mmm[yy][x].contains(&(-1, 0)) {
                counter += 1;
            }
        }
        if counter % 2 == 0 {
            // println!("yy");
            ins = false;
        }
        let mut i = false;
        for d in 0..x.min(y) {
            let dx = -(d as isize);
            if mmm[dy + y][dx + x] {}
        }
        if ins {
            println!("here: {x}|{y}|{inv}");
            inside += 1;
        }
    }
    inside.save();
}

fn main() {
    // print!("Part 1: ");
    // one();
    print!("Part 2: ");
    two();
}
