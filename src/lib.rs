#![feature(iter_repeat_n, ptr_from_ref, iter_map_windows, iter_from_coroutine)]
#![cfg_attr(bootstrap, feature(generators))]
#![cfg_attr(not(bootstrap), feature(coroutines))]

pub mod prelude {
    pub use crate::itertools2::*;
    pub use crate::parser::*;
    pub use crate::util::*;
    pub use hashbrown::*;
    pub use itertools::Itertools;
    pub use linked_hash_map::LinkedHashMap;
    pub use nalgebra::sparse::*;
    #[allow(ambiguous_glob_reexports, clippy::useless_attribute)]
    pub use nalgebra::*;
    pub use petgraph::{algo::*, data::*, graph::*, *};
    pub use rustworkx_core::{
        centrality::*,
        coloring::*,
        connectivity::{
            all_simple_paths_multiple_targets, articulation_points, bfs_undirected,
            chain_decomposition, connected_components, core_number, cycle_basis, find_cycle,
            longest_simple_path_multiple_targets, number_connected_components,
            stoer_wagner_min_cut,
        },
        dictmap::*,
        distancemap::*,
        generators::*,
        shortest_path::{
            astar, bellman_ford as bellman_ford2, dijkstra as dijkstra2, k_shortest_path,
            negative_cycle_finder,
        },
        token_swapper::*,
        traversal::*,
        utils::*,
    };
    pub use z3::ast::{Ast, AstKind};
    pub use z3::*;
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

    pub use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, Bound, LinkedList, TryReserveError, VecDeque,
    };
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
