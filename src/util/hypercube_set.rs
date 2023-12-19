use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;
use range_utils::{BasicNum, RangeUtil};
use std::ops::RangeInclusive;

use crate::itertools2::Itertools2;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HypercubeSet<const DIM: usize, T: Ord + Clone + BasicNum> {
    pub ranges: Vec<[RangeInclusive<T>; DIM]>,
}
impl<const DIM: usize, T: Ord + Clone + BasicNum + Debug + Hash> HypercubeSet<DIM, T> {
    fn _new<R: RangeUtil<T>>(ranges: Vec<[R; DIM]>) -> Self {
        Self {
            ranges: ranges
                .into_iter()
                .map(|r| r.map(|r| r.from_incl()..=r.to_incl()))
                .collect(),
        }
    }

    pub fn size(&self) -> T
    where
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
    {
        self.ranges
            .iter()
            .map(|r| {
                let p: T = r
                    .iter()
                    .map(|r| r.to_incl().inc() - r.from_incl())
                    .reduce(|a, b| a * b)
                    .unwrap();
                p
            })
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn new<R: RangeUtil<T>>(ranges: Vec<[R; DIM]>) -> Self {
        let mut s = Self::_new(ranges);
        s.normalize();
        s
    }

    /// Normalize all ranges, i.e. remove duplicated
    fn normalize(&mut self) {
        *self = self.ranges.iter().cloned().fold(
            Self::_new::<RangeInclusive<T>>(vec![]),
            |mut s, r| {
                let other = Self::_new(vec![r]);
                let mut intersection = s.clone();
                intersection.intersect(&other);
                s._union(&other);
                // if !intersection.ranges.is_empty() {
                //     s._set_minus(&intersection);
                //     s._union(&intersection);
                //     println!("s: {s:?}");
                // }
                let mut changed;
                loop {
                    changed = false;
                    let mut rem = vec![];
                    for i in 0..s.ranges.len() {
                        for j in 0..s.ranges.len() {
                            if i != j
                                && s.ranges[i]
                                    .iter()
                                    .zip(&s.ranges[j])
                                    .all(|(a, b)| a.intersection(b) == Some(b.clone()))
                            {
                                rem.push(j);
                                changed = true;
                            }
                        }
                    }
                    for i in rem.into_iter().sorted().rev().dedup() {
                        s.ranges.remove(i);
                    }
                    let l = s.ranges.len();
                    'outer: for i in 0..l {
                        for j in 0..l {
                            if i != j
                                && s.ranges[i]
                                    .iter()
                                    .zip(&s.ranges[j])
                                    .all(|(a, b)| a.intersects(b))
                            {
                                let mut a = Self::_new(vec![s.ranges[i].clone()]);
                                let mut b = Self::_new(vec![s.ranges[j].clone()]);
                                let mut intersection = a.clone();
                                intersection.intersect(&b);
                                a._set_minus(&intersection);
                                b._set_minus(&intersection);
                                s.ranges.extend(a.ranges);
                                s.ranges.extend(b.ranges);
                                s.ranges.extend(intersection.ranges);
                                s.ranges.remove(i.max(j));
                                s.ranges.remove(i.min(j));
                                changed = true;
                                break 'outer;
                            }
                        }
                    }
                    if !changed {
                        break;
                    }
                }
                loop {
                    changed = false;

                    let mut merge = vec![];
                    'outer: for i in 0..s.ranges.len() {
                        for j in 0..s.ranges.len() {
                            if i != j {
                                if s.ranges[i]
                                    .iter()
                                    .zip(&s.ranges[j])
                                    .filter(|(a, b)| a == b)
                                    .count()
                                    == DIM - 1
                                {
                                    let (a, b) = s.ranges[i]
                                        .iter()
                                        .zip(&s.ranges[j])
                                        .find(|(a, b)| a != b)
                                        .unwrap();
                                    if a.to_incl() == b.from_incl().dec() {
                                        merge.push((
                                            i,
                                            j,
                                            s.ranges[i]
                                                .iter()
                                                .zip(&s.ranges[j])
                                                .position(|(a, b)| a != b)
                                                .unwrap(),
                                        ));
                                        changed = true;
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                    for (from, to, i) in merge.clone() {
                        s.ranges[from][i] =
                            s.ranges[from][i].from_incl()..=s.ranges[to][i].to_incl();
                    }
                    for (_, to, _) in merge.into_iter().rev() {
                        s.ranges.remove(to);
                    }
                    if !changed {
                        break;
                    }
                }
                s
            },
        );
    }

    pub fn set_minus(&mut self, other: &Self) {
        self._set_minus(other);
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
                    // let mut r = a.to_vec();
                    // let mut b = b.to_vec();
                    // r.remove(i);
                    // b.remove(i);
                    // new_ranges.extend(Self::_setminus_rec(&r, &b).into_iter().map(|mut r| {
                    //     r.insert(i, aa.clone());
                    //     println!("{r:?}|{aa:?}");
                    //     r
                    // }));
                }
                if let Some(aa) = bb {
                    let mut r = a.to_vec();
                    r[i] = aa;
                    new_ranges.push(r);
                    // let mut r = a.to_vec();
                    // let mut b = b.to_vec();
                    // r.remove(i);
                    // b.remove(i);
                    // new_ranges.extend(Self::_setminus_rec(&r, &b).into_iter().map(|mut r| {
                    //     r.insert(i, aa.clone());
                    //     println!("{r:?}|{aa:?}");
                    //     r
                    // }));
                }
            } else {
                if true || a.len() == 1 {
                    new_ranges.push(a.to_vec());
                } else {
                    let mut r = a.to_vec();
                    let mut b = b.to_vec();
                    r.remove(i);
                    b.remove(i);
                    new_ranges.extend(Self::_setminus_rec(&r, &b).into_iter().map(|mut r| {
                        r.insert(i, a[i].clone());
                        r
                    }));
                }
            }
        }
        new_ranges
    }

    fn _set_minus(&mut self, other: &Self) {
        let mut new_ranges = vec![];
        for r in &self.ranges {
            for o in &other.ranges {
                new_ranges.extend(Self::_setminus_rec(r, o));
            }
        }
        self.ranges = new_ranges
            .into_iter()
            .filter(|r| {
                other
                    .ranges
                    .iter()
                    .all(|rr| !rr.iter().zip(r).all(|(rr, r)| rr.intersects(r)))
            })
            .map(|r| r.into_iter().cfsa())
            .unique()
            .collect_vec();
    }

    pub fn intersect(&mut self, other: &Self) {
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
                let mut r = r.collect_vec();
                if r.iter().all(|r| r.is_some()) {
                    Some(r.iter_mut().map(|r| r.take().unwrap()).cfsa())
                } else {
                    None
                }
            })
            .collect_vec();
        // self.normalize();
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
    use std::ops::RangeFull;

    use super::*;

    #[test]
    fn test_1d_intersection() {
        let mut h = HypercubeSet::new(vec![[0..=10], [20..=25]]);
        h.intersect(&HypercubeSet::new(vec![[5..=40]]));
        assert_eq!(h.ranges, vec![[5..=10], [20..=25]]);
    }

    #[test]
    fn test_2d_intersection() {
        let mut h = HypercubeSet::new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h.intersect(&HypercubeSet::new(vec![[5..=15, 5..=15]]));
        assert_eq!(h.ranges, vec![[5..=10, 5..=15]]);
    }

    #[test]
    fn test_3d_intersection() {
        let mut h = HypercubeSet::new(vec![[0..=10, 0..=20, 0..=30]]);
        h.intersect(&HypercubeSet::new(vec![[5..=15, 5..=15, 5..=45]]));
        assert_eq!(h.ranges, vec![[5..=10, 5..=15, 5..=30]]);
    }

    #[test]
    fn test_4d_intersection() {
        let mut h = HypercubeSet::new(vec![
            [0..=10, 0..=20, 0..=30, 10..=20],
            [12..=25, 30..=40, 31..=32, -5..=5],
        ]);
        h.intersect(&HypercubeSet::new(vec![[0..=25, 10..=30, 25..=35, 4..=10]]));
        assert_eq!(
            h.ranges,
            vec![
                [0..=10, 10..=20, 25..=30, 10..=10],
                [12..=25, 30..=30, 31..=32, 4..=5]
            ]
        );
    }

    #[test]
    fn test_set_minus_1d() {
        let mut h = HypercubeSet::_new(vec![[0..=10], [20..=25]]);
        h._set_minus(&HypercubeSet::_new(vec![[5..=40]]));
        assert_eq!(h.ranges, vec![[0..=4]]);
    }

    #[test]
    fn test_set_minus_2d0() {
        let mut h = HypercubeSet::_new(vec![[0..=10, 0..=20]]);
        h._set_minus(&HypercubeSet::_new(vec![[0..=5, 0..=15]]));
        assert_eq!(h.ranges, vec![[6..=10, 0..=20], [0..=10, 16..=20]]);
        let mut h = HypercubeSet::_new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h._set_minus(&HypercubeSet::_new(vec![[0..=5, 0..=15], [0..=5, 30..=40]]));
        assert_eq!(
            h.ranges,
            vec![[6..=10, 0..=20], [0..=10, 16..=20], [-20..=-1, 30..=40]]
        );
    }

    #[test]
    fn test_set_minus_2d2() {
        let mut h = HypercubeSet::new(vec![[0..=10, 0..=20]]);
        println!("{h:?}");
        h.set_minus(&HypercubeSet::new(vec![[0..=5, 0..=15]]));
        assert_eq!(h.ranges, vec![[6..=10, 0..=20], [0..=5, 16..=20]]);
        let mut h = HypercubeSet::_new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h.set_minus(&HypercubeSet::new(vec![[0..=5, 0..=15], [0..=5, 30..=40]]));
        assert_eq!(
            h.ranges,
            vec![[6..=10, 0..=20], [0..=5, 16..=20], [-20..=-1, 30..=40]]
        );
    }

    #[test]
    fn test_set_minus_2d() {
        let mut h = HypercubeSet::_new(vec![[0..=10, 0..=20], [-20..=5, 30..=40]]);
        h._set_minus(&HypercubeSet::_new(vec![[5..=15, 5..=15]]));
        assert_eq!(
            h.ranges,
            vec![
                [0..=4, 0..=20],
                [0..=10, 0..=4],
                [0..=10, 16..=20],
                [-20..=5, 30..=40]
            ]
        );
    }

    #[test]
    fn test_1d_union() {
        let mut h = HypercubeSet::new::<RangeFull>(vec![]);
        h._union(&HypercubeSet::new(vec![[5..=40]]));
        assert_eq!(h.ranges, vec![[5..=40]]);
        let mut h = HypercubeSet::new(vec![[0..=10], [20..=25]]);
        h.union(&HypercubeSet::new(vec![[5..=40]]));
        assert_eq!(h.ranges, vec![[0..=40]]);
    }
}
