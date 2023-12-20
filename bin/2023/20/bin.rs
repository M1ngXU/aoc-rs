#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| (~" " > id) >> " -> " (| id)[", "])[LE]);
    let s: Vec<(&str, Vec<&str>)> = pi!(p);
    let mut flips = HashMap::new();
    let inputs_for: HashMap<&str, HashSet<&str>> = s
        .iter()
        .filter(|(c, _)| c.starts_with("&"))
        .map(|(c, _)| {
            (
                &(*c)[1..],
                s.iter()
                    .filter(|(_, to)| to.contains(&&c[1..]))
                    .map(|(c, _)| *c)
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut conj: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut total = 0;
    let mut clow = 0;
    let mut chigh = 0;
    for _ in 0..1 {
        for (next, to) in &s {
            match *next {
                "broadcaster" => {
                    clow += 1;
                    for t in to {
                        match t {
                            f if s.iter().find(|(x, _)| x == &format!("%{f}")).is_some() => {
                                let x: &mut bool = flips.entry(*f).or_default();
                                *x = !*x;
                                clow += 1;
                            }
                            f if s.iter().find(|(x, _)| x == &format!("&{f}")).is_some() => {
                                // let x = conj.entry(f).or_default();
                                // x.insert("broadcaster");
                                clow += 1;
                            }
                            _ => {
                                clow += 1;
                            }
                        }
                    }
                }
                f if f.starts_with("%") => {
                    let h = flips.get(&f[1..]).copied().unwrap_or(false);
                    println!("{h}");
                    for t in to {
                        match t {
                            f if s.iter().find(|(x, _)| x == &format!("%{f}")).is_some() => {
                                if !h {
                                    let x: &mut bool = flips.entry(*f).or_default();
                                    *x = !*x;
                                    clow += 1;
                                } else {
                                    chigh += 1;
                                    // ignore pulse?
                                }
                            }
                            f if s.iter().find(|(x, _)| x == &format!("&{f}")).is_some() => {
                                if h {
                                    let x = conj.entry(f).or_default();
                                    x.insert(t);
                                    chigh += 1;
                                }
                            }
                            _ => {
                                if h {
                                    chigh += 1;
                                } else {
                                    clow += 1;
                                }
                            }
                        }
                    }
                }
                f if f.starts_with("&") => {
                    let h = if let Some(x) = conj.get(&(*f)[1..]) {
                        if x == &inputs_for[&f[1..]] {
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    };

                    for t in to {
                        match t {
                            f if s.iter().find(|(x, _)| x == &format!("%{f}")).is_some() => {
                                if !h {
                                    let x: &mut bool = flips.entry(*f).or_default();
                                    *x = !*x;
                                    clow += 1;
                                } else {
                                    chigh += 1;
                                    // ignore pulse?
                                }
                            }
                            f if s.iter().find(|(x, _)| x == &format!("&{f}")).is_some() => {
                                if h {
                                    let x = conj.entry(f).or_default();
                                    x.insert(t);
                                    chigh += 1;
                                }
                            }
                            _ => {
                                if h {
                                    chigh += 1;
                                } else {
                                    clow += 1;
                                }
                            }
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    panic!("{clow}|{chigh}");
    (clow * chigh).save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
