mod prelude {
    pub use nom::branch::*;
    pub use nom::bytes::complete::{
        take_till as tt, take_till1 as tt1, take_until as tu, take_until1 as tu1, take_while as tw,
        take_while1 as tw1, take_while_m_n as twmn, *,
    };
    pub use nom::character::complete::{line_ending as le, *};
    pub use nom::combinator::{map_opt as mo, map_parser as mp, *};
    pub use nom::multi::{count as rpt, many0 as mn0, many1 as mn1, many_m_n as mnmn, *};
    pub use nom::number::complete::{double, float};
    pub use nom::sequence::{
        delimited as dlt, pair, preceded as pcd, separated_pair as spair, terminated as tmd,
        tuple as tpl, *,
    };
    pub use nom::*;
}
use std::{fs::File, io::Write, path::Path};

use itertools::Itertools;
use nalgebra::{Dyn, OMatrix};
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
    let base = format!(
        "https://adventofcode.com/{}/day/{}",
        dir.parent().unwrap().file_name().unwrap().to_string_lossy(),
        dir.file_name()
            .unwrap()
            .to_string_lossy()
            .trim_start_matches('0'),
    );
    let client = reqwest::blocking::Client::builder()
        .user_agent("Auto-Download by M1ngXU: https://github.com/M1ngXU/aoc-rs/blob/0ddff0231a11dbd2ea68ecb4d7e1bca4ff8603f5/src/parser.rs#L34")
        .build()
        .unwrap();
    let res = client
        .get(format!("{}/input", base))
        .header("Cookie", include_str!("../cookie.txt"))
        .send()
        .unwrap();
    assert!(
        res.status().is_success(),
        "Error: {} ({base})",
        res.text().unwrap()
    );
    let binding = res.text().expect("Failed to read `input`.");
    let input = binding.trim();
    File::create(dir.join("input.txt"))
        .unwrap()
        .write_all(input.as_bytes())
        .expect("Failed to save `input.txt`.");

    let res = client.get(base).send().unwrap();
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

    println!();

    println!("First lines of input:");
    println!("====================");
    println!(
        "{}",
        input
            .lines()
            .take(5)
            .map(|l| if l.len() > 50 {
                format!("{} ...", &l[..50])
            } else {
                l.to_string()
            })
            .join("\n")
    );
    println!("====================");

    std::fs::write(dir.join("example.txt"), example).unwrap();
}

/// Leak the string ...
pub fn leak(s: &str) -> &'static str {
    s.to_string().leak()
}

#[macro_export]
/// parses `input.txt` using the parsers, then unwraps. leaks the input string
macro_rules! pi {
	($example:literal: $($t:tt)*) => {{
		// no need to make $($t)* mutable for the caller
		let mut p = $($t)*;
        let current = std::path::PathBuf::from(file!());
        let dir = current.parent().unwrap();
        download_input(dir);
        let s = leak(&if cfg!(any(feature = "dex", feature = "ex")) {
				std::fs::read_to_string(dir.join($example)).unwrap()
		} else {
				std::fs::read_to_string(dir.join("input.txt")).unwrap()
		});
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
    ($($t:tt)*) => {
        pi!("example.txt": $($t)*)
    }
}
pub use pi;

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

/// For each `find` in `finds`, replace it with the corresponding `replace`
///
/// leaks the returned string (unowned)
pub fn rpl<'a, const N: usize, R: Replacer + Copy>(
    finds: [Regex; N],
    replaces: [R; N],
) -> impl FnMut(I<'a>) -> IResult<I<'a>, I<'a>> {
    move |i: I<'a>| {
        let mut out = i.to_string();
        for (f, r) in finds.iter().zip(replaces.iter()) {
            out = f.replace_all(&out, *r).to_string();
        }
        Ok(("", leak(&out)))
    }
}

/// Parse all digits in the input, skipping non-digit characters
///
/// If you don't want to skip non-digit characters, just use:
/// `chp(pn)`
pub fn pds(i: I) -> IResult<I, Vec<isize>> {
    Ok((
        "",
        i.chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect(),
    ))
}

pub fn ms0(i: I) -> IResult<I, I> {
    multispace0(i)
}
pub fn ms1(i: I) -> IResult<I, I> {
    multispace1(i)
}

/// Parse all numbers in the input, skipping non-numeric characters
pub fn pns(mut i: I) -> IResult<I, Vec<isize>> {
    let mut out = vec![];
    while !i.is_empty() {
        match pn(i) {
            Ok((i2, n)) => {
                i = i2;
                out.push(n);
            }
            _ => {
                i = &i[1..];
            }
        }
    }
    Ok((i, out))
}

pub fn pjs<'a, T: Deserialize<'a>>(i: I<'a>) -> IResult<I<'a>, T> {
    from_str(i).map(|o| ("", o)).map_err(|e| {
        let _e = e;
        // println!("{_e}");
        Err::Error(nom::error::Error::new(i, ErrorKind::Not))
    })
}

/// Parse all floats in the input
pub fn pfs(mut i: I) -> IResult<I, Vec<f64>> {
    let mut out = vec![];
    while !i.is_empty() {
        match pf(i) {
            Ok((i2, n)) => {
                i = i2;
                out.push(n);
            }
            _ => {
                i = &i[1..];
            }
        }
    }
    Ok((i, out))
}

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
		t!(@)
	};
	(@) => {
		le
	};
	($t:tt) => {
		tag($t)
	};
	($t:tt $($r:tt)*) => {
		pair(t!($t), t!($($r)*))
	};
}
use regex::{Regex, Replacer};
use serde::Deserialize;
use serde_json::from_str;
pub use t;

pub fn id(x: I) -> IResult<I, I> {
    Ok(("", x))
}

pub fn lle(i: I) -> IResult<I, (I, I)> {
    pair(le, le)(i)
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
pub fn sb<'a, O, U>(
    p: impl FnMut(I<'a>) -> IResult<I<'a>, U>,
    f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    separated_list0(p, f)
}
/// Split by line ending: Vec<&str>, without trailing del
pub fn sble<'a, O>(
    f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    separated_list0(
        le,
        map_parser(
            alt((take_until("\r\n"), take_until("\n"), take_while(|_| true))),
            f,
        ),
    )
}
/// Split by double line endings: Vec<&str>, without trailing del
pub fn sblele<'a, O>(
    f: impl FnMut(I<'a>) -> IResult<I<'a>, O>,
) -> impl FnMut(I<'a>) -> IResult<I<'a>, Vec<O>> {
    separated_list0(
        pair(le, le),
        map_parser(
            alt((
                take_until("\r\n\r\n"),
                take_until("\n\n"),
                take_while(|_| true),
            )),
            f,
        ),
    )
}

/// Parse digit (`0..=9`): isize
pub fn pd(i: I) -> IResult<I, isize> {
    mp(take(1_usize), i128)(i).map(|(i, n)| (i, n as isize))
}

/// Parse number: isize
pub fn pn(i: I) -> IResult<I, isize> {
    i128(i).map(|(i, n)| (i, n as isize))
}

/// Parse float: f64
pub fn pf(i: I) -> IResult<I, f64> {
    double(i)
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

pub type GridMatrix<T> = OMatrix<T, Dyn, Dyn>;

/// Parses each character as a 2d grid (matrix)
pub fn grd(i: I) -> IResult<I, GridMatrix<char>> {
    let grid = sble(ch)(i)?.1;
    Ok((
        "",
        GridMatrix::from_vec(
            grid.len(),
            grid[0].len(),
            grid.into_iter().flatten().collect(),
        )
        .transpose(),
    ))
}
/// Parses each digit as a 2d grid (matrix)
pub fn grdd(i: I) -> IResult<I, GridMatrix<isize>> {
    let grid = sble(pds)(i)?.1;
    Ok((
        "",
        GridMatrix::from_vec(
            grid.len(),
            grid[0].len(),
            grid.into_iter().flatten().collect(),
        )
        .transpose(),
    ))
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

/// Take all
pub fn ta(i: I) -> IResult<I, I> {
    Ok(("", i))
}

#[macro_export]
macro_rules! parser {
    () => {
        parser!(LE)
    };
    (@ $($t:tt)*) => {
        $($t)*
    };
    (($($t:tt)*)) => {
        parser!($($t)*)
    };
    (~ $single_token:literal > $($rest:tt)*) => {
        parser!((@ tu($single_token)) > $($rest)*)
    };
    (~ $single_token:literal $($rest:tt)*) => {
        parser!(~ $single_token > $($rest)*)
    };
    ($single_token:literal) => {
        t!($single_token)
    };
    (LE) => {
        alt((t!("\n"), t!("\r\n")))
    };
    (LELE) => {
        alt((t!("\n\n"), t!("\r\n\r\n")))
    };
    (LELELE) => {
        alt((t!("\n\n\n"), t!("\r\n\r\n\r\n")))
    };
    ({$($first:tt)*} {$($second:tt)*} $($rest:tt)*) => {
        parser!({@ alt((parser!($($first)*), parser!($($second)*)))} $($rest)*)
    };
    ({$($first:tt)*} $($rest:tt)*) => {
        parser!(($($first)*) $($rest)*)
    };
    ($single_token:tt) => {
        $single_token
    };
    ((| $($content:tt)*)[LE]) => {
        sb(parser!(LE), mp(alt((tu("\r\n"), tu("\n"), ta)), parser!($($content)*)))
    };
    ((| $($content:tt)*)[LELE]) => {
        sb(parser!(LELE), mp(alt((tu("\r\n\r\n"), tu("\n\n"), ta)), parser!($($content)*)))
    };
    ((| $($content:tt)*)[LELELE]) => {
        sb(parser!(LELELE), mp(alt((tu("\r\n\r\n\r\n"), tu("\n\n\n"), ta)), parser!($($content)*)))
    };
    ((| $($content:tt)*)[$($delimiter:tt)?]) => {
        sb(parser!($($delimiter)?), mp(alt(($(tu($delimiter),)? ta,)), parser!($($content)*)))
    };
    ($content:tt [$($delimiter:tt)*]) => {
        sb(parser!($($delimiter)*), parser!($content))
    };
    ($trash:tt <<| $($useful:tt)*) => {
        parser!($trash << ($($useful)*))
    };
    ($trash:tt << $useful:tt $($rest:tt)*) => {
        parser!((@ pcd(parser!($trash), parser!($useful))) $($rest)*)
    };
    ($useful:tt >>| $($trash:tt)*) => {
        parser!($useful >> ($($trash)*))
    };
    ($useful:tt >> $trash:tt $($rest:tt)*) => {
        parser!((@ tmd(parser!($useful), parser!($trash))) $($rest)*)
    };
    ($first:tt >| $($second:tt)*) => {
        mp(parser!($first), parser!($($second)*))
    };
    ($first:tt > $second:tt $($rest:tt)*) => {
        parser!((@ mp(parser!($first), parser!($second))) $($rest)*)
    };
    ($lit:literal $($rest:tt)*) => {
        pair(t!($lit), parser!($($rest)*))
    };
    (($($content:tt)*)*) => {
        many0::<_, _, Error<I>, _>(parser!($($content)*))
    };
    (($($content:tt)*)+) => {
        many1::<_, _, Error<I>, _>(parser!($($content)*))
    };
    (($($t:tt)*) $($r:tt)*) => {
        pair(parser!($($t)*), parser!($($r)*))
    };
    ($cur:tt $($t:tt)*) => {
        pair(parser!($cur), parser!($($t)*))
    };
}
pub use parser;

#[cfg(windows)]
mod consts {
    pub const LE: &str = "\r\n";
    pub const LELE: &str = "\r\n\r\n";
    pub const LELELE: &str = "\r\n\r\n\r\n";
}
#[cfg(unix)]
mod consts {
    pub const LE: &str = "\n";
    pub const LELE: &str = "\n\n";
    pub const LELELE: &str = "\n\n\n";
}
pub use consts::*;

#[cfg(test)]
mod tests {
    // #![feature(trace_macros)]
    // trace_macros!(true);

    use nalgebra::Matrix2;

    use super::*;

    #[test]
    fn test_macro00() {
        let mut a = parser!((| id)[LE]);
        let b = a("abc\ndef");
        assert_eq!(b, Ok(("", vec!["abc", "def"])));
        let mut a = parser!((| id)[LELE]);
        let b = a("abc\r\n\r\ndef");
        assert_eq!(b, Ok(("", vec!["abc", "def"])));
        let mut a = parser!((| id)[LELELE]);
        let b = a("abc\n\n\ndef");
        assert_eq!(b, Ok(("", vec!["abc", "def"])));
    }

    #[test]
    fn test_macro01() {
        let mut a = parser!((pn)[le]);
        let b = a("1\n2");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!(pn[le]);
        let b = a("1\n2");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!((@ take(1_usize))*);
        let b = a("12");
        assert_eq!(b, Ok(("", vec!["1", "2"])));
    }

    #[test]
    fn test_macro02() {
        let mut a = parser!(("n:" pn)[le]);
        let b = a("n:1\nn:2");
        assert_eq!(b, Ok(("", vec![("n:", 1), ("n:", 2)])));
        let mut a = parser!((@ pair(t!("n:"), pn))[le]);
        let b = a("n:1\nn:2");
        assert_eq!(b, Ok(("", vec![("n:", 1), ("n:", 2)])));
        let mut a = parser!(~"\n" > pn);
        let b = a("3n:1\nn:2");
        assert_eq!(b, Ok(("\nn:2", 3)));
        let mut a = parser!(~"\n" pn);
        let b = a("3n:1\nn:2");
        assert_eq!(b, Ok(("\nn:2", 3)));
    }

    #[test]
    fn test_macro03() {
        let mut a = parser!((("n:") << pn (ms1) <<| pn)[le]);
        let b = a("n:1 2\nn:2 3");
        assert_eq!(b, Ok(("", vec![(1, 2), (2, 3)])));
        let mut a = parser!(("n:" << pn ms1 <<| pn)[le]);
        let b = a("n:1 2\nn:2 3");
        assert_eq!(b, Ok(("", vec![(1, 2), (2, 3)])));
        let mut a = parser!(((pn) >> "n" (pn) >>| "f")[le]);
        let b = a("1n2f\n2n3f");
        assert_eq!(b, Ok(("", vec![(1, 2), (2, 3)])));
        let mut a = parser!((pn >> "n" pn >>| "f")[le]);
        let b = a("1n2f\n2n3f");
        assert_eq!(b, Ok(("", vec![(1, 2), (2, 3)])));
        let mut a = parser!(((pn) >>| "n")[le]);
        let b = a("1n\n2n");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!((pn >>| "n")[le]);
        let b = a("1n\n2n");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!(((pn) >> ("n"))[le]);
        let b = a("1n\n2n");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!((pn >> ("n"))[le]);
        let b = a("1n\n2n");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!((pn >> "n")[le]);
        let b = a("1n\n2n");
        assert_eq!(b, Ok(("", vec![1, 2])));
    }

    #[test]
    fn test_macro04() {
        let mut a = parser!((("n:" ms0) <<| pn)[le]);
        let b = a("n:  1\nn:2");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!((("n:" ms1) <<| pf)[le]);
        let b = a("n:  1.0\nn:2");
        assert_eq!(b, Ok(("\nn:2", vec![1.0])));
        let mut a = parser!((("n:" ms1) << (pf))[le]);
        let b = a("n:  1.0\nn:2");
        assert_eq!(b, Ok(("\nn:2", vec![1.0])));
        let mut a = parser!((("n:" ms1) << pf)[le]);
        let b = a("n:  1.0\nn:2");
        assert_eq!(b, Ok(("\nn:2", vec![1.0])));
    }

    #[test]
    fn test_macro05() {
        let mut a = parser!(((("n:" ms0) << (@ take(1usize))) >| pn)[le]);
        let b = a("n:  1\nn:2");
        assert_eq!(b, Ok(("", vec![1, 2])));
        let mut a = parser!(((("n:" ms0) << (@ take(1usize))) > pn "t")[le]);
        let b = a("n:  1t\nn:2t");
        assert_eq!(b, Ok(("", vec![(1, "t"), (2, "t")])));
        let mut a = parser!(((("n:" ms0) << (@ take(1usize))) >| @ id)[le]);
        let b = a("n:  1\nn:2");
        assert_eq!(b, Ok(("", vec!["1", "2"])));
        let mut a = parser!(((("n:" ms0) << (@ take(1usize))) > (pn))[le]);
        let b = a("n:  1\nn:2");
        assert_eq!(b, Ok(("", vec![1, 2])));
    }

    #[test]
    fn test_macro06() {
        let mut a = parser!(ms1 pns);
        let b = a("  1 3 55");
        assert_eq!(b, Ok(("", ("  ", vec![1, 3, 55]))));
    }

    #[test]
    fn test_macro07() {
        // let mut a = parser!({"a" "c"}{"q" "z"});
        let mut a = parser!({({"a" "c"}{"b" "e"}{"c" "q"})}{"q" "z"});
        let b: Result<(&str, (&str, &str)), Err<nom::error::Error<_>>> = a("cq");
        assert_eq!(b, Ok(("", ("c", "q"))));
        let b = a("qz");
        assert_eq!(b, Ok(("", ("q", "z"))));
    }

    #[test]
    fn test_sb() {
        assert_eq!(_sb("\n", id)("1\n2\n3\n"), Ok(("", vec!["1", "2", "3"])));
        assert_eq!(_sb("\n", id)("1\n2\n3"), Ok(("", vec!["1", "2", "3"])));
    }

    #[test]
    fn test_parse_all_numbers() {
        assert_eq!(("", vec![1, -2, 3]), pns("1, -2, 3").unwrap());
        assert_eq!(("", vec![1, -22, 3]), pns("1, -22, - 3").unwrap());
        assert_eq!(("", vec![1, -2, 3]), pns("1-2+3").unwrap());
        assert_eq!(("", vec![1, -2, 3]), pns("1f-2da+b-f3").unwrap());
    }

    #[test]
    fn test_parse_all_digits() {
        assert_eq!(("", vec![1, 2, 3]), pds("1, -2, 3").unwrap());
        assert_eq!(("", vec![1, 2, 2, 3]), pds("1, -22, - 3").unwrap());
        assert_eq!(("", vec![1, 2, 3]), pds("1-2+3").unwrap());
        assert_eq!(("", vec![1, 2, 3]), pds("1f-2da+b-f3").unwrap());
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
    fn parse_digit() {
        assert_eq!(pd("123"), Ok(("23", 1)));
        assert_eq!(pd("0"), Ok(("", 0)));
        assert_eq!(pd("0.3"), Ok((".3", 0)));
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

    #[test]
    fn test_grid() {
        assert_eq!(Matrix2::new('1', '2', '3', '4'), grd("12\n34").p());
        assert_eq!(Matrix2::new(1, 2, 3, 4), grdd("12\n34").p());
    }
}
