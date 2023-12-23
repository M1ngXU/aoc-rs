#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use aoc_rs::prelude::*;
use rayon::ScopeFifo;

fn try_all(
    scope: &ScopeFifo<'_>,
    graph: Arc<FixedGraph<(usize, usize)>>,
    nx: usize,
    ny: usize,
    ex: usize,
    ey: usize,
    c: isize,
    seen: HashSet<(usize, usize)>,
    best1: Arc<AtomicUsize>,
    best2: Arc<AtomicUsize>,
) {
    if (nx, ny) == (ex, ey) {
        if c < 0 {
            best2.fetch_max((-c) as usize, Ordering::Relaxed);
        } else {
            best1.fetch_max(c as usize, Ordering::Relaxed);
        }
    } else {
        for ((x, y), cc) in graph
            .get_edges(&(nx, ny))
            .unwrap()
            .iter()
            .map(|((x, y), c)| ((*x, *y), *c))
        {
            if !seen.contains(&(x, y)) {
                let mut seen = seen.clone();
                seen.insert((nx, ny));
                let mut new_cost = if c < 0 { c - cc.abs() } else { c + cc.abs() };
                if cc < 0 && new_cost > 0 {
                    new_cost = -new_cost;
                }
                let graph = graph.clone();
                let best1 = best1.clone();
                let best2 = best2.clone();
                scope.spawn_fifo(move |s| {
                    try_all(s, graph, x, y, ex, ey, new_cost, seen, best1, best2)
                });
            }
        }
    }
}

fn solve() {
    let p = parser!((| ch)[LE]);
    let s = pi!(p);
    let mut graph = FixedGraph::new();
    let (sx, sy) = (s[0].iter().position(|c| c == &'.').unwrap(), 0);
    let (ex, ey) = (
        s.last().unwrap().iter().position(|c| c == &'.').unwrap(),
        s.len() - 1,
    );
    for (y, row) in s.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c != &'#' {
                graph.add_vertex((x, y));
            }
        }
    }
    for (cx, cy) in s.ii() {
        if graph.adjacencies.contains_key(&(cx, cy)) {
            for (dx, dy) in ADJ {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if s.ibi(nx, ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if graph.adjacencies.contains_key(&(nx, ny)) {
                        graph.add_edge(
                            (cx, cy),
                            (nx, ny),
                            if ['<', '>', '^', 'v']
                                .zp(ADJ)
                                .into_iter()
                                .all(|(x, y)| x != s[cy][cx] || y == (dx, dy))
                            {
                                0
                            } else {
                                1
                            },
                        );
                    }
                }
            }
        }
    }
    let mut segmented = FixedGraph::new();
    segmented.add_vertex((sx, sy));
    let mut todo = vec![(sx, sy)];
    while let Some((nx, ny)) = todo.pop() {
        for ((x, y), _) in graph.adjacencies[&(nx, ny)]
            .iter()
            .filter(|(_, z)| **z == 0)
        {
            let mut count = 1;
            let mut ox = nx;
            let mut oy = ny;
            let mut cx = *x;
            let mut cy = *y;
            let mut only_part1 = true;
            while graph.get_edges(&(cx, cy)).unwrap().len() <= 2 {
                let (oox, ooy) = (cx, cy);
                let Some(((ccx, ccy), w)) = graph
                    .get_edges(&(cx, cy))
                    .unwrap()
                    .iter()
                    .find(|(xy, _)| xy != &&(ox, oy))
                    .map(|(x, w)| (*x, *w))
                else {
                    break;
                };
                if w == 1 {
                    only_part1 = false;
                }
                (cx, cy) = (ccx, ccy);
                (ox, oy) = (oox, ooy);
                count += 1;
            }
            if !segmented.adjacencies.contains_key(&(cx, cy)) {
                todo.push((cx, cy));
            }
            if (nx, ny) != (cx, cy) {
                segmented.add_edge(
                    (nx, ny),
                    (cx, cy),
                    if only_part1 { 1 } else { -1 } * count as isize,
                );
            }
        }
    }
    let best1 = Arc::new(AtomicUsize::new(0));
    let best2 = Arc::new(AtomicUsize::new(0));
    let graph = Arc::new(segmented);
    rayon::scope_fifo(|s| {
        try_all(
            s,
            graph,
            sx,
            sy,
            ex,
            ey,
            0,
            HashSet::new(),
            best1.clone(),
            best2.clone(),
        );
    });
    let best1 = best1.load(Ordering::Relaxed);
    let best2 = best2.load(Ordering::Relaxed);
    println!("Part 1: {best1}");
    println!("Part 2: {best2}");
}

fn main() {
    solve();
}
