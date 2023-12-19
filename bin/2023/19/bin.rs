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
        (
            Vec<Vec<Range<i64>>>,
            Vec<Vec<Range<i64>>>,
            Vec<Vec<Range<i64>>>,
            Vec<Vec<Range<i64>>>,
        ),
    >,
    cur: &'static str,
    xr: Vec<Range<i64>>,
    mr: Vec<Range<i64>>,
    ar: Vec<Range<i64>>,
    sr: Vec<Range<i64>>,
) -> (
    Vec<Vec<Range<i64>>>,
    Vec<Vec<Range<i64>>>,
    Vec<Vec<Range<i64>>>,
    Vec<Vec<Range<i64>>>,
) {
    if let Some((x, m, a, s)) = memo.get(&(cur, xr.clone(), mr.clone(), ar.clone(), sr.clone())) {
        return (x.clone(), m.clone(), a.clone(), s.clone());
    }
    if cur == "R" {
        return (vec![], vec![], vec![], vec![]);
    }
    if cur == "A" {
        return (vec![xr], vec![mr], vec![ar], vec![sr]);
    }
    let (rules_, other_to) = &rules[cur];
    let mut combs = 0;
    let mut other = [xr.clone(), mr.clone(), ar.clone(), sr.clone()];
    let mut dupesx: Vec<Vec<Range<i64>>> = vec![];
    let mut dupesm: Vec<Vec<Range<i64>>> = vec![];
    let mut dupesa: Vec<Vec<Range<i64>>> = vec![];
    let mut dupess: Vec<Vec<Range<i64>>> = vec![];
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
            let (rangesx, rangesm, rangesa, rangess) = comb(
                rules,
                memo,
                to,
                ranges[0].clone(),
                ranges[1].clone(),
                ranges[2].clone(),
                ranges[3].clone(),
            );
            let [x, m, a, s] = ranges;
            memo.insert(
                (to, x, m, a, s),
                (
                    rangesx.clone(),
                    rangesm.clone(),
                    rangesa.clone(),
                    rangess.clone(),
                ),
            );
            dupesx.extend(rangesx);
            dupesm.extend(rangesm);
            dupesa.extend(rangesa);
            dupess.extend(rangess);
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
            let (rangesx, rangesm, rangesa, rangess) = comb(
                rules,
                memo,
                to,
                ranges[0].clone(),
                ranges[1].clone(),
                ranges[2].clone(),
                ranges[3].clone(),
            );
            let [x, m, a, s] = ranges;
            memo.insert(
                (to, x, m, a, s),
                (
                    rangesx.clone(),
                    rangesm.clone(),
                    rangesa.clone(),
                    rangess.clone(),
                ),
            );
            dupesx.extend(rangesx);
            dupesm.extend(rangesm);
            dupesa.extend(rangesa);
            dupess.extend(rangess);
        }
    }
    {
        println!("{other:?}");
        let (rangesx, rangesm, rangesa, rangess) = comb(
            rules,
            memo,
            other_to,
            other[0].clone(),
            other[1].clone(),
            other[2].clone(),
            other[3].clone(),
        );
        let [x, m, a, s] = other;
        memo.insert(
            (other_to, x, m, a, s),
            (
                rangesx.clone(),
                rangesm.clone(),
                rangesa.clone(),
                rangess.clone(),
            ),
        );
        dupesx.extend(rangesx);
        dupesm.extend(rangesm);
        dupesa.extend(rangesa);
        dupess.extend(rangess);
    }
    // let [dx, dm, da, ds] = [dupesx, dupesm, dupesa, dupess].map(|dupes| {
    //     dupes
    //         .into_iter()
    //         .permutations(2)
    //         .flat_map(|v| {
    //             let a = v[0].clone();
    //             let b = v[1].clone();
    //             a.into_iter()
    //                 .flat_map(|r| {
    //                     b.iter()
    //                         .clone()
    //                         .filter_map(|x| x.intersection(&r))
    //                         .collect_vec()
    //                 })
    //                 .collect_vec()
    //         })
    //         .collect_vec()
    // });
    memo.insert(
        (cur, xr, mr, ar, sr),
        (
            dupesx.clone(),
            dupesm.clone(),
            dupesa.clone(),
            dupess.clone(),
        ),
    );
    // let d = [dx, ]
    // let d = [dx, dm, da, ds]
    //     .map(|r| r.into_iter().map(|x| x.to_incl() - x.from_incl() + 1).s())
    //     .p();

    (
        dupesx.clone(),
        dupesm.clone(),
        dupesa.clone(),
        dupess.clone(),
    )
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
    let (x, m, a, s) = comb(
        &rules,
        &mut HashMap::new(),
        "in",
        vec![1..4001],
        vec![1..4001],
        vec![1..4001],
        vec![1..4001],
    );
    let mut total = 0;
    let mut seen: Vec<(
        Vec<Range<i64>>,
        Vec<Range<i64>>,
        Vec<Range<i64>>,
        Vec<Range<i64>>,
    )> = Vec::new();
    for i in 0..x.len() {
        let x = x[i].clone();
        let m = m[i].clone();
        let a = a[i].clone();
        let s = s[i].clone();
        let mut d = 0;
        for (xx, mm, aa, ss) in &seen {
            let x = x
                .iter()
                .flat_map(|r| xx.iter().filter_map(|rr| r.intersection(rr)).collect_vec())
                .collect_vec();
            let m = m
                .iter()
                .flat_map(|r| mm.iter().filter_map(|rr| r.intersection(rr)).collect_vec())
                .collect_vec();
            let a = a
                .iter()
                .flat_map(|r| aa.iter().filter_map(|rr| r.intersection(rr)).collect_vec())
                .collect_vec();
            let s = s
                .iter()
                .flat_map(|r| ss.iter().filter_map(|rr| r.intersection(rr)).collect_vec())
                .collect_vec();
            d += ([x, m, a, s])
                .map(|r| r.into_iter().flat_map(|r| r).collect::<HashSet<_>>().len())
                .p();
        }
        seen.push((x.clone(), m.clone(), a.clone(), s.clone()));
        // seen.iter().map(|(xx, mm, aa, ss)| {
        //     x.iter()
        //         .flat_map(|r| xx.iter().filter_map(|rr| r.intersection(rr)).collect_vec())
        // });
        total += ([x, m, a, s])
            .map(|r| r.into_iter().flat_map(|r| r).collect::<HashSet<_>>().len())
            .p() as i64
            - d as i64;
    }
    total.save();
    // dbg!([x, m, a, s])
    //     .map(|r| r.into_iter().flat_map(|r| r).collect::<HashSet<_>>().len())
    //     .p()
    //     .save();
}

fn two() {}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
