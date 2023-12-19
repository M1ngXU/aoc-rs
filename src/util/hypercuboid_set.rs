use std::{
    fmt::Debug,
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Sub,
        SubAssign,
    },
};

use itertools::Itertools;
use num::{One, Zero};
use range_utils::{BasicNum, RangeUtil};
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Hash)]
/// Object invariant: normalized, i.e. no overlapping/adjacent hypercuboids
pub struct HypercuboidSet<const DIM: usize, T: Ord + Clone + BasicNum> {
    pub ranges: Vec<[RangeInclusive<T>; DIM]>,
}
impl<const DIM: usize, T: Ord + Clone + BasicNum> Default for HypercuboidSet<DIM, T> {
    fn default() -> Self {
        Self { ranges: vec![] }
    }
}
impl<const DIM: usize, T: Ord + Clone + BasicNum> PartialEq for HypercuboidSet<DIM, T> {
    fn eq(&self, other: &Self) -> bool {
        self.ranges
            .iter()
            .all(|r| other.ranges.iter().any(|rr| r == rr))
            && self.ranges.len() == other.ranges.len()
    }
}
impl<const DIM: usize, T: Ord + Clone + BasicNum> Eq for HypercuboidSet<DIM, T> {}
macro_rules! impl_arithmetic_ops {
    ($trait:ident, $trait_assign:ident, $fn:ident, $fn_assign:ident, $internal:ident) => {
        impl_arithmetic_ops!(@ mut $trait, $trait_assign, $fn, $fn_assign, $internal, HypercuboidSet<DIM, T>, HypercuboidSet<DIM, T>);
        impl_arithmetic_ops!(@ $trait, $fn, $internal, HypercuboidSet<DIM, T>, &HypercuboidSet<DIM, T>);
        impl_arithmetic_ops!(@ mut $trait, $trait_assign, $fn, $fn_assign, $internal, &HypercuboidSet<DIM, T>, HypercuboidSet<DIM, T>);
        impl_arithmetic_ops!(@ $trait, $fn, $internal, &HypercuboidSet<DIM, T>, &HypercuboidSet<DIM, T>);
    };
    (@ $trait:ident, $fn:ident, $internal:ident, $rhs:ty, $lhs:ty) => {
        impl<const DIM: usize, T: Ord + Clone + BasicNum> $trait<$rhs> for $lhs {
            type Output = HypercuboidSet<DIM, T>;

            fn $fn(self, rhs: $rhs) -> Self::Output {
                let mut x = self.clone();
                x.$internal(&rhs);
                x
            }
        }
    };
    (@ mut $trait:ident, $trait_assign:ident, $fn:ident, $fn_assign:ident, $internal:ident, $rhs:ty, $lhs:ty) => {
        impl<const DIM: usize, T: Ord + Clone + BasicNum> $trait<$rhs> for $lhs {
            type Output = HypercuboidSet<DIM, T>;

            fn $fn(mut self, rhs: $rhs) -> Self::Output {
                self.$internal(&rhs);
                self
            }
        }
        impl<const DIM: usize, T: Ord + Clone + BasicNum> $trait_assign<$rhs> for $lhs {
            fn $fn_assign(&mut self, rhs: $rhs) {
                self.$internal(&rhs);
            }
        }
    }
}
impl_arithmetic_ops!(Add, AddAssign, add, add_assign, union);
impl_arithmetic_ops!(BitOr, BitOrAssign, bitor, bitor_assign, union);
impl_arithmetic_ops!(Sub, SubAssign, sub, sub_assign, set_minus);
impl_arithmetic_ops!(BitAnd, BitAndAssign, bitand, bitand_assign, intersect);
impl_arithmetic_ops!(
    BitXor,
    BitXorAssign,
    bitxor,
    bitxor_assign,
    symmetric_difference
);

impl<const DIM: usize, T: Ord + Clone + BasicNum> HypercuboidSet<DIM, T> {
    fn _new<R: RangeUtil<T>>(ranges: Vec<[R; DIM]>) -> Self {
        Self {
            ranges: ranges
                .into_iter()
                .map(|r| r.map(|r| r.starts_at()..=r.ends_at()))
                .collect(),
        }
    }

    pub fn size(&self) -> T
    where
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: Zero + One,
    {
        self.ranges
            .iter()
            .map(|r| {
                let p: T = r
                    .iter()
                    .map(|r| r.ends_at().inc() - r.starts_at())
                    .reduce(|a, b| a * b)
                    .unwrap();
                p
            })
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn new<R: RangeUtil<T>>(ranges: Vec<[R; DIM]>) -> Self {
        let mut s = Self::_new(ranges);
        s.normalize();
        s
    }

    pub fn symmetric_difference(&mut self, other: &Self) {
        let mut one = other.clone();
        one.set_minus(&*self);
        self.set_minus(other);
        self.union(&one);
    }

    /// Normalize all ranges, i.e. remove duplicated
    fn normalize(&mut self) {
        *self = self.ranges.iter().cloned().fold(
            Self::_new::<RangeInclusive<T>>(vec![]),
            |mut s, r| {
                let mut other = Self::_new(vec![r]);
                // other._set_minus(&s);
                // println!("other af: {other:?}");
                // s._union(&other);
                // normalization(existing, new):
                // - make new disjoint of existing (new -= existing)
                // - if ranges are adjacent (all dimensions but one match, last dim is adjacent), merge
                other._set_minus(&s);
                while let Some(new) = other.ranges.pop() {
                    if s.ranges.iter().any(|r| {
                        r.iter()
                            .zip(&new)
                            .all(|(a, b)| a.intersection(b).is_some_and(|x| &x == b))
                    }) {
                        continue;
                    }
                    if let Some(a) = s.ranges.iter_mut().find(|r| {
                        r.iter()
                            .zip(&new)
                            .all(|(a, b)| a.intersection(b).is_some_and(|x| &x == a))
                    }) {
                        *a = new;
                        continue;
                    }
                    let mut changed = vec![];
                    for (i, current) in s.ranges.iter().enumerate().rev() {
                        let mut diff = new
                            .iter()
                            .zip(&*current)
                            .enumerate()
                            .filter(|(_, (a, b))| a != b);
                        if let Some((_, (a, b))) = diff.next() {
                            if diff.count() == 0 {
                                if a.ends_at().inc() == b.starts_at() {
                                    let mut current = current.clone();
                                    current[i] = a.starts_at()..=b.ends_at();
                                    other.ranges.push(current.clone());
                                    changed.push(i);
                                } else if b.ends_at().inc() == a.starts_at() {
                                    let mut current = current.clone();
                                    current[i] = b.starts_at()..=a.ends_at();
                                    other.ranges.push(current.clone());
                                    changed.push(i);
                                }
                            }
                        }
                    }
                    for c in changed {
                        s.ranges.swap_remove(c);
                    }
                    other._set_minus(&s);
                }
                // let mut changed;
                // loop {
                //     changed = false;
                //     let mut rem = vec![];
                //     for i in 0..s.ranges.len() {
                //         for j in 0..s.ranges.len() {
                //             if i != j
                //                 && s.ranges[i]
                //                     .iter()
                //                     .zip(&s.ranges[j])
                //                     .all(|(a, b)| a.intersection(b) == Some(b.clone()))
                //             {
                //                 rem.push(j);
                //                 changed = true;
                //             }
                //         }
                //     }
                //     for i in rem.into_iter().sorted().rev().dedup() {
                //         s.ranges.remove(i);
                //     }
                //     let l = s.ranges.len();
                //     'outer: for i in 0..l {
                //         for j in 0..l {
                //             if i != j
                //                 && s.ranges[i]
                //                     .iter()
                //                     .zip(&s.ranges[j])
                //                     .all(|(a, b)| a.intersects(b))
                //             {
                //                 let mut a = Self::_new(vec![s.ranges[i].clone()]);
                //                 let mut b = Self::_new(vec![s.ranges[j].clone()]);
                //                 let mut intersection = a.clone();
                //                 intersection.intersect(&b);
                //                 a._set_minus(&intersection);
                //                 b._set_minus(&intersection);
                //                 s.ranges.extend(a.ranges);
                //                 s.ranges.extend(b.ranges);
                //                 s.ranges.extend(intersection.ranges);
                //                 s.ranges.remove(i.max(j));
                //                 s.ranges.remove(i.min(j));
                //                 changed = true;
                //                 break 'outer;
                //             }
                //         }
                //     }
                //     if !changed {
                //         break;
                //     }
                // }
                // loop {
                //     changed = false;

                //     let mut merge = vec![];
                //     'outer: for i in 0..s.ranges.len() {
                //         for j in 0..s.ranges.len() {
                //             if i != j {
                //                 if s.ranges[i]
                //                     .iter()
                //                     .zip(&s.ranges[j])
                //                     .filter(|(a, b)| a == b)
                //                     .count()
                //                     == DIM - 1
                //                 {
                //                     let (a, b) = s.ranges[i]
                //                         .iter()
                //                         .zip(&s.ranges[j])
                //                         .find(|(a, b)| a != b)
                //                         .unwrap();
                //                     if a.ends_at() == b.starts_at().dec() {
                //                         merge.push((
                //                             i,
                //                             j,
                //                             s.ranges[i]
                //                                 .iter()
                //                                 .zip(&s.ranges[j])
                //                                 .position(|(a, b)| a != b)
                //                                 .unwrap(),
                //                         ));
                //                         changed = true;
                //                         break 'outer;
                //                     }
                //                 }
                //             }
                //         }
                //     }
                //     for (from, to, i) in merge.clone() {
                //         s.ranges[from][i] =
                //             s.ranges[from][i].starts_at()..=s.ranges[to][i].ends_at();
                //     }
                //     for (_, to, _) in merge.into_iter().rev() {
                //         s.ranges.remove(to);
                //     }
                //     if !changed {
                //         break;
                //     }
                // }
                s
            },
        );
    }

    pub fn set_minus(&mut self, other: &Self) {
        println!("START");
        self._set_minus(other);
        println!("EHERE");
        self.normalize();
    }

    fn _setminus_rec(
        a: &[RangeInclusive<T>],
        b: &[RangeInclusive<T>],
    ) -> Vec<Vec<RangeInclusive<T>>> {
        let mut new_ranges = vec![];
        for i in 0..a.len() {
            if a[i].intersects(&b[i]) {
                let (aa, bb) = a[i].setminus(&b[i]);
                if let Some(aa) = aa {
                    let mut r = a.to_vec();
                    r[i] = aa;
                    new_ranges.push(r);
                }
                if let Some(aa) = bb {
                    let mut r = a.to_vec();
                    r[i] = aa;
                    new_ranges.push(r);
                }
            } else {
                new_ranges.push(a.to_vec());
            }
        }
        new_ranges
    }

    // most likely wrong
    fn _set_minus(&mut self, other: &Self) {
        let mut other = other.clone();
        other._intersect(&self);
        if other.ranges.is_empty() {
            return;
        }
        let mut blocks = vec![];
        for r in &self.ranges {
            let mut block = vec![];
            for o in &other.ranges {
                let mut o2 = Self::_new(vec![r.clone()]);
                o2._intersect(&Self::_new(vec![o.clone()]));
                // at most one range
                assert!(o2.ranges.len() <= 1);
                if let Some(o) = o2.ranges.get(0) {
                    let mut small_hypercuboids = vec![vec![]];
                    for i in 0..DIM {
                        let opts = [
                            r[i].starts_at()..=o[i].starts_at().dec(),
                            o[i].clone(),
                            o[i].ends_at().inc()..=r[i].ends_at(),
                        ];
                        small_hypercuboids = small_hypercuboids
                            .into_iter()
                            .cartesian_product(opts)
                            .map(|(old, new)| {
                                old.into_iter().chain(std::iter::once(new)).collect_vec()
                            })
                            .collect_vec();
                        //         println!("{:?}|{:?}", r[i], o[i]);
                        //         if r[i].intersects(&o[i]) {
                        //             let (aa, bb) = r[i].setminus(&o[i]);
                        //             println!("{aa:?}|{bb:?}");
                        //             let mut nr = Self::default();
                        //             if let Some(aa) = aa {
                        //                 let mut r = r.clone();
                        //                 r[i] = aa;
                        //                 nr._union(&Self::_new(vec![r]));
                        //             }
                        //             if let Some(aa) = bb {
                        //                 let mut r = r.clone();
                        //                 r[i] = aa;
                        //                 nr._union(&Self::_new(vec![r]));
                        //             }
                        //             println!("nr: {nr:?}");
                        //             if i > 0 {
                        //                 nrr._intersect(&nr);
                        //             } else {
                        //                 nrr._union(&nr);
                        //             }
                        //         } else {
                        //             println!("r: {r:?}");
                        //             if i > 0 {
                        //                 nrr._intersect(&Self::_new(vec![r.clone()]));
                        //             } else {
                        //                 nrr._union(&Self::_new(vec![r.clone()]));
                        //             }
                        //         }
                    }
                    let nrr = Self::_new(
                        small_hypercuboids
                            .into_iter()
                            .filter(|x| x.iter().all(|x| !x.is_empty()))
                            .map(|r| {
                                let len = r.len();
                                let x: [RangeInclusive<T>; DIM] =
                                    r.try_into().unwrap_or_else(|_| {
                                        panic!(
                                            "Length not matching?? (dim=`{DIM}` vs recv=`{len}`)"
                                        )
                                    });
                                x
                            })
                            .filter(|x| x != o)
                            .collect_vec(),
                    );
                    block.push(nrr);
                } else {
                    block.push(Self::_new(vec![r.clone()]));
                }
            }
            if block.len() <= 1 {
                blocks.extend(block);
            } else {
                blocks.push(
                    block
                        .into_iter()
                        .reduce(|mut a, b| {
                            a._intersect(&b);
                            a
                        })
                        .unwrap(),
                );
            };
        }
        *self = if blocks.len() == 1 {
            blocks.into_iter().next().unwrap()
        } else if blocks.len() == 0 {
            Self::default()
        } else {
            blocks.into_iter().fold(Self::default(), |mut a, b| {
                let mut o = other.clone();
                o._intersect(&b);
                if o.ranges.is_empty() {
                    a._union(&b);
                }
                a
            })
        };
    }

    pub fn intersect(&mut self, other: &Self) {
        self._intersect(other);
        // normalizing should be useless
        let a = self.clone();
        self.normalize();
        assert!(a == *self, "Intersection required normalization??");
    }

    fn _intersect(&mut self, other: &Self) {
        self.ranges = self
            .ranges
            .iter()
            .flat_map(|r1| {
                other
                    .ranges
                    .iter()
                    .map(move |r2| r1.iter().zip(r2.iter()).map(|(r1, r2)| r1.intersection(r2)))
            })
            .filter_map(|r| {
                let r = r.collect_vec();
                if r.iter().all(|r| r.is_some()) {
                    let r = r.into_iter().map(|r| r.unwrap()).collect_vec();

                    let len = r.len();
                    let x: [RangeInclusive<T>; DIM] = r.try_into().unwrap_or_else(|_| {
                        panic!("Length not matching?? (dim=`{DIM}` vs recv=`{len}`)")
                    });
                    Some(x)
                } else {
                    None
                }
            })
            .collect_vec();
    }

    pub fn union(&mut self, other: &Self) {
        self._union(other);
        self.normalize();
    }

    fn _union(&mut self, other: &Self) {
        self.ranges.extend(other.ranges.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(
            HypercuboidSet::new(vec![[0..=10, 10..=20, 10..=14], [0..=10, 10..=20, 10..=14]]),
            HypercuboidSet::new(vec![[0..=10, 10..=20, 10..=14]])
        );
        assert_eq!(
            HypercuboidSet::new(vec![
                [0..=10, 10..=20, 10..=14],
                [10..=15, 10..=20, 10..=14]
            ]),
            HypercuboidSet::new(vec![[0..=15, 10..=20, 10..=14]])
        );
        assert_eq!(
            HypercuboidSet::new(vec![
                [0..=10, 10..=20, 10..=14],
                [11..=15, 10..=20, 10..=14]
            ]),
            HypercuboidSet::new(vec![[0..=15, 10..=20, 10..=14]])
        );
    }

    #[test]
    fn test_normalize_complex() {
        assert_eq!(
            HypercuboidSet::new(vec![[0..=10], [16..=20], [11..=15],]),
            HypercuboidSet::new(vec![[0..=20]])
        );
    }

    #[test]
    fn test_3d_intersection() {
        let mut h = HypercuboidSet::new(vec![[0..=10, 0..=20, 0..=30]]);
        h.intersect(&HypercuboidSet::new(vec![[5..=15, 5..=15, 5..=45]]));
        assert_eq!(h, HypercuboidSet::new(vec![[5..=10, 5..=15, 5..=30]]));
    }

    #[test]
    fn test_4d_intersection() {
        let mut h = HypercuboidSet::new(vec![
            [0..=10, 0..=20, 0..=30, 10..=20],
            [12..=25, 30..=40, 31..=32, -5..=5],
        ]);
        h.intersect(&HypercuboidSet::new(vec![[
            0..=25,
            10..=30,
            25..=35,
            4..=10,
        ]]));
        assert_eq!(
            h,
            HypercuboidSet::new(vec![
                [0..=10, 10..=20, 25..=30, 10..=10],
                [12..=25, 30..=30, 31..=32, 4..=5]
            ])
        );
    }

    #[test]
    fn test_set_minus_1d() {
        let mut h = HypercuboidSet::new(vec![[0..=10], [20..=25]]);
        h.set_minus(&HypercuboidSet::new(vec![[5..=40]]));
        assert_eq!(h, HypercuboidSet::new(vec![[0..=4]]));
    }

    #[test]
    fn test_set_minus_2d0() {
        let mut h = HypercuboidSet::new(vec![[0..=10, 0..=20]]);
        h.set_minus(&HypercuboidSet::new(vec![[0..=5, 0..=15]]));
        assert_eq!(
            h,
            HypercuboidSet::new(vec![[6..=10, 0..=15], [0..=10, 16..=20]])
        );
        let mut h = HypercuboidSet::new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h.set_minus(&HypercuboidSet::new(vec![
            [0..=5, 0..=15],
            [0..=5, 30..=40],
        ]));
        assert_eq!(
            h,
            HypercuboidSet::new(vec![
                [6..=10, 0..=15],
                [0..=10, 16..=20],
                [-20..=-1, 30..=40]
            ])
        );
    }

    #[test]
    fn test_set_minus_1d2() {
        let mut cut = HypercuboidSet::new(vec![[5..=40]]);
        cut.set_minus(&HypercuboidSet::new(vec![[0..=10], [20..=25]]));
        assert_eq!(cut, HypercuboidSet::new(vec![[11..=19], [26..=40]]));
    }

    #[test]
    fn test_set_minus_2d2() {
        // let mut h = HypercuboidSet::new(vec![[0..=10, 0..=20]]);
        // h.set_minus(&HypercuboidSet::new(vec![[0..=5, 0..=15]]));
        // assert_eq!(h.ranges, vec![[6..=10, 0..=20], [0..=5, 16..=20]]);
        let mut h = HypercuboidSet::new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h.set_minus(&HypercuboidSet::new(vec![
            [0..=5, 0..=15],
            [0..=5, 30..=40],
        ]));
        assert_eq!(
            h,
            HypercuboidSet::new(vec![
                [6..=10, 0..=15],
                [0..=10, 16..=20],
                [-20..=-1, 30..=40]
            ])
        );
    }

    #[test]
    fn test_set_minus_2d() {
        let mut h = HypercuboidSet::new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h.set_minus(&HypercuboidSet::new(vec![[5..=15, 5..=15]]));
        assert_eq!(
            h,
            HypercuboidSet::new(vec![
                [0..=4, 0..=20],
                [5..=10, 0..=4],
                [5..=10, 16..=20],
                [-20..=5, 30..=40]
            ])
        );
    }

    #[test]
    fn test_1d_union() {
        // let mut h = HypercuboidSet::new::<RangeFull>(vec![]);
        // h.union(&HypercuboidSet::new(vec![[5..=40]]));
        // assert_eq!(h, HypercuboidSet::new(vec![[5..=40]]));
        let mut h = HypercuboidSet::new(vec![[0..=10], [20..=25]]);
        h.union(&HypercuboidSet::new(vec![[5..=40]]));
        assert_eq!(h, HypercuboidSet::new(vec![[0..=40]]));
    }
}
