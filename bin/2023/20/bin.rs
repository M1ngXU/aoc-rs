#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn one() {
    let p = parser!((| (~" " > id) >> " -> " (| id)[", "])[LE]);
    let input: Vec<(&str, Vec<&str>)> = pi!(p);
    let mut flips = HashMap::new();
    let mut conj: HashMap<&str, HashMap<&str, bool>> = HashMap::new();
    let inputs_for: HashMap<&str, HashSet<&str>> = input
        .iter()
        .filter(|(c, _)| c.starts_with("&"))
        .map(|(c, _)| {
            (
                *c,
                input
                    .iter()
                    .filter(|(_, to)| to.contains(&&c[1..]))
                    .map(|(c, _)| *c)
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut dependencies = HashMap::new();
    for (from, _) in &input {
        match *from {
            "broadcast" => {}
            f if f.starts_with("%") => {
                let f2 = &f[1..];
                dependencies.insert(
                    f,
                    input
                        .iter()
                        .filter(|(_, to)| to.contains(&f2))
                        .map(|(c, _)| *c)
                        .collect::<HashSet<_>>(),
                );
            }
            f if f.starts_with("&") => {
                let f2 = &f[1..];
                dependencies.insert(
                    f,
                    input
                        .iter()
                        .filter(|(_, to)| to.contains(&f2))
                        .map(|(c, _)| *c)
                        .collect::<HashSet<_>>(),
                );
            }
            "rx" => {
                dependencies.insert(
                    "rx",
                    input
                        .iter()
                        .filter(|(_, to)| to.contains(&"rx"))
                        .map(|(c, _)| *c)
                        .collect::<HashSet<_>>(),
                );
            }
            _ => {}
        }
    }
    let mut cycles: HashMap<&str, HashSet<i64>> = HashMap::new();
    cycles.insert("broadcaster", HashSet::from([1i64]));
    // loop {
    //     let mut changed = false;
    //     'outer: for (d, deps) in &dependencies {
    //         println!("{d}");
    //         if d.starts_with("&") {
    //             let mut nums = vec![];
    //             for dd in deps {
    //                 if let Some(x) = cycles.get(dd) {
    //                     nums.push(x.clone());
    //                 } else {
    //                     continue 'outer;
    //                 }
    //             }
    //             let old = cycles.get(d).map(|x| x.clone());
    //             let new = nums
    //                 .into_iter()
    //                 .map(|x| x.into_iter().collect_vec())
    //                 .multi_cartesian_product()
    //                 .map(|x| x.into_iter().reduce(|a, b| a.lcm(b)).unwrap())
    //                 .collect();
    //             if let Some(x) = cycles.insert(d, new) {
    //                 if Some(x) != old {
    //                     println!("new &");
    //                     changed = true;
    //                 }
    //             } else {
    //                 println!("inserted &");
    //                 changed = true;
    //             }
    //         } else if d.starts_with("%") {
    //             let mut nums = vec![];
    //             for dd in deps {
    //                 if let Some(x) = cycles.get(dd) {
    //                     nums.push(x.clone());
    //                 }
    //             }
    //             if nums.is_empty() {
    //                 continue 'outer;
    //             }
    //             println!("{nums:?}");
    //             let old = cycles.get(d).map(|x| x.clone());

    //             if let Some(x) = cycles.insert(d, nums.into_iter().flatten().collect()) {
    //                 if Some(x) != old {
    //                     println!("new %");
    //                     changed = true;
    //                 }
    //             } else {
    //                 println!("inserted %");
    //                 changed = true;
    //             }
    //         } else {
    //             println!("{d}");
    //         }
    //     }
    //     if !changed {
    //         break;
    //     }
    // }
    // println!("{cycles:?}");
    // panic!("{dependencies:?}");
    // let mut total = 0;
    let mut clow = 0;
    let mut chigh = 0;
    // let mut repeats = HashMap::new();
    // repeats.insert("broadcaster", 1);
    // loop {
    //     let mut changed = false;

    //     'outer: for (d, deps) in &dependencies {
    //         if deps.contains(&"broadcaster") {
    //             println!("{d}|{deps:?}");
    //         }
    //         if repeats.contains_key(d) {
    //             continue;
    //         }
    //         if d.starts_with("&") {
    //             let mut nums = vec![];
    //             for dd in deps {
    //                 if let Some(x) = repeats.get(dd) {
    //                     nums.push(*x);
    //                 } else {
    //                     continue 'outer;
    //                 }
    //             }
    //             changed = true;
    //             repeats.insert(d, nums.into_iter().reduce(|a, b| a.lcm(b)).unwrap());
    //         }
    //         if d.starts_with("%") {
    //             let mut nums = vec![];
    //             for dd in deps {
    //                 if let Some(x) = repeats.get(dd) {
    //                     nums.push(*x);
    //                 } else {
    //                     continue 'outer;
    //                 }
    //             }
    //             println!("{d}");
    //         }
    //     }

    //     if !changed {
    //         break;
    //     }
    // }
    // panic!();
    // let mut last_on = HashMap::new();
    println!(
        "{}",
        [3761u64, 3797, 4079, 3919]
            .into_iter()
            .reduce(|a, b| a.lcm(b))
            .unwrap()
    );
    for i in 1..=10000000 {
        // if i % 100 == 0 {
        //     println!("{i}");
        // }
        // if flips.get("%lh").is_some_and(|x| *x) {
        //     println!("lh: ON!: {i}");
        // }
        // if flips.get("%fk").is_some_and(|x| *x) {
        //     println!("fk: ON!: {i}");
        // }
        // if flips.get("%qj").is_some_and(|x| *x) {
        //     println!("kh: ON!: {i}");
        // }
        // for (f, t) in &flips {
        //     if *t {
        //         if !last_on.contains_key(f) {
        //             last_on.insert(*f, i - 1);
        //         } else {
        //             println!("{f}|{i}|{}", last_on[f]);
        //             assert!(i % last_on[f] == 0);
        //         }
        //     }
        // }
        // println!(
        //     "{:?} ({})",
        //     last_on
        //         .iter()
        //         .sorted_by_key(|(_, v)| **v)
        //         .collect::<Vec<_>>(),
        //     last_on.len()
        // );
        for x in &[
            "%zl", "%hx", "%cm", "%jx", "%zt", "%bn", "%fj", "%fq", "%vm", "%cn", "%np",
        ][10..11]
        {
            if flips.get(x).is_some_and(|x| *x) {
                // println!("{x}: {:b}", i - 1)
            }
        }
        let mut todo = VecDeque::from([("broadcaster", false)]);
        // let mut visited = Vec::new();
        while let Some((next, high)) = todo.pop_front() {
            // if next == "rx" && !high {
            //     i.save();
            //     panic!();
            // }
            if high {
                chigh += 1;
            } else {
                clow += 1;
            }
            // if visited.contains(&(flips.clone(), conj.clone(), next, high)) {
            //     break;
            // }
            // visited.push((flips.clone(), conj.clone(), next, high));
            if input
                .iter()
                .find(|(s, _)| s == &next || &(*s)[1..] == next)
                .is_none()
            {
                continue;
            }
            let (s, to) = input
                .iter()
                .find(|(s, _)| s == &next || &(*s)[1..] == next)
                .unwrap();
            match *s {
                "broadcaster" => {
                    for t in to {
                        todo.push_back((t, high));
                    }
                }
                f if f.starts_with("%") => {
                    // println!("{flips:?}");
                    let h = if !high {
                        let x: &mut bool = flips.entry(f).or_default();
                        *x = !*x;
                        // println!("{x}");
                        *x
                    } else {
                        true
                    };
                    for t in to {
                        if input.iter().find(|(x, _)| x == &format!("&{t}")).is_some() {
                            let x = conj.entry(t).or_default();
                            if h {
                                x.insert(f, h);
                            } else {
                                x.remove(f);
                            }
                        }
                        if h && input.iter().find(|(x, _)| x == &format!("%{t}")).is_some() {
                            chigh += 1;
                            continue;
                        }
                        todo.push_back((t, h));
                    }
                }
                f if f.starts_with("&") => {
                    // println!("{inputs_for:?}|{f}|{:?}|{conj:?}", conj.get(&f));
                    let h = if let Some(x) = conj.get(&f[1..]) {
                        if inputs_for[&f].iter().all(|xx| x.contains_key(xx)) {
                            if f == "&tg" {
                                println!("{:b}|{i}", i);
                            }
                            // conj.get_mut(&f[1..]).unwrap().clear();
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    };

                    for t in to {
                        if h && input.iter().find(|(x, _)| x == &format!("&{t}")).is_some() {
                            let x = conj.entry(t).or_default();
                            if h {
                                x.insert(f, h);
                            } else {
                                x.remove(f);
                            }
                        }
                        if h && input.iter().find(|(x, _)| x == &format!("%{t}")).is_some() {
                            chigh += 1;
                            continue;
                        }
                        todo.push_back((t, h));
                    }
                }
                _ => unreachable!(),
            }
        }
        // println!("{flips:?}|{conj:?}");
        // println!("======={clow}|{chigh}");
    }
    (clow * chigh).save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
