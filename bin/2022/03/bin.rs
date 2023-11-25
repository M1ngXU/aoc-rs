use aoc_rs::prelude::*;

fn one() {
    let p = sb(
        LE,
        map(ch, |c| -> (_, _) {
            c.chunks(c.len() / 2)
                .map(|c| c.iter().copied().collect::<HashSet<_>>())
                .collect_tuple()
                .unwrap()
        }),
    );
    let s = pi!(p);
    s.into_iter()
        .map(|(f, s)| match f.intersection(&s).next().unwrap() {
            c if ('a'..='z').contains(c) => *c as u8 - b'a' + 1,
            c => *c as u8 - b'A' + 1 + 26,
        })
        .cuz()
        .s()
        .save();
}

fn two() {
    let p = sb(LE, ch);
    let s = pi!(p);
    s.into_iter()
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .tuples()
        .map(|(a, b, c)| {
            match a
                .intersection(&b.intersection(&c).copied().collect::<HashSet<_>>())
                .next()
                .unwrap()
            {
                c if ('a'..='z').contains(c) => *c as u8 - b'a' + 1,
                c => *c as u8 - b'A' + 1 + 26,
            }
        })
        .cuz()
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
