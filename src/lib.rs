#![feature(iter_repeat_n, iter_map_windows)]
#![cfg_attr(bootstrap, feature(generators))]
#![cfg_attr(not(bootstrap), feature(coroutines))]
#![feature(iter_from_coroutine)]

pub mod prelude {
    pub use crate::itertools2::*;
    pub use crate::parser::*;
    pub use crate::util::*;
    pub use itertools::Itertools;
    pub use regex::*;

    pub use std::collections::*;
    pub use std::convert::{identity, AsMut, AsRef, Infallible};
    pub use std::fs::*;
    pub use std::iter::{
        empty, from_coroutine, from_fn, once, once_with, repeat, repeat_n, repeat_with, successors,
        zip,
    };
}
pub mod itertools2;
pub mod parser;
pub mod util;
