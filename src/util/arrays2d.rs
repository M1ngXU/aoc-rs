use itertools::Itertools;

pub trait Transpose {
    type Transposed;

    fn t(self) -> Self::Transposed;
}
impl<T> Transpose for Vec<Vec<T>> {
    type Transposed = Self;

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

    use crate::prelude::Indeces2d;

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
}
