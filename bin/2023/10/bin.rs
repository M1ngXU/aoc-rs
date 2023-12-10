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
    m.insert(0, vec![vec![]; m[0].len()]);
    m.push(vec![vec![]; m[0].len()]);
    for r in m.iter_mut() {
        r.insert(0, vec![]);
        r.push(vec![]);
    }
    let m2 = m.clone();

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
                        if m2[y][x + 1].contains(&(0, 1)) {
                            out.push((0, -1));
                        }

                        if m2[y + 1][x].contains(&(1, 0)) {
                            out.push((-1, 0));
                        }

                        if m2[y + 1][x + 2].contains(&(-1, 0)) {
                            out.push((1, 0));
                        }

                        if m2[y + 2][x + 1].contains(&(0, -1)) {
                            out.push((0, 1));
                        }
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

    enum State {
        Loop,
        NonBlocking,
        Ground,
    }

    let mut mmm = mm
        .iter()
        .map(|r| r.iter().map(|_| State::Ground).collect_vec())
        .collect_vec();
    for (_, (x, y)) in reachable {
        if !['F', 'L'].contains(&s[y as usize - 1][x as usize - 1]) {
            mmm[y as usize][x as usize] = State::NonBlocking;
        } else if s[y as usize - 1][x as usize - 1] != '.' {
            mmm[y as usize][x as usize] = State::Loop;
        }
    }

    let mut inside = 0;
    for y in 0..mmm.len() {
        let mut outside = true;
        for x in 0..=y {
            if mmm[y - x][x] == State::Loop {
                outside = !outside;
            } else {
            }
        }
    }

    let mut inside = 0;
    'outer: for (x, y) in (1..mmm[0].len() - 1)
        .cartesian_product(1..mmm.len() - 1)
        .filter(|(x, y)| mmm[*y][*x].is_empty())
    {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx != 0 && dy != 0) {
                    //(dx == 0 || dy == 0) && dx != dy {
                    let mut xx = x as isize;
                    let mut yy = y as isize;
                    let mut inside = 0;
                    while (1..mmm[0].len() - 1).contains(&(xx as usize))
                        && (1..mmm.len() - 1).contains(&(yy as usize))
                    {
                        let non_blocking = vec![(-dx, 0), (0, dy), (0, -dy), (dx, 0)];
                        if !(mmm[yy as usize][xx as usize].contains(&non_blocking[0])
                            && mmm[yy as usize][xx as usize].contains(&non_blocking[1])
                            || mmm[yy as usize][xx as usize].contains(&non_blocking[2])
                                && mmm[yy as usize][xx as usize].contains(&non_blocking[3]))
                            && !mmm[yy as usize][xx as usize].is_empty()
                        {
                            inside += 1;
                        }
                        xx += dx;
                        yy += dy;
                    }
                    if inside % 2 == 0 {
                        continue 'outer;
                    }
                }
            }
        }
        inside += 1;
    }
    inside.save();
}

fn main() {
    // print!("Part 1: ");
    // one();
    print!("Part 2: ");
    two();
}
