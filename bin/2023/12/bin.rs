#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    chari: usize,
    last_dmg: bool,
    groupi: usize,
    lgroups: isize,
    set_next: bool,
    set_next_dmg: bool,
}
impl State {
    fn new(
        chari: usize,
        last_dmg: bool,
        groupi: usize,
        lgroups: isize,
        set_next: bool,
        set_next_dmg: bool,
    ) -> Self {
        Self {
            chari,
            last_dmg,
            groupi,
            lgroups,
            set_next,
            set_next_dmg,
        }
    }
}

fn rec_call(
    memo: &mut HashMap<State, usize>,
    chars: &Vec<char>,
    groups: &Vec<isize>,
    state: State,
) -> usize {
    if state.chari >= chars.len() {
        return (!state.last_dmg && state.groupi == groups.len()
            || state.last_dmg
                && state.groupi == groups.len() - 1
                && groups[state.groupi] == state.lgroups) as usize;
    }
    if let Some(v) = memo.get(&state) {
        return *v;
    }
    if !state.set_next && chars[state.chari] == '?' {
        let mut s2 = state;
        s2.set_next = true;
        let v1 = rec_call(
            memo,
            chars,
            groups,
            State {
                set_next_dmg: false,
                ..s2
            },
        );
        let v2 = rec_call(
            memo,
            chars,
            groups,
            State {
                set_next_dmg: true,
                ..s2
            },
        );
        memo.insert(state, v1 + v2);
        return v1 + v2;
    }
    let mut new_state = state;
    let isdmg = state.set_next_dmg || chars[state.chari] == '#';
    new_state.set_next = false;
    new_state.set_next_dmg = false;
    new_state.last_dmg = isdmg;
    new_state.chari += 1;
    if isdmg {
        new_state.lgroups += 1;
    } else {
        if state.last_dmg {
            if new_state.groupi < groups.len() && groups[new_state.groupi] == new_state.lgroups {
                new_state.groupi += 1;
                new_state.lgroups = 0;
            } else {
                return 0;
            }
        } else {
            new_state.last_dmg = isdmg;
        }
    }
    let v = rec_call(memo, chars, groups, new_state);
    memo.insert(state, v);
    v
}

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
                dp[0][i][0] = [1, 1];
            }
            for j in 1..=counts.len() {
                for i in 1..=chars.len() {
                    if chars[i - 1] != '.' {
                        let j_ = counts[j - 1] as usize;
                        if i >= j_ {
                            if chars[i - j_ + 1 - 1..=i - 1].iter().all(|&c| c != '.')
                                && (i - 1 + 1 == j_ || chars[i - j_ - 1] != '#')
                            {
                                let dmgsb = chars[..i - j_ + 1 - 1]
                                    .iter()
                                    .filter(|&c| c == &'#')
                                    .count();
                                let dmgse = chars[..=i - 1].iter().filter(|&c| c == &'#').count();
                                dp[j][i][dmgse][0] = dp[j - 1][i - j_][dmgsb][1];
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

    // s.par_iter()
    //     .map(|(c, g)| {
    //         rec_call(
    //             &mut HashMap::new(),
    //             &c,
    //             &g,
    //             State::new(0, false, 0, 0, false, false),
    //         )
    //     })
    //     .sum::<usize>()
    //     .save();
}

fn main() {
    print!("Part 1: ");
    one();
}
