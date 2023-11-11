mod prelude {
	pub use nom::branch::*;
	pub use nom::bytes::complete::*;
	pub use nom::character::complete::*;
	pub use nom::combinator::*;
	pub use nom::multi::*;
	pub use nom::sequence::*;
	pub use nom::*;
}
pub use prelude::*;

#[macro_export]
/// parses `input.txt` using the parsers, then unwraps
macro_rules! pi {
	($($t:tt)*) => {{
		// no need to make $($t)* mutable for the caller
		let mut p = $($t)*;
		cfg_if::cfg_if! {
			if #[cfg(feature = "dex")] {
				dbg!(p(include_str!("example.txt")).p())
			} else if #[cfg(feature = "ex")] {
				p(include_str!("example.txt")).p()
			} else {
				p(include_str!("input.txt")).p()
			}
		}
	}};
}
pub use pi;

#[cfg(windows)]
mod consts {
	pub const NL: &str = "\r\n";
	pub const NLNL: &str = "\r\n\r\n";
	pub const NLNLNL: &str = "\r\n\r\n\r\n";
}
#[cfg(not(windows))]
mod consts {
	pub const NL: &str = "\n";
	pub const NLNL: &str = "\n\n";
	pub const NLNLNL: &str = "\n\n\n";
}
pub use consts::*;

pub trait ParseAndUnwrap<O> {
	fn p(self) -> O;
}
impl<'a, O: std::fmt::Debug> ParseAndUnwrap<O> for IResult<I<'a>, O> {
	fn p(self) -> O {
		if self.as_ref().is_ok_and(|(i, _)| !i.is_empty()) {
			panic!("rest is not empty: {:?}", self);
		}
		self.unwrap().1
	}
}

type I<'a> = &'a str;

/// Consumes the whole string and maps the it with `f`
pub fn to_p<'a, O>(mut f: impl FnMut(I<'a>) -> O) -> impl FnMut(I<'a>) -> IResult<I<'a>, O> {
	move |i: I| Ok(("", f(i)))
}

pub fn id<'a>(x: I<'a>) -> IResult<I<'a>, I<'a>> {
	Ok(("", x))
}

/// Split by: Vec<&str>
pub fn sb<'a, O>(
	p: &'static str,
	f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
	terminated(
		separated_list0(
			tag(p),
			map_parser(alt((take_until(p), take_while1(|_| true))), f),
		),
		opt(tag(p)),
	)
}

/// Parse number: isize
pub fn pn<'a>(i: I<'a>) -> IResult<I<'a>, isize> {
	map_res(recognize(pair(opt(one_of("+-")), digit1)), |s: I| {
		s.parse::<isize>()
	})(i)
}

/// Parse float: f64
pub fn pf<'a>(i: I<'a>) -> IResult<I<'a>, f64> {
	map_res(
		recognize(tuple((
			opt(one_of("+-")),
			digit0,
			opt(tuple((tag("."), digit0))),
		))),
		|s: I| s.parse::<f64>(),
	)(i)
}

pub fn dlt<'a>(
	first: &'static str,
	second: &'static str,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, I<'a>> {
	delimited(tag(first), take_until(second), tag(second))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sb() {
		assert_eq!(sb("\n", id)("1\n2\n3\n"), Ok(("", vec!["1", "2", "3"])));
		assert_eq!(sb("\n", id)("1\n2\n3"), Ok(("", vec!["1", "2", "3"])));
	}

	#[test]
	fn parse_number() {
		assert_eq!(pn("123"), Ok(("", 123)));
		assert_eq!(pn("+123"), Ok(("", 123)));
		assert_eq!(pn("-123"), Ok(("", -123)));
		assert_eq!(pn("0"), Ok(("", 0)));
		assert_eq!(pn("+0"), Ok(("", 0)));
		assert_eq!(pn("-0"), Ok(("", 0)));
		assert_eq!(pn("000"), Ok(("", 0)));
	}

	#[test]
	fn parse_float() {
		assert_eq!(pf("123"), Ok(("", 123.0)));
		assert_eq!(pf("+123"), Ok(("", 123.0)));
		assert_eq!(pf("-123"), Ok(("", -123.0)));
		assert_eq!(pf("0"), Ok(("", 0.0)));
		assert_eq!(pf("+0"), Ok(("", 0.0)));
		assert_eq!(pf("-0"), Ok(("", 0.0)));
		assert_eq!(pf("000"), Ok(("", 0.0)));
		assert_eq!(pf("0.0"), Ok(("", 0.0)));
		assert_eq!(pf("+0.0"), Ok(("", 0.0)));
		assert_eq!(pf("-0.0"), Ok(("", 0.0)));
		assert_eq!(pf("000.000"), Ok(("", 0.0)));
		assert_eq!(pf(".1"), Ok(("", 0.1)));
	}

	#[test]
	fn test_delimited() {
		assert_eq!(dlt("(", ")")("(123)"), Ok(("", "123")));
		assert_eq!(dlt("(", ")")("(123)456"), Ok(("456", "123")));
		assert_eq!(dlt("(", ")")("(123)456(789)"), Ok(("456(789)", "123")));
		assert_eq!(
			dlt("(", ")")("(123)456(789)012"),
			Ok(("456(789)012", "123"))
		);
	}
}
