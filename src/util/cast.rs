macro_rules! def_cast {
	($name:ident, $type:ty) => {
		#[doc = "Cast to `"]
		#[doc = stringify!($type)]
		#[doc = "`"]
		fn $name(self) -> impl Iterator<Item = $type>;
	};
}

/// Casts primitive types to another primitive type
pub trait CastIter {
	def_cast!(ci8, i8);
	def_cast!(ci16, i16);
	def_cast!(ci32, i32);
	def_cast!(ci64, i64);
	def_cast!(ci128, i128);
	def_cast!(ciz, isize);

	def_cast!(cu8, u8);
	def_cast!(cu16, u16);
	def_cast!(cu32, u32);
	def_cast!(cu64, u64);
	def_cast!(cu128, u128);
	def_cast!(cuz, usize);

	def_cast!(cf32, f32);
	def_cast!(cf64, f64);
}

/// Casts primitive types to another primitive type
pub trait _CastIter {
	def_cast!(ci8, i8);
	def_cast!(ci16, i16);
	def_cast!(ci32, i32);
	def_cast!(ci64, i64);
	def_cast!(ci128, i128);
	def_cast!(ciz, isize);

	def_cast!(cu8, u8);
	def_cast!(cu16, u16);
	def_cast!(cu32, u32);
	def_cast!(cu64, u64);
	def_cast!(cu128, u128);
	def_cast!(cuz, usize);

	def_cast!(cf32, f32);
	def_cast!(cf64, f64);
}

macro_rules! impl_cast {
	($type:ty) => {
		impl<I: Iterator<Item = $type>> _CastIter for ($type, I) {
				            impl_cast!(@ ci8, i8);
				            impl_cast!(@ ci16, i16);
				            impl_cast!(@ ci32, i32);
				            impl_cast!(@ ci64, i64);
				            impl_cast!(@ ci128, i128);
				            impl_cast!(@ ciz, isize);

				            impl_cast!(@ cu8, u8);
				            impl_cast!(@ cu16, u16);
				            impl_cast!(@ cu32, u32);
				            impl_cast!(@ cu64, u64);
				            impl_cast!(@ cu128, u128);
				            impl_cast!(@ cuz, usize);

				            impl_cast!(@ cf32, f32);
				            impl_cast!(@ cf64, f64);
				        }
	};
	(@ $name:ident, $type:ty) => {
		fn $name(self) -> impl Iterator<Item = $type> {
		            self.1.map(|x| x as $type)
		        }
	};
}
impl_cast!(i8);
impl_cast!(i16);
impl_cast!(i32);
impl_cast!(i64);
impl_cast!(i128);
impl_cast!(isize);

impl_cast!(u8);
impl_cast!(u16);
impl_cast!(u32);
impl_cast!(u64);
impl_cast!(u128);
impl_cast!(usize);

impl_cast!(f32);
impl_cast!(f64);


macro_rules! impl_cast2 {
	($name:ident, $type:ty) => {
		fn $name(self) -> impl Iterator<Item = $type> {
			_CastIter::$name((Default::default(), self))
		}
	};
}

impl<I: Iterator<Item = T>, T: Default> CastIter for I
where
	(T, I): _CastIter,
{
	impl_cast2!(ci8, i8);

	impl_cast2!(ci16, i16);

	impl_cast2!(ci32, i32);

	impl_cast2!(ci64, i64);

	impl_cast2!(ci128, i128);

	impl_cast2!(ciz, isize);

	impl_cast2!(cu8, u8);

	impl_cast2!(cu16, u16);

	impl_cast2!(cu32, u32);

	impl_cast2!(cu64, u64);

	impl_cast2!(cu128, u128);

	impl_cast2!(cuz, usize);

	impl_cast2!(cf32, f32);

	impl_cast2!(cf64, f64);
}

// tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cu128() {
		let data = vec![1u8, 2, 3, 4, 5];
		let iter = data.into_iter();
		let result: Vec<u128> = iter.cu128().collect();
		assert_eq!(result, vec![1u128, 2, 3, 4, 5]);
	}

	#[test]
	fn test_cuz() {
		let data = vec![1u64, 2, 3, 4, 5];
		let iter = data.into_iter();
		let result: Vec<usize> = iter.cuz().collect();
		assert_eq!(result, vec![1usize, 2, 3, 4, 5]);
	}

	#[test]
	fn test_cf32() {
		let data = vec![1i32, 2, 3, 4, 5];
		let iter = data.into_iter();
		let result: Vec<f32> = iter.cf32().collect();
		assert_eq!(result, vec![1f32, 2.0, 3.0, 4.0, 5.0]);
	}

	#[test]
	fn test_cf64() {
		let data = vec![1f32, 2.0, 3.0, 4.0, 5.0];
		let iter = data.into_iter();
		let result: Vec<f64> = iter.cf64().collect();
		assert_eq!(result, vec![1f64, 2.0, 3.0, 4.0, 5.0]);
	}
}
