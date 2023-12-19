#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!(id);
    let (a, b): (Vec<&str>, Vec<&str>) = pi!(p)
        .split_once("\n\n")
        .map(|(a, b)| (a.split("\n").collect_vec(), b.split("\n").collect_vec()))
        .unwrap();
    let rules: HashMap<&str, (Vec<(&str, &str)>, &str)> = a
        .into_iter()
        .map(|x| {
            x.split_once('{')
                .map(|(name, r)| {
                    (
                        name,
                        (
                            r[..r.len() - 1]
                                .split(",")
                                .take(r.split(",").count() - 1)
                                .map(|x| x.split_once(":").map(|(a, b)| (a, b)).unwrap())
                                .collect_vec(),
                            r.rsplit_once(",").unwrap().1.trim_end_matches('}'),
                        ),
                    )
                })
                .unwrap()
        })
        .collect::<HashMap<_, _>>();
    let cube = HypercubeSet::new(vec![[1..=4000, 1..=4000, 1..=4000, 1..=4000]]);
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
        for (rule, to) in rules {
            let (var, num) = rule.split_once(['<', '>']).unwrap();
            let num = num.parse::<i64>().unwrap();
            let mut next_cube = cube.clone();
            let mut ranges = vec![[1..=4000, 1..=4000, 1..=4000, 1..=4000]];
            let index = ["x", "m", "a", "s"]
                .into_iter()
                .position(|x| x == var)
                .unwrap();
            if rule.contains('<') {
                ranges[0][index] = 1..=num - 1;
            } else {
                ranges[0][index] = num + 1..=4000;
            }
            let ranges = HypercubeSet::new(ranges);
            next_cube.intersect(&ranges);
            cube.set_minus(&ranges);
            todo.push((to, next_cube));
        }
        todo.push((other, cube));
    }
    total.save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
