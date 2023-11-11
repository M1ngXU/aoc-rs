use std::iter::{Product, Sum};

pub trait Arithmetic<T> {
	/// equivalent to `.sum()` (but no inference issues)
	fn s(self) -> T;
	/// equivalent to `.prod()` (but no inference issues)
	fn p(self) -> T;
	/// equivalent to `.max().unwrap()`
	fn mx(self) -> T;
	/// equivalent to `.min().unwrap()`
	fn mn(self) -> T;
}
impl<T: Sum + Product + Ord, I: Iterator<Item = T>> Arithmetic<T> for I {
	fn s(self) -> T {
		self.sum()
	}

	fn p(self) -> T {
		self.product()
	}

	fn mx(self) -> T {
		self.max().unwrap()
	}

	fn mn(self) -> T {
		self.min().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sum() {
		let data = vec![1, 2, 3, 4, 5];
		let iter = data.into_iter();
		assert_eq!(iter.s(), 15); // 1 + 2 + 3 + 4 + 5 = 15

		let empty: Vec<i32> = Vec::new();
		let iter = empty.into_iter();
		assert_eq!(iter.s(), 0); // Sum of an empty iterator should be 0
	}

	#[test]
	fn test_product() {
		let data = vec![1, 2, 3, 4, 5];
		let iter = data.into_iter();
		assert_eq!(iter.p(), 120); // 1 * 2 * 3 * 4 * 5 = 120

		let empty: Vec<i32> = Vec::new();
		let iter = empty.into_iter();
		assert_eq!(iter.p(), 1); // Product of an empty iterator should be 1
	}
}
