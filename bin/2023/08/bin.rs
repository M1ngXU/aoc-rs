#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!(~"\n" > ch >> "\n\n" (| ~" " > id >> " = (" (~"," > id >> ", " ~")" > id >> ")"))[LE]);
    let (t, rest): (Vec<char>, Vec<(&str, (&str, &str))>) = pi!(p);
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

fn two() {
    let p = parser!(~"\n" > ch >> "\n\n" (| ~" " > id >> " = (" (~"," > id >> ", " ~")" > id >> ")"))[LE]);
    let (t, rest): (Vec<char>, Vec<(&str, (&str, &str))>) = pi!(p);
    let mut inst = rest
        .iter()
        .filter(|s| s.0.ends_with('A'))
        .map(|s| s.0)
        .collect_vec();
    let mut cycles = vec![];
    for inst in &inst {
        let mut cycle = 0;
        let mut i = *inst;
        for c in t.iter().copied().cycle() {
            // println!("{inst:?}");
            if i.ends_with('Z') {
                break;
            }
            if c == 'L' {
                i = rest.iter().find(|x| x.0 == i).unwrap().1 .0;
            } else {
                i = rest.iter().find(|x| x.0 == i).unwrap().1 .1;
            }
            cycle += 1;
        }
        cycles.push(cycle);
    }
    cycles
        .into_iter()
        .reduce(|a, b| a * b / gcd(a, b))
        .unwrap()
        .save();
}
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn main() {
    print!("Part 1: ");
    // one();
    print!("Part 2: ");
    two();
}
