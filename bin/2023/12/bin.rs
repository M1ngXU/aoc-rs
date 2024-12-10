#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

const REPEATS: usize = 5;

fn one() {
    let p = parser!((| ~" " > ch >> " " pns)[LE]);
    let s = pi!(p);
    let s = s
        .into_iter()
        .map(|(c, g)| {
            (
                c.iter()
                    .copied()
                    .chain(once('?'))
                    .cycle()
                    .take(c.len() * REPEATS + REPEATS - 1)
                    .collect_vec(),
                g.iter()
                    .copied()
                    .cycle()
                    .take(g.len() * REPEATS)
                    .collect_vec(),
            )
        })
        .collect_vec();

    s.into_par_iter()
        .map(|(chars, counts)| {
            let dmgs = chars.iter().filter(|x| x == &&'#').count();
            let mut dp = vec![vec![vec![[0, 0]; dmgs + 1]; chars.len() + 1]; counts.len() + 1];
            for i in 0..=chars.len() {
                dp[0][i][0][1] = 1;
            }
            for j in 1..=counts.len() {
                for i in 1..=chars.len() {
                    if chars[i - 1] != '.' {
                        let j_ = counts[j - 1] as usize;
                        if i >= j_ {
                            if chars[i - j_..=i - 1].iter().all(|&c| c != '.')
                                && (i == j_ || chars[i - j_ - 1] != '#')
                            {
                                dp[j][i][chars[..=i - 1].iter().filter(|&c| c == &'#').count()]
                                    [0] = dp[j - 1][i - j_]
                                    [chars[..i - j_].iter().filter(|&c| c == &'#').count()][1];
                            }
                        }
                    }
                    if chars[i - 1] != '#' {
                        for d in 0..=dmgs {
                            dp[j][i][d][1] = dp[j][i - 1][d].s();
                        }
                    }
                }
            }
            dp[counts.len()][chars.len()][dmgs].into_iter().s()
        })
        .sum::<usize>()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
}
