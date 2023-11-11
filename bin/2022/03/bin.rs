use aoc_rs::prelude::*;

fn one() {
	let p = sb(
		NL,
		to_p(|s| -> (_, _) {
			s.chars()
				.collect_vec()
				.chunks(s.len() / 2)
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
	let p = sb(NL, to_p(|s| s.chars().collect::<HashSet<_>>()));
	let s = pi!(p);
	s.into_iter()
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
