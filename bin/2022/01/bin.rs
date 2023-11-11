use aoc_rs::prelude::*;

fn one() {
	pi!(sb(NLNL, map(sb(NL, pn), |v| v.into_iter().s())))
		.into_iter()
		.max()
		.unwrap()
		.save();
}

fn two() {
	pi!(sb(NLNL, map(sb(NL, pn), |v| v.into_iter().s())))
		.into_iter()
		.sorted()
		.rev()
		.take(3)
		.s()
		.save();
}

fn main() {
	print!("Part 1: ");
	one();
	print!("Part 2: ");
	two();
}
