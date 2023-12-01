use aoc_rs::prelude::*;

const DIGITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn one() {
    let reg_first = r!(&format!(".*?({})", DIGITS[10..].join("|")));
    let reg_last = r!(&format!(".*({})", DIGITS[10..].join("|")));
    include_str!("example1.txt")
        .lines()
        .map(|s| {
            let first = DIGITS
                .into_iter()
                .position(|d| d == reg_first.captures(s).unwrap().get(1).unwrap().as_str())
                .unwrap()
                % 10;
            let last = DIGITS
                .into_iter()
                .position(|d| d == reg_last.captures(s).unwrap().get(1).unwrap().as_str())
                .unwrap()
                % 10;
            first * 10 + last
        })
        .s()
        .save();
}

fn two() {
    let reg_first = r!(&format!(".*?({})", DIGITS.join("|")));
    let reg_last = r!(&format!(".*({})", DIGITS.join("|")));
    include_str!("example2.txt")
        .lines()
        .map(|s| {
            let first = DIGITS
                .into_iter()
                .position(|d| d == reg_first.captures(s).unwrap().get(1).unwrap().as_str())
                .unwrap()
                % 10;
            let last = DIGITS
                .into_iter()
                .position(|d| d == reg_last.captures(s).unwrap().get(1).unwrap().as_str())
                .unwrap()
                % 10;
            first * 10 + last
        })
        .s()
        .save();
}

fn main() {
    print!("Part 1: ");
    one();
    print!("Part 2: ");
    two();
}
