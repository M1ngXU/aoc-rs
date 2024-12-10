#![feature(iter_map_windows, iter_from_coroutine)]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use aoc_rs::prelude::*;
use rayon::ScopeFifo;

fn try_all(
    scope: &ScopeFifo<'_>,
    graph: Arc<Vec<Vec<(usize, isize)>>>,
    nc: usize,
    ec: usize,
    cost: isize,
    seen: u64,
    best1: Arc<AtomicUsize>,
    best2: Arc<AtomicUsize>,
) {
    if nc == ec {
        if cost < 0 {
            best2.fetch_max((-cost) as usize, Ordering::Relaxed);
        } else {
            best1.fetch_max(cost as usize, Ordering::Relaxed);
        }
    } else {
        for i in 0..graph[nc].len() {
            let c = graph[nc][i].0;
            let cc = graph[nc][i].1;
            if seen & 1 << c == 0 {
                let mut seen = seen;
                seen |= 1 << c;
                let mut new_cost = if cost < 0 {
                    cost - cc.abs()
                } else {
                    cost + cc.abs()
                };
                if cc < 0 && new_cost > 0 {
                    new_cost = -new_cost;
                }
                let graph = graph.clone();
                let best1 = best1.clone();
                let best2 = best2.clone();
                scope.spawn_fifo(move |s| try_all(s, graph, c, ec, new_cost, seen, best1, best2));
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
    let mut vertex_index = HashMap::new();
    let mut counter = 1;
    let mut segmented = FixedGraph::new();
    segmented.add_vertex((sx, sy, 0));
    vertex_index.insert((sx, sy), 0);
    let mut todo = vec![(sx, sy, 0)];
    while let Some((nx, ny, nc)) = todo.pop() {
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
            if !vertex_index.contains_key(&(cx, cy)) {
                vertex_index.insert((cx, cy), counter);
                todo.push((cx, cy, counter));
                counter += 1;
            }
            if (nx, ny) != (cx, cy) {
                segmented.add_edge(
                    (nx, ny, nc),
                    (cx, cy, vertex_index[&(cx, cy)]),
                    if only_part1 { 1 } else { -1 } * count as isize,
                );
            }
        }
    }
    let segmented_without_xy = segmented
        .adjacencies
        .iter()
        .map(|((_, _, c), adj)| (*c, adj))
        .sorted_by_key(|(c, _)| *c)
        .map(|(_, adj)| adj.iter().map(|((_, _, c), w)| (*c, *w)).collect_vec())
        .collect_vec();
    let best1 = Arc::new(AtomicUsize::new(0));
    let best2 = Arc::new(AtomicUsize::new(0));
    let ec = vertex_index[&(ex, ey)];
    let graph = Arc::new(segmented_without_xy);
    rayon::scope_fifo(|s| {
        try_all(s, graph, 0, ec, 0, 1, best1.clone(), best2.clone());
    });
    let best1 = best1.load(Ordering::Relaxed);
    let best2 = best2.load(Ordering::Relaxed);
    println!("Part 1: {best1}");
    println!("Part 2: {best2}");
}

fn main() {
    solve();
}
