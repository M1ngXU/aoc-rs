#![feature(iter_map_windows)]

use aoc_rs::prelude::*;

fn one() {
	let s = pi!(ch);
	(s.into_iter()
		.map_windows(|a: &[_; 4]| a.into_iter().all_unique())
		.position(identity)
		.unwrap()
		+ 4)
	.save();
}

fn two() {
	let s = pi!(ch);
	(s.into_iter()
		.map_windows(|a: &[_; 14]| a.into_iter().all_unique())
		.position(identity)
		.unwrap()
		+ 14)
		.save();
}

fn main() {
	print!("Part 1: ");
	one();
	print!("Part 2: ");
	two();
}
