use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt::Debug,
    iter::{Product, Sum},
    ops::Sub,
};

use itertools::Itertools;

pub trait BooleanIter {
    /// Counts the amount of `true` in an iter.
    fn trues(self) -> usize;
    /// Counts the amount of `false` in an iter.
    fn falses(self) -> usize;
}
impl<I: Iterator<Item = bool>> BooleanIter for I {
    fn trues(self) -> usize {
        self.filter(|x| *x).count()
    }
    fn falses(self) -> usize {
        self.filter(|x| !*x).count()
    }
}

pub trait Arithmetic<T> {
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

    fn f(mut self) -> T {
        self.next().unwrap()
    }
    fn l(self) -> T {
        self.last().unwrap()
    }
}

pub trait Itertools2<T> {
    /// Collect to a fixed sized array `[T; N]`, equivalent to `.collect_vec().try_into().unwrap()`
    fn cfsa<const N: usize>(self) -> [T; N];
    /// Chunk consecutive elements which have the same output of `f` into a `Vec`
    fn chunked_by(self, f: impl Fn(&T) -> bool) -> Vec<Vec<T>>;
    /// difference between two values
    fn d(self) -> impl Iterator<Item = T>
    where
        T: Copy + Sub<Output = T>;
}
impl<T: Debug, I: Iterator<Item = T>> Itertools2<T> for I {
    fn cfsa<const N: usize>(self) -> [T; N] {
        self.collect_vec().try_into().unwrap()
    }
    fn chunked_by(self, f: impl FnMut(&T) -> bool) -> Vec<Vec<T>> {
        self.group_by(f)
            .into_iter()
            .map(|(_, g)| g.collect_vec())
            .collect_vec()
    }

    fn d(self) -> impl Iterator<Item = T>
    where
        T: Copy + Sub<Output = T>,
    {
        self.tuple_windows().map(|(a, b)| b - a)
    }
}

pub trait TakeIsize {
    type Item;

    fn takei(self, i: isize) -> TakeIsizeIter<Self::Item>;
}
impl<T, I: Iterator<Item = T> + 'static> TakeIsize for I {
    type Item = T;

    fn takei(self, i: isize) -> TakeIsizeIter<Self::Item> {
        TakeIsizeIter::new(self, i)
    }
}
pub struct TakeIsizeIter<T> {
    iter: Box<dyn Iterator<Item = T>>,
    buf: VecDeque<T>,
    i: isize,
}
impl<T> TakeIsizeIter<T> {
    fn new(mut iter: impl Iterator<Item = T> + 'static, mut i: isize) -> Self {
        let mut buf = VecDeque::new();
        for _ in 0..-i {
            if let Some(t) = iter.next() {
                buf.push_back(t);
            } else {
                i = 0;
                buf.clear();
                break;
            }
        }
        Self {
            iter: Box::new(iter),
            buf,
            i,
        }
    }
}
impl<T> Iterator for TakeIsizeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i.cmp(&0) {
            Ordering::Greater => {
                self.i -= 1;
                self.iter.next()
            }
            Ordering::Less => {
                let n = self.buf.pop_front().unwrap();
                if let Some(t) = self.iter.next() {
                    self.buf.push_back(t);
                    Some(n)
                } else {
                    None
                }
            }
            Ordering::Equal => None,
        }
    }
}

/// All 4 adjacent cells of (x, y) in a 2D grid (without diagonals)
pub fn adj(x: usize, y: usize, max_x: usize, max_y: usize) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(x, y)| x == &0 || y == &0)
        .filter(|(x, y)| x != &0 && y != &0)
        .filter(move |(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;
            x >= 0 && x < max_x as isize && y >= 0 && y < max_y as isize
        })
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
}

/// All 8 adjacent cells of (x, y) in a 2D grid (with diagonal ones)
pub fn adjd(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(x, y)| x != &0 || y != &0)
        .filter(move |(dx, dy)| {
            let x = x as isize + dx;
            let y = y as isize + dy;
            x >= 0 && x < max_x as isize && y >= 0 && y < max_y as isize
        })
        .map(move |(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
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

    #[test]
    fn test_take_isize() {
        let iter = 0usize..10;
        assert_eq!(iter.takei(5).collect_vec(), vec![0, 1, 2, 3, 4]);
        let iter = 0usize..10;
        assert_eq!(
            iter.takei(-1).collect_vec(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
        let iter = 0usize..10;
        assert_eq!(iter.takei(-10).collect_vec(), Vec::<usize>::new());
    }
}
