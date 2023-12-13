use itertools::Itertools;

use crate::itertools2::{Arithmetic, Itertools2};
use std::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::Sub,
};

pub trait Indeces1d {
    /// equivalent to `0..self.len()`
    fn i(&self) -> impl Iterator<Item = usize> + 'static;
}
impl<T> Indeces1d for Vec<T> {
    fn i(&self) -> impl Iterator<Item = usize> + 'static {
        0..self.len()
    }
}
impl<const N: usize, T> Indeces1d for [T; N] {
    fn i(&self) -> impl Iterator<Item = usize> + 'static {
        0..N
    }
}
impl<const N: usize, T> Indeces1d for &[T; N] {
    fn i(&self) -> impl Iterator<Item = usize> + 'static {
        0..N
    }
}
impl<T> Indeces1d for &[T] {
    fn i(&self) -> impl Iterator<Item = usize> + 'static {
        0..self.len()
    }
}

/// Dumb compiler thinks that vec might impl iterator in the future
pub trait ArithmeticV<T> {
    /// equivalent to `.sum()` (but no inference issues)
    fn s(self) -> T;
    /// equivalent to `.prod()` (but no inference issues)
    fn p(self) -> T;
    /// equivalent to `.max().unwrap()`
    fn mx(self) -> T;
    /// equivalent to `.min().unwrap()`
    fn mn(self) -> T;
    /// equivalent to `.first().unwrap()`
    fn f(self) -> T;
    /// equivalent to `.last().unwrap()`
    fn l(self) -> T;
    /// difference between two values
    fn d(self) -> Vec<T>
    where
        T: Copy + Sub<Output = T> + 'static;
}

impl<T: Sum + Product + Ord + Copy + Debug> ArithmeticV<T> for &Vec<T> {
    fn s(self) -> T {
        self.iter().copied().s()
    }

    fn p(self) -> T {
        self.iter().copied().p()
    }

    fn mx(self) -> T {
        self.iter().copied().mx()
    }

    fn mn(self) -> T {
        self.iter().copied().mn()
    }

    fn f(self) -> T {
        *self.first().unwrap()
    }

    fn l(self) -> T {
        *self.last().unwrap()
    }

    fn d(self) -> Vec<T>
    where
        T: Copy + Sub<Output = T> + 'static,
    {
        self.iter().copied().d().collect_vec()
    }
}
impl<const N: usize, T: Sum + Product + Ord + Copy + Debug> ArithmeticV<T> for &[T; N] {
    fn s(self) -> T {
        self.iter().copied().s()
    }

    fn p(self) -> T {
        self.iter().copied().p()
    }

    fn mx(self) -> T {
        self.iter().copied().mx()
    }

    fn mn(self) -> T {
        self.iter().copied().mn()
    }

    fn f(self) -> T {
        *self.first().unwrap()
    }

    fn l(self) -> T {
        *self.last().unwrap()
    }

    fn d(self) -> Vec<T>
    where
        T: Copy + Sub<Output = T> + 'static,
    {
        self.iter().copied().d().collect_vec()
    }
}

/// A fixed size array of `N` elements initialized with `0..N`
pub fn fsa<const N: usize>() -> [usize; N] {
    let mut r = [0; N];
    for (i, r) in r.iter_mut().enumerate() {
        *r = i;
    }
    r
}

/// Zip two fixed size arrays of `N` elements
pub trait ZipFixedSizeArray<const N: usize, T, O> {
    /// Zip two fixed size arrays of `N` elements
    fn zp(self, o: [O; N]) -> [(T, O); N];
}
impl<const N: usize, T, O> ZipFixedSizeArray<N, T, O> for [T; N] {
    fn zp(self, o: [O; N]) -> [(T, O); N] {
        let mut r: [(T, O); N] = unsafe { std::mem::zeroed() };
        for (i, t) in self.into_iter().zip(o).enumerate() {
            r[i] = t;
        }
        r
    }
}
