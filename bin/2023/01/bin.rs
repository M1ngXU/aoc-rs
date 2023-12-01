use aoc_rs::prelude::*;

const DIGITS: [&str; 19] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
    "4", "5", "6", "7", "8", "9",
];

fn process(rf: &Regex, rl: &Regex, s: &str) -> usize {
    let [f, l] = [rf, rl].map(|r| r.captures(s).unwrap().get(1).unwrap().as_str());
    let [first, last] = [f, l].map(|s| (DIGITS.into_iter().position(|d| d == s).unwrap() + 1) % 10);
    first * 10 + last
}

fn get_regex(d: &[&str]) -> (Regex, Regex) {
    let first = r!(&format!(".*?({})", d.join("|")));
    let last = r!(&format!(".*({})", d.join("|")));
    (first, last)
}

fn one() {
    let (reg_first, reg_last) = get_regex(&DIGITS[9..]);
    let s = pi!("example1.txt": sble(map(id, |s| process(&reg_first, &reg_last, s))));
    s.into_iter().s().save()
}

fn two() {
    let (reg_first, reg_last) = get_regex(&DIGITS);
    let s = pi!("example2.txt": sble(map(id, |s| process(&reg_first, &reg_last, s))));
    s.into_iter().s().save()
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
