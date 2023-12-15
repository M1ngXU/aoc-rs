#![feature(iter_repeat_n, iter_map_windows, iter_from_coroutine)]
#![cfg_attr(bootstrap, feature(generators))]
#![cfg_attr(not(bootstrap), feature(coroutines))]

pub mod prelude {
    pub use crate::itertools2::*;
    pub use crate::parser::*;
    pub use crate::util::*;
    pub use itertools::Itertools;
    pub use linked_hash_map::LinkedHashMap;
    pub use nalgebra::coordinates::*;
    pub use nalgebra::dimension::*;
    pub use nalgebra::storage::*;
    // who uses `zero`/`one`/`abs`/`iter`/`clamp` submodule anyway
    #[allow(ambiguous_glob_reexports, clippy::useless_attribute)]
    pub use nalgebra::*;
    // who uses `zero`/`one`/`abs`/`iter`/`clamp` submodule anyway
    #[allow(ambiguous_glob_reexports, clippy::useless_attribute)]
    pub use num::*;
    pub use range_utils::*;
    pub use rayon::prelude::*;
    // who uses `bytes` submodule anyway
    #[allow(ambiguous_glob_reexports, clippy::useless_attribute)]
    pub use regex::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::de::*;
    pub use serde_json::ser::*;
    pub use serde_json::Map as JMap;
    // who uses `Map` anyway
    #[allow(ambiguous_glob_reexports, clippy::useless_attribute)]
    pub use serde_json::*;

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
