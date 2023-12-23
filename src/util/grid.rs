use nalgebra::{
    allocator::{Allocator, Reallocator},
    DefaultAllocator, Dim, DimAdd, Dyn, Matrix, RawStorage, Scalar, Storage, StorageMut,
    ViewStorage, ViewStorageMut, U1,
};

pub const ADJ: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
pub const ADJD: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

pub trait MatrixTools<T> {
    type Adjacent<'a>
    where
        Self: 'a,
        T: 'a;
    type AdjacentMut<'a>
    where
        Self: 'a,
        T: 'a;
    type Padded;

    /// A submatrix of the matrix, centered at (x, y) with radius r.
    fn adj(&self, x: usize, y: usize, r: usize) -> Self::Adjacent<'_>;
    /// A mutable submatrix of the matrix, centered at (x, y) with radius r. Clipped to the bounds of the matrix.
    fn adj_mut(&mut self, x: usize, y: usize, r: usize) -> Self::AdjacentMut<'_>;
    /// Iterator over the matrix, yielding the index and the value at that index. Note that the order column-major.
    fn enumerate_iter<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)>
    where
        Self: 'a,
        T: 'a;
    /// Iterator over the matrix, yielding the index and the value at that index. Note that the order column-major.
    fn enumerate_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (usize, usize, &'a mut T)>
    where
        Self: 'a,
        T: 'a;
    /// Matrix padded with `pad` on all sides.
    fn pad(&self, pad: T) -> Self::Padded;
}

fn limit_index_adj(x: usize, max: usize, r: usize) -> usize {
    1 + if x < r { x } else { r } + if x + r >= max { max - x - 1 } else { r }
}

impl<T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C> + StorageMut<T, R, C>> MatrixTools<T>
    for Matrix<T, R, C, S>
where
    Self: Clone,
    R: DimAdd<U1>,
    C: DimAdd<U1>,
    <R as DimAdd<U1>>::Output: DimAdd<U1>,
    <C as DimAdd<U1>>::Output: DimAdd<U1>,
    DefaultAllocator: Reallocator<T, R, C, <R as DimAdd<U1>>::Output, C>,
    DefaultAllocator: Reallocator<
        T,
        <R as DimAdd<U1>>::Output,
        C,
        <R as DimAdd<U1>>::Output,
        <C as DimAdd<U1>>::Output,
    >,
    DefaultAllocator: Reallocator<
        T,
        <R as DimAdd<U1>>::Output,
        <C as DimAdd<U1>>::Output,
        <<R as DimAdd<U1>>::Output as DimAdd<U1>>::Output,
        <C as DimAdd<U1>>::Output,
    >,
    DefaultAllocator: Reallocator<
        T,
        <<R as DimAdd<U1>>::Output as DimAdd<U1>>::Output,
        <C as DimAdd<U1>>::Output,
        <<R as DimAdd<U1>>::Output as DimAdd<U1>>::Output,
        <<C as DimAdd<U1>>::Output as DimAdd<U1>>::Output,
    >,
{
    type Adjacent<'a> = Matrix<
        T,
        Dyn,
        Dyn,
        ViewStorage<
            'a,
            T,
            Dyn,
            Dyn,
            <S as RawStorage<T, R, C>>::RStride,
            <S as RawStorage<T, R, C>>::CStride,
        >,
    > where
        Self: 'a,
        T: 'a;
    type AdjacentMut<'a> = Matrix<
        T,
        Dyn,
        Dyn,
        ViewStorageMut<
            'a,
            T,
            Dyn,
            Dyn,
            <S as RawStorage<T, R, C>>::RStride,
            <S as RawStorage<T, R, C>>::CStride,
        >,
    > where
        Self: 'a,
        T: 'a;
    type Padded = Matrix<
        T,
        <<R as DimAdd<nalgebra::Const<1>>>::Output as DimAdd<nalgebra::Const<1>>>::Output,
        <<C as DimAdd<nalgebra::Const<1>>>::Output as DimAdd<nalgebra::Const<1>>>::Output,
        <DefaultAllocator as Allocator<
            T,
            <<R as DimAdd<nalgebra::Const<1>>>::Output as DimAdd<nalgebra::Const<1>>>::Output,
            <<C as DimAdd<nalgebra::Const<1>>>::Output as DimAdd<nalgebra::Const<1>>>::Output,
        >>::Buffer,
    >;

    fn adj(&self, x: usize, y: usize, r: usize) -> Self::Adjacent<'_> {
        self.view(
            (x.saturating_sub(r), y.saturating_sub(r)),
            (
                limit_index_adj(x, self.shape().0, r),
                limit_index_adj(y, self.shape().1, r),
            ),
        )
    }

    fn adj_mut(&mut self, x: usize, y: usize, r: usize) -> Self::AdjacentMut<'_> {
        let x1 = x.saturating_sub(r);
        let y1 = y.saturating_sub(r);
        let w = limit_index_adj(x, self.shape().0, r);
        let h = limit_index_adj(y, self.shape().1, r);
        self.index_mut(((x1..x1 + w), (y1..y1 + h)))
    }

    fn enumerate_iter<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)>
    where
        Self: 'a,

        T: 'a,
    {
        self.iter()
            .enumerate()
            .map(move |(i, x)| (i % self.shape().1, i / self.shape().1, x))
    }

    fn enumerate_iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (usize, usize, &'a mut T)>
    where
        Self: 'a,

        T: 'a,
    {
        let s = self.shape().1;
        self.iter_mut()
            .enumerate()
            .map(move |(i, x)| (i % s, i / s, x))
    }

    fn pad(&self, pad: T) -> Self::Padded {
        let new = self.clone().insert_row(0, pad.clone());
        let new = new.insert_column(0, pad.clone());
        let new = new.insert_row(self.shape().0 + 1, pad.clone());
        new.insert_column(self.shape().1 + 1, pad)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;
    use nalgebra::{Matrix2, Matrix4, Matrix6};

    use super::*;

    #[test]
    fn test_adj() {
        let cut = Matrix6::from_fn(|x, y| y * 6 + x);
        assert_eq!(
            cut.adj(0, 0, 1).iter().copied().collect_vec(),
            vec![0, 1, 6, 7]
        );
        assert_eq!(
            cut.adj(1, 1, 1).iter().copied().collect_vec(),
            vec![0, 1, 2, 6, 7, 8, 12, 13, 14]
        );
        assert_eq!(
            cut.adj(2, 2, 2).iter().copied().collect_vec(),
            vec![
                0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 18, 19, 20, 21, 22, 24, 25, 26,
                27, 28
            ]
        );
        assert_eq!(
            cut.adj(1, 2, 1).iter().copied().collect_vec(),
            vec![6, 7, 8, 12, 13, 14, 18, 19, 20]
        );
        assert_eq!(
            cut.adj(0, 2, 1).iter().copied().collect_vec(),
            vec![6, 7, 12, 13, 18, 19]
        );
        assert_eq!(
            cut.adj(5, 1, 1).iter().copied().collect_vec(),
            vec![4, 5, 10, 11, 16, 17]
        );
        assert_eq!(cut.adj(5, 5, 0).iter().copied().collect_vec(), vec![35]);
    }

    #[test]
    fn test_adj_mut() {
        let mut cut = Matrix6::from_fn(|x, y| y * 6 + x);
        cut.adj_mut(0, 0, 2).apply(|x| *x += 1);
        assert_eq!(
            cut.adj(0, 0, 3).iter().copied().collect_vec(),
            vec![1, 2, 3, 3, 7, 8, 9, 9, 13, 14, 15, 15, 18, 19, 20, 21]
        );
    }

    #[test]
    fn test_enumerate_iter() {
        assert_eq!(
            HashSet::from([(0, 0, &1), (1, 0, &3), (0, 1, &2), (1, 1, &4)]),
            Matrix2::new(1, 2, 3, 4).enumerate_iter().collect(),
        );
        assert_eq!(
            HashSet::from([
                (0, 0, &mut 1),
                (1, 0, &mut 3),
                (0, 1, &mut 2),
                (1, 1, &mut 4)
            ]),
            Matrix2::new(1, 2, 3, 4).enumerate_iter_mut().collect(),
        );
    }

    #[test]
    fn test_pad() {
        let cut = Matrix2::new(1, 2, 3, 4);
        assert_eq!(
            cut.pad(42),
            Matrix4::new(42, 42, 42, 42, 42, 1, 2, 42, 42, 3, 4, 42, 42, 42, 42, 42)
        );
    }
}
