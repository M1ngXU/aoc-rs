#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn tilt_north_south(s: &mut Vec<Vec<char>>, dy: isize) {
    (0..s[0].len()).into_iter().for_each(|x| {
        let mut mt = if dy > 0 { s.len() as isize - 1 } else { 0 };
        for i in if dy > 0 {
            (0..s.len()).rev().collect_vec()
        } else {
            (0..s.len()).collect_vec()
        } {
            if s[i][x] == '#' {
                mt = i as isize - dy;
            } else if s[i][x] == 'O' {
                s[i][x] = '.';
                s[mt as usize][x] = 'O';
                mt -= dy;
            }
        }
    });
}

fn tilt_east_west(s: &mut Vec<Vec<char>>, dx: isize) {
    s.iter_mut().for_each(|row| {
        let mut mt = if dx > 0 { row.len() as isize - 1 } else { 0 };
        for i in if dx > 0 {
            (0..row.len()).rev().collect_vec()
        } else {
            (0..row.len()).collect_vec()
        } {
            if row[i] == '#' {
                mt = i as isize - dx;
            } else if row[i] == 'O' {
                row[i] = '.';
                row[mt as usize] = 'O';
                mt -= dx;
            }
        }
    });
}

fn one() {
    let p = parser!((|ch)[LE]);
    let mut s = pi!(p);

    tilt_north_south(&mut s, -1);
    s.ii()
        .filter(|(x, y)| s[*y][*x] == 'O')
        .map(|(_, y)| s.len() - y)
        .s()
        .save();
}

fn two() {
    let p = parser!((|ch)[LE]);
    let mut s = pi!(p);
    let mut seen = HashMap::new();

    for i in 0..1_000_000_000 {
        if let Some(x) = seen.get(&s) {
            let diff = i - x;
            let todo = (1_000_000_000 - i) % diff;
            let res = x + todo;
            s = seen.into_iter().find(|(_, v)| *v == res).unwrap().0;
            break;
        }
        seen.insert(s.clone(), i);
        tilt_north_south(&mut s, -1);
        tilt_east_west(&mut s, -1);
        tilt_north_south(&mut s, 1);
        tilt_east_west(&mut s, 1);
    }
    s.ii()
        .filter(|(x, y)| s[*y][*x] == 'O')
        .map(|(_, y)| s.len() - y)
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
