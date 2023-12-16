#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use std::{sync::Arc, thread::available_parallelism, time::Instant};

use aoc_rs::prelude::*;

fn one() {
    let p =
        parser!(~"\n" > ch >> "\n\n" (~" " > id >> " = (" ~"," > id >> ", " ~")" > id >> ")")[LE]);
    let (t, rest) = pi!("example1.txt": p);
    let mut inst = "AAA";
    let mut counts = 0;
    for c in t.into_iter().cycle() {
        if inst == "ZZZ" {
            break;
        }
        if c == 'L' {
            inst = rest.iter().find(|x| x.0 == inst).unwrap().1 .0;
        } else {
            inst = rest.iter().find(|x| x.0 == inst).unwrap().1 .1;
        }
        counts += 1;
    }
    counts.save();
}

#[allow(dead_code)]
fn two() {
    let p =
        parser!(~"\n" > ch >> "\n\n" (~" " > id >> " = (" ~"," > id >> ", " ~")" > id >> ")")[LE]);
    let (t, rest) = pi!("example2.txt": p);
    rest.iter()
        .filter(|s| s.0.ends_with('A'))
        .map(|s| {
            t.iter()
                .copied()
                .cycle()
                .scan(s.0, |i, c| {
                    if i.ends_with('Z') {
                        return None;
                    }
                    let ch = rest.iter().find(|x| x.0 == *i).unwrap().1;
                    *i = if c == 'L' { ch.0 } else { ch.1 };
                    Some(())
                })
                .count()
        })
        .reduce(GcdLcm::lcm)
        .unwrap()
        .save();
}

const SEND_AFTER: usize = (1 << 28) - 1;

fn twop() {
    let p =
        parser!(~"\n" > ch >> "\n\n" (~" " > id >> " = (" ~"," > id >> ", " ~")" > id >> ")")[LE]);
    let (t, rest_old): (Vec<char>, Vec<(&str, (&str, &str))>) = pi!("example2.txt": p);

    let mapping: HashMap<&str, usize> = rest_old
        .iter()
        .copied()
        .sorted_by_key(|(x, _)| x.chars().nth(2).unwrap())
        .enumerate()
        .map(|(i, (x, _))| (x, i))
        .collect();
    let z = mapping
        .iter()
        .filter(|(x, _)| x.chars().nth(2).unwrap() == 'Z')
        .map(|(_, i)| *i)
        .mn();
    let mut map = vec![usize::MAX; mapping.len()];
    for (l, left, right) in rest_old
        .iter()
        .copied()
        .map(|(l, (left, right))| (mapping[l], mapping[left], mapping[right]))
    {
        map[l] = left << 16 | right;
    }
    let rest = Arc::new(map);
    let instructions = t.into_iter().map(|l| l == 'L').collect_vec();
    let instructions = Arc::new(instructions);
    let mut recv = vec![];
    let starts = rest_old
        .iter()
        .filter(|s| s.0.ends_with('A'))
        .map(|s| mapping[s.0])
        .collect_vec();
    let p = available_parallelism().unwrap().get().min(starts.len());
    for (i, (s, r)) in repeat_with(|| std::sync::mpsc::channel())
        .take(p)
        .enumerate()
    {
        recv.push(r);
        let rest = rest.clone();
        let instructions = instructions.clone();
        let mut location = starts[i];
        std::thread::spawn(move || {
            let mut found = vec![];
            let mut i = 0;
            let mut j = 0;
            loop {
                i += 1;
                if i & SEND_AFTER == 0 {
                    s.send(found).unwrap();
                    found = vec![];
                }
                if instructions[j] {
                    location = rest[location] >> 16;
                } else {
                    location = rest[location] & 0xFFFF;
                }
                if location >= z {
                    found.push(i);
                }
                j += 1;
                if j == instructions.len() {
                    j = 0;
                }
            }
        });
    }
    const MEASURE_LAST_X: usize = 10;
    let mut start = [Instant::now(); MEASURE_LAST_X];
    for i in 1.. {
        let intersection = recv
            .iter_mut()
            .map(|r| r.recv().unwrap().into_iter().collect::<HashSet<_>>())
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap();
        if !intersection.is_empty() {
            intersection.into_iter().mn().save();
            return;
        }
        println!(
            "Iteration: {}, Iterations/second: {:.0}",
            i * SEND_AFTER,
            ((SEND_AFTER * MEASURE_LAST_X.min(i)) as f64)
                / start[i % MEASURE_LAST_X].elapsed().as_secs_f64()
        );
        start[i % MEASURE_LAST_X] = Instant::now();
    }
}
fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    twop();
}
