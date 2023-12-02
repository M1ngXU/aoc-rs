#![feature(iter_map_windows)]

use std::cmp::Ordering;

use aoc_rs::prelude::*;

fn cmp(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a
            .as_f64()
            .unwrap()
            .partial_cmp(&b.as_f64().unwrap())
            .unwrap(),
        (Value::Array(a), Value::Array(b)) => {
            for (a, b) in a.iter().zip(b) {
                let c = cmp(a, b);
                if c.is_ne() {
                    return c;
                }
            }
            a.len().cmp(&b.len())
        }
        (Value::Array(_), Value::Number(b)) => cmp(a, &json!([b.clone()])),
        (Value::Number(a), Value::Array(_)) => cmp(&json!([a.clone()]), b),
        _ => todo!(),
    }
}

fn one() {
    let p = _sb("\n\n", pair(pu("\n", pjs::<Value>), pjs::<Value>));
    let s = pi!(p);
    s.into_iter()
        .enumerate()
        .filter(|(_, (a, b))| cmp(a, b).is_lt())
        .map(|(i, _)| i + 1)
        .s()
        .save();
}

fn two() {
    let p = _sb("\n\n", pair(pu("\n", pjs), pjs));
    let s = pi!(p);
    let d1 = json![[[2]]];
    let d2 = json![[[6]]];
    let s = s
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .chain([d1.clone(), d2.clone()])
        .sorted_by(cmp)
        .collect::<Vec<_>>();
    s.into_iter()
        .enumerate()
        .filter(|(_, a)| a == &d1 || a == &d2)
        .map(|(i, _)| i + 1)
        .p()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
