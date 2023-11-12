use itertools::Itertools;

pub trait DisplayBlocksFlattened {
	fn db(self, w: usize, h: usize);
}
impl DisplayBlocksFlattened for &[bool] {
	fn db(self, w: usize, h: usize) {
		assert_eq!(w * h, self.len());
		for i in 0..h {
			for j in 0..w {
				print!(
					"{}",
					if self[i * w + j] {
						"\x1b[44m  \x1b[m"
					} else {
						"  "
					}
				);
			}
			println!();
		}
	}
}

pub trait DisplayBlocks {
	fn db(self);
}
impl DisplayBlocks for &[&[bool]] {
	fn db(self) {
		if self.is_empty() {
			return;
		}
		self.into_iter()
			.copied()
			.flatten()
			.copied()
			.collect_vec()
			.db(self.len(), self[0].len());
	}
}
