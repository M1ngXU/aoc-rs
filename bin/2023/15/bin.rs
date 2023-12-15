#![feature(iter_repeat_n, ascii_char, iter_map_windows, iter_from_coroutine)]

use std::hash::Hash;

use aoc_rs::prelude::*;

#[derive(Clone, Debug)]
struct S(String);
impl Hash for S {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(h(&self.0) as u8);
    }
}
impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        h(&self.0) == h(&other.0)
    }
}
impl Eq for S {}

fn h(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h = (17 * (h + c.as_ascii().unwrap().to_u8() as usize)) % 256;
    }
    h
}

fn one() {
    let p = parser!((| id)[","]);
    let s = pi!(p);
    s.into_iter().map(h).s().save()
}

fn two() {
    let p = parser!((| id)[","]);
    let s = pi!(p);

    let mut boxes = HashMap::<S, LinkedHashMap<String, usize>>::new();
    for l in s {
        if l.ends_with('-') {
            let label = S(l[..l.len() - 1].to_string());
            boxes.entry(label.clone()).or_default().remove(&label.0);
        } else {
            let (label, a) = l.split_once('=').unwrap();
            let a = a.parse::<usize>().unwrap();
            let label_hash = S(label.to_string());
            *boxes
                .entry(label_hash.clone())
                .or_default()
                .entry(label.to_string())
                .or_default() = a;
        }
    }
    boxes
        .into_iter()
        .map(|(i, l)| {
            l.into_iter()
                .enumerate()
                .map(|(j, (_, a))| (h(&i.0) + 1) * (j + 1) * a)
                .s()
        })
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
