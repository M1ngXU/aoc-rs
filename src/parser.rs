mod prelude {
    pub use nom::branch::*;
    pub use nom::bytes::complete::{
        take_till as tt, take_till1 as tt1, take_until as tu, take_until1 as tu1, take_while as tw,
        take_while1 as tw1, take_while_m_n as twmn, *,
    };
    pub use nom::character::complete::*;
    pub use nom::combinator::{map_opt as mo, map_parser as mp, *};
    pub use nom::multi::{count as rpt, many0 as mn0, many1 as mn1, many_m_n as mnmn, *};
    pub use nom::sequence::{
        delimited as dlt, pair, preceded as pcd, separated_pair as spair, terminated as tmd,
        tuple as tpl, *,
    };
    pub use nom::*;
}
use std::{fs::File, path::Path};

use itertools::Itertools;
use nom::error::{Error, ErrorKind};
pub use prelude::*;

#[macro_export]
/// `r!(some_string)` is equivalent to `Regex::new(some_string).unwrap()`
macro_rules! r {
    ($($t:tt)*) => {
        regex::Regex::new($($t)*).unwrap()
    };
}
pub use r;

pub fn download_input(dir: &Path) {
    if dir.join("input.txt").exists() {
        return;
    }
    let mut f = File::create(dir.join("input.txt")).unwrap();
    let base = format!(
        "https://adventofcode.com/{}/day/{}",
        dir.parent().unwrap().file_name().unwrap().to_string_lossy(),
        dir.file_name().unwrap().to_string_lossy(),
    );
    let mut res = reqwest::blocking::Client::new()
        .get(&format!("{}/input", base))
        .header("Cookie", include_str!("../cookie.txt"))
        .send()
        .unwrap();
    assert!(res.status().is_success());
    res.copy_to(&mut f).unwrap();

    let res = reqwest::blocking::get(base).unwrap();
    assert!(res.status().is_success());
    let binding = res.text().unwrap();
    let example = r!("<\\w*>|</\\w*>")
        .replace_all(
            binding
                .split_once("<pre><code>")
                .unwrap()
                .1
                .split_once("</code></pre>")
                .unwrap()
                .0
                .trim(),
            "",
        )
        .to_string();

    println!("Found example:");
    println!("====================");
    println!("{}", example);
    println!("====================");

    std::fs::write(dir.join("example.txt"), example).unwrap();
}
#[macro_export]
/// parses `input.txt` using the parsers, then unwraps. leaks the input string
macro_rules! pi {
	($($t:tt)*) => {{
		// no need to make $($t)* mutable for the caller
		let mut p = $($t)*;
        let current = std::path::PathBuf::from(file!());
        let dir = current.parent().unwrap();
        download_input(dir);
        let s = Box::leak(if cfg!(any(feature = "dex", feature = "ex")) {
				std::fs::read_to_string(dir.join("example.txt")).unwrap()
		} else {
				std::fs::read_to_string(dir.join("input.txt")).unwrap()
		}.into_boxed_str());
		cfg_if::cfg_if! {
			if #[cfg(feature = "dex")] {
				dbg!(p(s).p())
			} else if #[cfg(feature = "ex")] {
				p(s).p()
			} else {
				p(s).p()
			}
		}
	}};
}
pub use pi;

#[cfg(windows)]
mod consts {
    pub const LE: &str = "\r\n";
    pub const LLE: &str = "\r\n\r\n";
    pub const LLLE: &str = "\r\n\r\n\r\n";
}
#[cfg(not(windows))]
mod consts {
    pub const LE: &str = "\n";
    pub const LLE: &str = "\n\n";
    pub const LLLe: &str = "\n\n\n";
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

/// Parse until `p` and map the result with `f`
pub fn pu<'a, O>(
    p: &'static str,
    f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, O> {
    terminated(map_parser(take_until(p), f), tag(p))
}

#[macro_export]
macro_rules! t {
	() => {
		t!(@);
	};
	(@) => {
		t!(LE);
	};
	($t:tt) => {
		tag($t)
	};
	($t:tt $($r:tt)*) => {
		pair(t!($t), t!($($r)*))
	};
}
pub use t;

#[cfg(windows)]
#[macro_export]
macro_rules! c {
	(@) => {
		"\r\n"
	};
	($t:literal) => {
		$t
	};
	($t:tt $($r:tt)*) => {
		concat!(c!($t), $($r),*)
	};
}
#[cfg(not(windows))]
#[macro_export]
macro_rules! c {
	(@) => {
		"\n"
	};
	($t:literal) => {
		$t
	};
	($t:tt $($r:tt)*) => {
		concat!(c!($t), $($r),*)
	};
}
pub use c;

pub fn id(x: I) -> IResult<I, I> {
    Ok(("", x))
}

/// Split by: Vec<&str>, maybe use `sb` instead??
pub fn _sb<'a, O>(
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

/// Split by: Vec<&str>, with optional trailing del
pub fn sbd<'a, O>(
    p: &'static str,
    f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    terminated(separated_list0(tag(p), f), opt(tag(p)))
}
/// Split by: Vec<&str>, without trailing del
pub fn sb<'a, O>(
    p: &'static str,
    f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    separated_list0(tag(p), f)
}

/// Parse number: isize
pub fn pn(i: I) -> IResult<I, isize> {
    map_res(recognize(pair(opt(one_of("+-")), digit1)), |s: I| {
        s.parse::<isize>()
    })(i)
}

/// Parse float: f64
pub fn pf(i: I) -> IResult<I, f64> {
    map_res(
        recognize(tuple((
            opt(one_of("+-")),
            digit0,
            opt(tuple((tag("."), digit0))),
        ))),
        |s: I| s.parse::<f64>(),
    )(i)
}

pub fn dlt2<'a>(
    first: &'static str,
    second: &'static str,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, I<'a>> {
    delimited(tag(first), take_until(second), tag(second))
}

/// Splits the input into a char array
pub fn ch(i: I) -> IResult<I, Vec<char>> {
    Ok(("", i.chars().collect()))
}

fn chars_as_str(i: I) -> impl Iterator<Item = I> {
    let lengths = i.chars().map(|c| c.len()).collect_vec();
    let mut indices: Vec<(usize, usize)> = Vec::with_capacity(lengths.len());
    for i in 0..lengths.len() {
        if i == 0 {
            indices.push((0, lengths[0]));
        } else {
            indices.push((indices[i - 1].0 + lengths[i], lengths[i]));
        }
    }
    indices.into_iter().map(|(s, l)| &i[s..s + l])
}

/// Splits the input into a str array each of length 1 and applies a parser onto each char (as string)
pub fn chp<'a, O>(
    mut p: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    move |i: I<'a>| {
        let mut out = vec![];
        for s in chars_as_str(i) {
            let (s, o) = p(s)?;
            if !s.is_empty() {
                return Err(Err::Error(Error::new(s, ErrorKind::NonEmpty)));
            }
            out.push(o);
        }
        Ok(("", out))
    }
}
/// Splits the input into a str array each of length 1 and applies a function onto each char (as string)
pub fn chf<'a, O>(mut f: impl FnMut(I<'a>) -> O) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    move |i: I<'a>| {
        let mut out = vec![];
        for s in chars_as_str(i) {
            out.push(f(s));
        }
        Ok(("", out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sb() {
        assert_eq!(_sb("\n", id)("1\n2\n3\n"), Ok(("", vec!["1", "2", "3"])));
        assert_eq!(_sb("\n", id)("1\n2\n3"), Ok(("", vec!["1", "2", "3"])));
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
        assert_eq!(dlt2("(", ")")("(123)"), Ok(("", "123")));
        assert_eq!(dlt2("(", ")")("(123)456"), Ok(("456", "123")));
        assert_eq!(dlt2("(", ")")("(123)456(789)"), Ok(("456(789)", "123")));
        assert_eq!(
            dlt2("(", ")")("(123)456(789)012"),
            Ok(("456(789)012", "123"))
        );
    }
}
