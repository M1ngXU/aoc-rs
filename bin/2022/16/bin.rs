#![feature(iter_map_windows)]

use std::ops::Neg;

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| "Valve " << (~" " > id) " has flow rate=" << pn "; tunnel" << ({"s"}{""}) << " lead" << ({"s"}{""}) << " to valve" << ({"s"}{""}) << ((| ms0 << id)[", "]))[LE]);
    let s: Vec<(&str, (isize, Vec<&str>))> = pi!(p);
    let a = s.iter().position(|(a, _)| a == &"AA").unwrap();
    let s = s
        .iter()
        .enumerate()
        .map(|(i, (_, (f, c)))| {
            (
                i,
                *f,
                c.iter()
                    .map(|c| s.iter().position(|(x, _)| x == c).unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let mut graph = FixedGraph::new();
    for (v, _, _) in &s {
        graph.add_vertex(*v);
    }
    for (v, _, t) in &s {
        for t in t {
            graph.add_edge(*v, *t, 1);
        }
    }
    let dists = graph.floyd_warshall().unwrap();
    let useful = s
        .iter()
        .filter(|(_, v, _)| v > &0)
        .enumerate()
        .map(|(ni, (oi, _, _))| (*oi, ni))
        .collect::<HashMap<_, _>>();
    let s = s
        .into_iter()
        .filter(|(i, _, _)| useful.contains_key(i))
        .map(|(i, v, _)| {
            (
                v,
                useful
                    .iter()
                    .map(|(j, r)| (*r, dists.distance(&i, &j).unwrap()))
                    .collect_vec(),
                dists.distance(&a, &i).unwrap(),
            )
        })
        .collect_vec();
    let best = s.iter().map(|(v, _, _)| *v).s() as i32;
    const END: isize = 26;
    dijkstra_dyn(
        s.iter()
            .enumerate()
            .cartesian_product(s.iter().enumerate())
            .filter(|((i, _), (j, _))| i < j)
            .map(|((i, (_, _, d)), (i2, (_, _, d2)))| {
                (
                    [-best * END as i32, 0i32],
                    (
                        (0..useful.len())
                            .map(|i| 1u32 << i)
                            .reduce(|a, b| a | b)
                            .unwrap(),
                        *d.min(d2) as i32,
                        *d as i32,
                        *d2 as i32,
                        0u32,
                        i as u32,
                        i2 as u32,
                    ),
                )
            })
            .collect_vec(),
        |[_, c],
         (
            open,
            time,
            do_something_at_human,
            do_something_at_ele,
            flow,
            current_human,
            current_ele,
        )| {
            let [human_states, ele_states] = [
                (do_something_at_human, current_human, current_ele),
                (do_something_at_ele, current_ele, current_human),
            ]
            .map(|(do_something_at, cur, other_cur)| {
                if do_something_at > time {
                    vec![(None, *do_something_at, *cur)]
                } else {
                    if open & (1 << *cur) != 0 {
                        vec![(Some(*cur), time + 1, *cur)]
                    } else {
                        let mut states = vec![];
                        if open != &0 {
                            for (to, cost) in &s[*cur as usize].1 {
                                if open & (1 << *to) != 0 && *other_cur != *to as u32 {
                                    states.push((None, time + *cost as i32, *to as _));
                                }
                            }
                        }
                        if states.is_empty() {
                            states.push((None, END as _, *cur));
                        }
                        states
                    }
                }
            });

            human_states
                .into_iter()
                .cartesian_product(ele_states)
                .filter(|((openh, _, curh), (opene, _, cure))| {
                    !(openh.is_some() && opene.is_some() && openh == opene)
                        && opene != &Some(*curh)
                        && curh != cure
                        && openh != &Some(*cure)
                })
                .map(|((openh, untilh, curh), (opene, untile, cure))| {
                    let mut open = *open;
                    let mut newflow = *flow;
                    if let Some(o) = openh {
                        open ^= 1 << o as u16;
                        newflow += s[o as usize].0 as u32;
                    }
                    if let Some(o) = opene {
                        open ^= 1 << o as u16;
                        newflow += s[o as usize].0 as u32;
                    }
                    let until = untilh.min(untile).max(time + 1).min(END as _);
                    let pass = until - time; // at least 1
                    let flow = c - *flow as i32 * pass;
                    (
                        [flow - (END as i32 - until) * best, flow],
                        (
                            open,
                            until,
                            if cure < curh { untile } else { untilh },
                            if cure < curh { untilh } else { untile },
                            newflow,
                            curh.min(cure),
                            cure.max(curh),
                        ),
                    )
                })
                .collect_vec()
        },
        |_, (_, t, _, _, _, _, _)| *t as isize == END,
    )
    .unwrap()
    .0[1]
        .neg()
        .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
