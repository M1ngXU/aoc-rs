#![feature(iter_map_windows, iter_from_coroutine, never_type)]

use std::{sync::Arc, thread::available_parallelism};

use aoc_rs::prelude::*;

fn one() {
    // TODO: generic graph parser, (un)directed
    let p = parser!((| ~":" > id >> ": " (| id)[" "])[LE]);
    let s: Vec<(&str, Vec<&str>)> = pi!(p);

    let mut graph = FixedGraph::new();
    for (from, tos) in s.clone() {
        for to in tos {
            graph.add_undirected_edge(from, to, 1);
        }
    }
    let mut a = vec![];
    let graph2 = Arc::new(graph.clone());
    for _ in 0..available_parallelism().unwrap().get() {
        let graph = graph2.clone();
        a.push(std::thread::spawn(move || {
            let mut freq = HashMap::new();
            for _ in 0..500 {
                for (from, tos) in graph.mst(true).unwrap().adjacencies {
                    for (to, _) in tos {
                        *freq
                            .entry(if from < to { (from, to) } else { (to, from) })
                            .or_insert(0) += 1_usize;
                    }
                }
            }
            freq
        }));
    }
    let freq = a
        .into_iter()
        .map(|x| x.join().unwrap())
        .reduce(|mut a, b| {
            for ((from, to), v) in b {
                *a.entry((from, to)).or_insert(0) += v;
            }
            a
        })
        .unwrap();
    println!(
        "{:?}",
        freq.clone()
            .into_iter()
            .sorted_by_key(|(_, v)| *v)
            .rev()
            .take(10)
            .collect_vec()
    );
    for ((from, to), _) in freq.into_iter().sorted_by_key(|(_, v)| *v).rev().take(3) {
        graph.remove_undirected_edge(&from, &to).unwrap();
    }
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(*graph.adjacencies.keys().next().unwrap());
    while let Some(node) = queue.pop_front() {
        visited.insert(node);
        for (to, _) in graph.adjacencies[&node].clone() {
            if !visited.contains(&to) {
                queue.push_back(to);
            }
        }
    }
    let a = visited.len();
    queue.push_back(
        graph
            .adjacencies
            .keys()
            .find(|x| !visited.contains(*x))
            .unwrap(),
    );
    visited.clear();
    while let Some(node) = queue.pop_front() {
        visited.insert(node);
        for (to, _) in graph.adjacencies[&node].clone() {
            if !visited.contains(&to) {
                queue.push_back(to);
            }
        }
    }
    let b = visited.len();
    (a * b).save();

    // let s: Vec<(&str, Vec<&str>)> = pi!(p);
    // let mut pgraph: Graph<_, (), Undirected> = Graph::new_undirected();
    // let mut mapping = HashMap::new();
    // for (from, tos) in s.clone() {
    //     let f = if let Some((f, _)) = mapping.iter().find(|(_, &x)| x == from) {
    //         *f
    //     } else {
    //         let f = pgraph.add_node(from);
    //         mapping.insert(f, from);
    //         f
    //     };
    //     for to in tos {
    //         let t = if let Some((t, _)) = mapping.iter().find(|(_, &x)| x == to) {
    //             *t
    //         } else {
    //             let t = pgraph.add_node(to);
    //             mapping.insert(t, to);
    //             t
    //         };
    //         mapping.insert(t, to);
    //         pgraph.add_edge(f, t, ());
    //     }
    // }
    // let x = stoer_wagner_min_cut(&pgraph, |_| Ok::<_, !>(1))
    //     .unwrap()
    //     .unwrap()
    //     .1
    //     .len();
    // (x * (pgraph.node_count() - x)).save();
}

fn main() {
    print!("Part 1: ");
    one();
}
