use itertools::Itertools;

pub trait MatrixTransform {
    type Transposed;
    type R90CW;
    type R90CC;
    type R180;

    fn mh(self) -> Self;
    fn mv(self) -> Self;
    fn t(self) -> Self::Transposed;
    fn r90cw(self) -> Self::R90CW;
    fn r90cc(self) -> Self::R90CC;
    fn r128(self) -> Self::R180;
}
impl<T> MatrixTransform for Vec<Vec<T>> {
    type Transposed = Self;
    type R90CW = Self;
    type R90CC = Self;
    type R180 = Self;

    fn t(self) -> Self::Transposed {
        if self.is_empty() {
            return self;
        }
        let mut r = Self::with_capacity(self[0].len());
        for _ in 0..self[0].len() {
            r.push(Vec::with_capacity(self.len()));
        }
        for i in 0..self.len() {
            assert_eq!(self[0].len(), self[i].len());
        }
        let m = self.len();
        let n = self[0].len();
        let mut v = self.into_iter().map(|r| r.into_iter()).collect_vec();
        for i in r.iter_mut().take(n) {
            for j in v.iter_mut().take(m) {
                i.push(j.next().unwrap());
            }
        }
        r
    }

    fn mh(self) -> Self {
        self.into_iter()
            .map(|r| r.into_iter().rev().collect_vec())
            .collect_vec()
    }

    fn mv(self) -> Self {
        self.into_iter().rev().collect_vec()
    }

    fn r90cw(self) -> Self::R90CW {
        self.t().mh()
    }

    fn r90cc(self) -> Self::R90CC {
        self.t().mv()
    }

    fn r128(self) -> Self::R180 {
        self.mh().mv()
    }
}

pub trait Indeces2d {
    /// equivalent to `(0..self[0].len()).cartesian_product(0..self.len())`
    fn ii(&self) -> impl Iterator<Item = (usize, usize)> + 'static;
}
impl<T> Indeces2d for Vec<Vec<T>> {
    fn ii(&self) -> impl Iterator<Item = (usize, usize)> + 'static {
        let x = self.get(0).map(|x| x.len());
        let len = self.len();
        x.into_iter()
            .flat_map(move |x| (0..x).cartesian_product(0..len))
    }
}
impl<const M: usize, const N: usize, T> Indeces2d for [[T; N]; M] {
    fn ii(&self) -> impl Iterator<Item = (usize, usize)> + 'static {
        (0..N).cartesian_product(0..M)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::prelude::{arrays2d::MatrixTransform, Indeces2d};

    #[test]
    fn test_2darray() {
        let cut: Vec<Vec<usize>> = vec![];
        assert_eq!(cut.ii().collect_vec(), vec![]);
        let cut: Vec<Vec<usize>> = vec![vec![]];
        assert_eq!(cut.ii().collect_vec(), vec![]);
        let cut: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            cut.ii().collect::<HashSet<_>>(),
            HashSet::from([(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)])
        );
    }

    #[test]
    fn test_transform() {
        let cut = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(cut.clone().t(), vec![vec![1, 4], vec![2, 5], vec![3, 6]]);
        assert_eq!(cut.clone().mh(), vec![vec![3, 2, 1], vec![6, 5, 4]]);
        assert_eq!(cut.clone().mv(), vec![vec![4, 5, 6], vec![1, 2, 3]]);
        assert_eq!(
            cut.clone().r90cw(),
            vec![vec![4, 1], vec![5, 2], vec![6, 3]]
        );
        assert_eq!(
            cut.clone().r90cc(),
            vec![vec![3, 6], vec![2, 5], vec![1, 4]]
        );
        assert_eq!(cut.clone().r128(), vec![vec![6, 5, 4], vec![3, 2, 1]]);
    }
}
