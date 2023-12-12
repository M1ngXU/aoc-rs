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
        let v = v1 + v2;
        memo.insert(state, v);
        return v;
    }
    let mut new_state = state;
    let isdmg = state.set_next_dmg || chars[state.chari] == '#';
    new_state.set_next = false;
    new_state.set_next_dmg = false;
    new_state.last_dmg = isdmg;
    new_state.chari += 1;
    if isdmg {
        new_state.lgroups += 1;
        let v = rec_call(memo, chars, groups, new_state);
        memo.insert(state, v);
        return v;
    } else {
        if state.last_dmg {
            if new_state.groupi < groups.len() && groups[new_state.groupi] == new_state.lgroups {
                new_state.groupi += 1;
                new_state.lgroups = 0;

                let v = rec_call(memo, chars, groups, new_state);
                memo.insert(state, v);
                return v;
            } else {
                return 0;
            }
        } else {
            new_state.last_dmg = isdmg;
            let v = rec_call(memo, chars, groups, new_state);
            memo.insert(state, v);
            return v;
        }
    }
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

    s.par_iter()
        .map(|(c, g)| {
            rec_call(
                &mut HashMap::new(),
                &c,
                &g,
                State::new(0, false, 0, 0, false, false),
            )
        })
        .sum::<usize>()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
}
