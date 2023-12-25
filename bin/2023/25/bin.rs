#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine, never_type)]

use aoc_rs::prelude::*;

fn one() {
    // TODO: generic graph parser, (un)directed
    let p = parser!((| ~":" > id >> ": " (| id)[" "])[LE]);
    let s: Vec<(&str, Vec<&str>)> = pi!(p);
    let mut pgraph: Graph<_, (), Undirected> = Graph::new_undirected();
    let mut mapping = HashMap::new();
    for (from, tos) in s.clone() {
        let f = if let Some((f, _)) = mapping.iter().find(|(_, &x)| x == from) {
            *f
        } else {
            let f = pgraph.add_node(from);
            mapping.insert(f, from);
            f
        };
        for to in tos {
            let t = if let Some((t, _)) = mapping.iter().find(|(_, &x)| x == to) {
                *t
            } else {
                let t = pgraph.add_node(to);
                mapping.insert(t, to);
                t
            };
            mapping.insert(t, to);
            pgraph.add_edge(f, t, ());
        }
    }
    let x = stoer_wagner_min_cut(&pgraph, |_| Ok::<_, !>(1))
        .unwrap()
        .unwrap()
        .1
        .len();
    (x * (pgraph.node_count() - x)).save();
}

fn main() {
    print!("Part 1: ");
    one();
}
