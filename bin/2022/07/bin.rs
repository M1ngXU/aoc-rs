#![feature(iter_map_windows)]

use std::path::PathBuf;

use aoc_rs::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
enum FS {
    Dir(HashMap<String, FS>),
    File(isize),
}

fn one() {
    // add newline to the beginning of the file
    let p = sb(t!(@ "$"), sb(le, id));
    let s = pi!(p);
    let mut cwd = PathBuf::from("/");
    let mut dir = FS::Dir(HashMap::new());
    for command in s.into_iter().skip(1) {
        if command.len() == 1 {
            let p = preceded(t!("cd "), take_while(|_| true))(command[0]).p();
            match p {
                "/" => cwd = PathBuf::from("/"),
                ".." => {
                    cwd.pop();
                }
                s => cwd.push(s),
            }
        } else {
            let s = command[1..].join("\n");
            let (s, output) = sb(
                le,
                tuple((
                    alt((
                        map(map_parser(take_until(" "), pn), FS::File),
                        map(tag("dir"), |_| FS::Dir(HashMap::new())),
                    )),
                    preceded(tag(" "), take_while(|_| true)),
                )),
            )(&s)
            .unwrap();
            assert!(s.is_empty());
            let mut cur_dir = &mut dir;
            for c in cwd.components() {
                cur_dir = match cur_dir {
                    FS::Dir(d) => d
                        .entry(c.as_os_str().to_str().unwrap().to_string())
                        .or_insert(FS::Dir(HashMap::new())),
                    _ => unreachable!(),
                }
            }
            match cur_dir {
                FS::Dir(d) => {
                    for (fs, name) in output {
                        d.insert(name.to_string(), fs);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    folder_sizes(&dir)
        .into_iter()
        .filter(|s| s < &100_000)
        .s()
        .save();
}

fn folder_sizes(fs: &FS) -> Vec<isize> {
    match fs {
        FS::Dir(d) => {
            let mut sum = 0;
            let mut v = d
                .values()
                .flat_map(|f| match f {
                    FS::Dir(_) => {
                        let a = folder_sizes(f);
                        sum += a.iter().copied().s();
                        a
                    }
                    FS::File(s) => {
                        sum += *s;
                        vec![]
                    }
                })
                .collect_vec();
            v.push(sum);
            v
        }
        FS::File(s) => vec![*s],
    }
}

fn two() {
    // add newline to the beginning of the file
    let p = sb(t!(@ "$"), sb(le, id));
    let s = pi!(p);
    let mut cwd = PathBuf::from("/");
    let mut dir = HashMap::new();
    for command in s.into_iter().skip(1) {
        if command.len() == 1 {
            let p = preceded(tag("cd "), take_while(|_| true))(command[0]).p();
            match p {
                "/" => cwd = PathBuf::from("/"),
                ".." => {
                    cwd.pop();
                }
                s => cwd.push(s),
            }
        } else {
            let s = command[1..].join("\n");
            let (s, output) = sb(
                le,
                tuple((
                    alt((
                        map(map_parser(take_until(" "), pn), FS::File),
                        map(tag("dir"), |_| FS::Dir(HashMap::new())),
                    )),
                    preceded(tag(" "), take_while(|_| true)),
                )),
            )(&s)
            .unwrap();
            assert!(s.is_empty());
            let mut cur_dir = &mut dir;
            for c in cwd.components() {
                let s = cur_dir
                    .entry(c.as_os_str().to_str().unwrap().to_string())
                    .or_insert(FS::Dir(HashMap::new()));
                match s {
                    FS::Dir(d) => cur_dir = d,
                    _ => unreachable!(),
                }
            }
            for (fs, name) in output {
                cur_dir.insert(name.to_string(), fs);
            }
        }
    }
    let f = folder_sizes2(&dir).into_iter().collect_vec();
    let req = 30000000 - (70000000 - dir_size(&dir));
    f.into_iter().filter(|f| f >= &req).mn().save();
}

fn dir_size(dir: &HashMap<String, FS>) -> isize {
    let mut s = 0;
    for (_, f) in dir {
        match f {
            FS::Dir(d) => s += dir_size(d),
            FS::File(si) => s += *si,
        }
    }
    s
}

fn folder_sizes2(fs: &HashMap<String, FS>) -> Vec<isize> {
    let mut s = 0;
    let mut v = vec![];
    for (_, f) in fs {
        match f {
            FS::Dir(d) => {
                let a = folder_sizes2(d);
                s += a.iter().copied().s();
                v.extend(a);
            }
            FS::File(si) => {
                s += *si;
            }
        }
    }
    v.push(s);
    v
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
