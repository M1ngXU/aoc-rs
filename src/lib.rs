#![feature(iter_repeat_n, iter_from_generator, iter_map_windows)]

pub mod prelude {
	pub use crate::itertools2::*;
	pub use crate::parser::*;
	pub use crate::util::*;
	pub use itertools::Itertools;

	pub use std::collections::*;
	pub use std::convert::{identity, AsMut, AsRef, Infallible};
	pub use std::fs::*;
	pub use std::iter::{
		empty, from_fn, from_generator, once, once_with, repeat, repeat_n, repeat_with, successors,
		zip,
	};
}
pub mod itertools2;
pub mod parser;
pub mod util;
