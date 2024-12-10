#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn solve() {
    let p = parser!(~"\n\n" > ((| ~"{" > id >> "{" (~"}" > (((| ({~">" > id ">"}{~"<" > id "<"}) pn >> ":" id)[","]) "," << id)))[LE]) >> LELE ((| "{" << (~"}" > ((| ({"x"}{"m"}{"a"}{"s"}) >> "=" pn)[","])))[LE]));
    let (a, _b): (
        Vec<(&str, (Vec<((&str, &str), (isize, &str))>, &str))>,
        Vec<Vec<(&str, isize)>>,
    ) = pi!(p);
    let rules = a.into_iter().collect::<HashMap<_, _>>();
    let cube = HypercuboidSet::new(vec![[1..=4000, 1..=4000, 1..=4000, 1..=4000]]);
    let mut todo = vec![("in", cube)];
    let mut total = 0;
    while let Some((next, mut cube)) = todo.pop() {
        if next == "R" {
            continue;
        }
        if next == "A" {
            total += cube.size();
            continue;
        }
        let (rules, other) = &rules[next];
        for ((var, symbol), (num, to)) in rules {
            let mut next_cube = cube.clone();
            let mut ranges = vec![[1..=4000, 1..=4000, 1..=4000, 1..=4000]];
            let index = ["x", "m", "a", "s"]
                .into_iter()
                .position(|x| &x == var)
                .unwrap();
            if symbol == &"<" {
                ranges[0][index] = 1..=num - 1;
            } else {
                ranges[0][index] = num + 1..=4000;
            }
            let ranges = HypercuboidSet::new(ranges);
            next_cube &= &ranges;
            cube -= &ranges;
            todo.push((to, next_cube));
        }
        todo.push((other, cube));
    }
    total.save();
}

fn main() {
    print!("Part 2: ");
    solve();
}
