use aoc_rs::prelude::*;

fn one() {
    let p = sb(LLE, sb(LE, id));
    let s = pi!(p);
    let stacks = s[0].clone();
    let instructions = s[1].clone();

    let mut stacks = stacks
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|s| {
            separated_list0(
                tag::<_, _, nom::error::Error<_>>(" "),
                alt((delimited(tag("["), take(1_usize), tag("]")), tag("   "))),
            )(s)
            .p()
            .into_iter()
            .map(|s| s.chars().next().unwrap())
            .map(|c| (c != ' ').then_some(c))
            .collect_vec()
        })
        .collect_vec();
    stacks = stacks.t();
    let mut stacks = stacks
        .into_iter()
        .map(|v| v.into_iter().rev().filter_map(identity).collect_vec())
        .collect_vec();

    for instruction in instructions {
        let (_, q, _, f, _, t) =
            tuple((tag("move "), pn, tag(" from "), pn, tag(" to "), pn))(instruction).p();
        for _ in 0..q {
            let c = stacks[f as usize - 1].pop().unwrap();
            stacks[t as usize - 1].push(c);
        }
    }
    stacks
        .into_iter()
        .map(|s| s.last().unwrap().clone())
        .collect::<String>()
        .save();
}

fn two() {
    let p = sb(LLE, sb(LE, id));
    let s = pi!(p);
    let stacks = s[0].clone();
    let instructions = s[1].clone();

    let mut stacks = stacks
        .into_iter()
        .rev()
        .skip(1)
        .rev()
        .map(|s| {
            separated_list0(
                tag::<_, _, nom::error::Error<_>>(" "),
                alt((delimited(tag("["), take(1_usize), tag("]")), tag("   "))),
            )(s)
            .p()
            .into_iter()
            .map(|s| s.chars().next().unwrap())
            .map(|c| (c != ' ').then_some(c))
            .collect_vec()
        })
        .collect_vec();
    stacks = stacks.t();
    let mut stacks = stacks
        .into_iter()
        .map(|v| v.into_iter().rev().filter_map(identity).collect_vec())
        .collect_vec();

    for instruction in instructions {
        let (_, q, _, f, _, t) =
            tuple((tag("move "), pn, tag(" from "), pn, tag(" to "), pn))(instruction).p();
        let s = stacks[f as usize - 1].len();
        let c = &stacks[f as usize - 1][s - q as usize..].to_vec();
        stacks[t as usize - 1].extend(c);
        stacks[f as usize - 1].truncate(s - q as usize);
    }
    stacks
        .into_iter()
        .map(|s| s.last().unwrap().clone())
        .collect::<String>()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
