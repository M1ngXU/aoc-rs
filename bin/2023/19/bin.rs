#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use std::ops::Range;

use aoc_rs::prelude::*;

fn comb(
    rules: &HashMap<&'static str, (Vec<(&'static str, &'static str)>, &'static str)>,
    memo: &mut HashMap<
        (
            &'static str,
            Vec<Range<i64>>,
            Vec<Range<i64>>,
            Vec<Range<i64>>,
            Vec<Range<i64>>,
        ),
        i64,
    >,
    cur: &'static str,
    xr: Vec<Range<i64>>,
    mr: Vec<Range<i64>>,
    ar: Vec<Range<i64>>,
    sr: Vec<Range<i64>>,
    xdr: Vec<Range<i64>>,
    mdr: Vec<Range<i64>>,
    adr: Vec<Range<i64>>,
    sdr: Vec<Range<i64>>,
) -> (
    i64,
    Vec<Range<i64>>,
    Vec<Range<i64>>,
    Vec<Range<i64>>,
    Vec<Range<i64>>,
) {
    if let Some(r) = memo.get(&(cur, xr.clone(), mr.clone(), ar.clone(), sr.clone())) {
        return (*r, xr, mr, ar, sr);
    }
    if cur == "R" {
        return (0, xr, mr, ar, sr);
    }
    if cur == "A" {
        return (
            [xr.clone(), mr.clone(), ar.clone(), sr.clone()]
                .into_iter()
                .zip([xdr.clone(), mdr.clone(), adr.clone(), sdr.clone()])
                .map(|(r, d)| {
                    r.into_iter()
                        .flat_map(|x| {
                            d.iter().fold(vec![x], |x, rr| {
                                x.into_iter()
                                    .map(|x| x.setminus(rr))
                                    .flat_map(|(a, b)| {
                                        if a.is_some() {
                                            vec![a.unwrap()]
                                        } else if b.is_some() {
                                            vec![b.unwrap()]
                                        } else if a.is_some() && b.is_some() {
                                            vec![a.unwrap(), b.unwrap()]
                                        } else {
                                            vec![]
                                        }
                                    })
                                    .map(|r| r.from_incl()..r.to_incl() + 1)
                                    .collect_vec()
                            })
                        })
                        .map(|x| x.end - x.start)
                        .s()
                })
                .p(),
            xr,
            mr,
            ar,
            sr,
        );
    }
    let (rules_, other_to) = &rules[cur];
    let mut combs = 0;
    let mut other = [xr.clone(), mr.clone(), ar.clone(), sr.clone()];
    let mut dupesx = vec![];
    let mut dupesm = vec![];
    let mut dupesa = vec![];
    let mut dupess = vec![];
    for (rule, to) in rules_ {
        if rule.contains('<') {
            let (var, num) = rule.split_once('<').unwrap();
            let num = num.parse::<i64>().unwrap();
            let mut ranges = [xr.clone(), mr.clone(), ar.clone(), sr.clone()];
            let i = ["x", "m", "a", "s"]
                .into_iter()
                .position(|x| x == var)
                .unwrap();
            ranges[i] = ranges[i]
                .iter()
                .cloned()
                .filter(|x| x.start < num as i64)
                .map(|x| x.start..x.end.min(num))
                .collect_vec();
            other[i] = other[i]
                .iter()
                .cloned()
                .filter(|x| x.end - 1 >= num)
                .map(|x| x.start.max(num)..x.end)
                .collect_vec();
            let (out, rangesx, rangesm, rangesa, rangess) = comb(
                rules,
                memo,
                to,
                ranges[0].clone(),
                ranges[1].clone(),
                ranges[2].clone(),
                ranges[3].clone(),
                dupesx.clone(),
                dupesm.clone(),
                dupesa.clone(),
                dupess.clone(),
            );
            let [x, m, a, s] = ranges;
            memo.insert((to, x, m, a, s), out);
            dupesx.push(rangesx);
            dupesm.push(rangesm);
            dupesa.push(rangesa);
            dupess.push(rangess);
            combs += out;
        } else {
            let (var, num) = rule.split_once('>').unwrap();
            let num = num.parse::<i64>().unwrap();
            let mut ranges = [xr.clone(), mr.clone(), ar.clone(), sr.clone()];
            let i = ["x", "m", "a", "s"]
                .into_iter()
                .position(|x| x == var)
                .unwrap();
            other[i] = other[i]
                .iter()
                .cloned()
                .filter(|x| x.start <= num as i64)
                .map(|x| x.start..x.end.min(num + 1))
                .collect_vec();
            ranges[i] = ranges[i]
                .iter()
                .cloned()
                .filter(|x| x.end > num + 1)
                .map(|x| x.start.max(num + 1)..x.end)
                .collect_vec();
            let (out, rangesx, rangesm, rangesa, rangess) = comb(
                rules,
                memo,
                to,
                ranges[0].clone(),
                ranges[1].clone(),
                ranges[2].clone(),
                ranges[3].clone(),
                dupesx.clone(),
                dupesm.clone(),
                dupesa.clone(),
                dupess.clone(),
            );
            let [x, m, a, s] = ranges;
            memo.insert((to, x, m, a, s), out);
            dupesx.push(rangesx);
            dupesm.push(rangesm);
            dupesa.push(rangesa);
            dupess.push(rangess);
            combs += out;
        }
    }
    {
        println!("{other:?}");
        let (out, rangesx, rangesm, rangesa, rangess) = comb(
            rules,
            memo,
            other_to,
            other[0].clone(),
            other[1].clone(),
            other[2].clone(),
            other[3].clone(),
            dupesx.clone(),
            dupesm.clone(),
            dupesa.clone(),
            dupess.clone(),
        );
        dupesx.push(rangesx);
        dupesm.push(rangesm);
        dupesa.push(rangesa);
        dupess.push(rangess);
        combs += out;
        let [x, m, a, s] = other;
        memo.insert((other_to, x, m, a, s), out);
    }
    let [dx, dm, da, ds] = [dupesx, dupesm, dupesa, dupess].map(|dupes| {
        dupes
            .into_iter()
            .permutations(2)
            .flat_map(|v| {
                let a = v[0].clone();
                let b = v[1].clone();
                a.into_iter()
                    .flat_map(|r| {
                        b.iter()
                            .clone()
                            .filter_map(|x| x.intersection(&r))
                            .collect_vec()
                    })
                    .collect_vec()
            })
            .collect_vec()
    });
    memo.insert((cur, xr, mr, ar, sr), combs);
    // let d = [dx, ]
    // let d = [dx, dm, da, ds]
    //     .map(|r| r.into_iter().map(|x| x.to_incl() - x.from_incl() + 1).s())
    //     .p();
    (combs - d, vec![], vec![], vec![], vec![])
}

fn one() {
    let p = parser!(id);
    // let p = parser!((| ch)[LE]);
    // let p = parser!((| pns)[LE]);
    // let p = parser!((| pds)[LE]);
    // let p = parser!((| ~" " > id >> " " ~" " > (| ms0 << id)["->"])[LE]);
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
    // let mut total = 0;
    // for x in &b {
    //     let part = &x[1..x.len() - 1];
    //     let part = part
    //         .split(",")
    //         .map(|x| {
    //             x.split_once("=")
    //                 .map(|(a, b)| (a, b.parse::<i64>().unwrap()))
    //                 .unwrap()
    //         })
    //         .collect::<HashMap<_, _>>();
    //     let mut current = "in";
    //     'outer: loop {
    //         if current == "A" || current == "R" {
    //             if current == "A" {
    //                 total += part["x"] + part["m"] + part["a"] + part["s"];
    //             }
    //             break;
    //         }
    //         let (rules, other) = &rules[current];
    //         for (rule, to) in rules {
    //             if rule.contains('<') {
    //                 let (var, num) = rule.split_once('<').unwrap();
    //                 let num = num.parse::<i64>().unwrap();
    //                 if part.contains_key(var) && part[var] < num {
    //                     current = to;
    //                     continue 'outer;
    //                 }
    //             } else {
    //                 let (var, num) = rule.split_once('>').unwrap();
    //                 let num = num.parse::<i64>().unwrap();
    //                 if part.contains_key(var) && part[var] > num {
    //                     current = to;
    //                     continue 'outer;
    //                 }
    //             }
    //         }
    //         current = other;
    //     }
    // }
    // total.save();
    comb(
        &rules,
        &mut HashMap::new(),
        "in",
        vec![1..4001],
        vec![1..4001],
        vec![1..4001],
        vec![1..4001],
    )
    .0
    .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
