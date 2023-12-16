use std::iter::once;

use itertools::Itertools;
use ndarray::{s, Array, Array2, ArrayBase, ArrayView2, Ix2, LinalgScalar, ViewRepr};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    /// Matrix padded with `pad` on all sides.
    fn pad(&self, pad: T) -> Self::Padded;
    /// Fills cols/rows to the right/bottom to reach a dimension which is a multiple of 2, then returns the original size.
    fn filled_to_p2(&mut self, fill: T) -> Ix2;
}

fn limit_index_adj(x: usize, max: usize, r: usize) -> usize {
    1 + if x < r { x } else { r } + if x + r >= max { max - x - 1 } else { r }
}

pub trait DotPar {
    type Output;

    fn dot_par(self, other: Self) -> Self::Output;
    fn dot_par_strassen(self, other: Self) -> Self::Output;
}
impl<'a, T: LinalgScalar + Send + Sync> DotPar for ArrayView2<'a, T> {
    type Output = Array2<T>;

    fn dot_par(self, other: Self) -> Self::Output {
        assert_eq!(self.shape()[1], other.shape()[0], "Shape mismatch");
        let out_shape = [self.shape()[0], other.shape()[1]];
        Array2::from_shape_vec(
            out_shape,
            (0..out_shape[0])
                .cartesian_product(0..out_shape[1])
                .collect_vec()
                .into_par_iter()
                .map(|(y, x)| self.row(y).dot(&other.column(x)))
                .collect(),
        )
        .unwrap()
    }
    #[allow(non_snake_case)]
    fn dot_par_strassen(self, other: Self) -> Self::Output {
        let a_ = self;
        let b_ = other;
        if a_.shape()[0] % 2 == 1
            || a_.shape()[1] % 2 == 1
            || b_.shape()[0] % 2 == 1
            || b_.shape()[1] % 2 == 1
            || a_.len().max(other.len()) < 64 * 64
        {
            // about twice as fast as `dot_par`
            a_.dot(&b_)
        } else {
            let split_x1 = a_.shape()[1].div_ceil(2);
            let split_y1 = a_.shape()[0].div_ceil(2);
            let split_x2 = b_.shape()[1].div_ceil(2);
            let split_y2 = b_.shape()[0].div_ceil(2);

            let a = a_.slice(s![0..split_y1, 0..split_x1]);
            let b = a_.slice(s![0..split_y1, split_x1..]);
            let c = a_.slice(s![split_y1.., 0..split_x1]);
            let d = a_.slice(s![split_y1.., split_x1..]);

            let A = b_.slice(s![0..split_y2, 0..split_x2]);
            let B = b_.slice(s![0..split_y2, split_x2..]);
            let C = b_.slice(s![split_y2.., 0..split_x2]);
            let D = b_.slice(s![split_y2.., split_x2..]);

            let ca = &c - &a;
            let CD = &C - &D;
            let cd = &c + &d;
            let CA = &C - &A;
            let todo = [
                (a.view(), A.view()),
                (ca.view(), CD.view()),
                (cd.view(), CA.view()),
            ];
            let ((t, u), v) = rayon::join(
                || {
                    rayon::join(
                        || todo[0].0.dot_par_strassen(todo[0].1),
                        || todo[1].0.dot_par_strassen(todo[1].1),
                    )
                },
                || todo[2].0.dot_par_strassen(todo[2].1),
            );

            let w = &t
                + (&c + &d - &a)
                    .view()
                    .dot_par_strassen((&A + &D - &C).view());

            let abcd = &a + &b - &c - &d;
            let BCAD = &B + &C - &A - &D;
            let todo = [
                (b.view(), B.view()),
                (abcd.view(), D.view()),
                (d.view(), BCAD.view()),
            ];

            let mut out = Array2::zeros((a_.shape()[0], b_.shape()[1]));
            let out1 = unsafe { &mut *std::ptr::from_mut(&mut out) };
            let out2 = unsafe { &mut *std::ptr::from_mut(&mut out) };
            let out3 = unsafe { &mut *std::ptr::from_mut(&mut out) };
            let out4 = unsafe { &mut *std::ptr::from_mut(&mut out) };

            rayon::join(
                || {
                    rayon::join(
                        || {
                            out1.slice_mut(s![..split_y1, ..split_x2])
                                .assign(&(t + todo[0].0.dot_par_strassen(todo[0].1)))
                        },
                        || {
                            out2.slice_mut(s![..split_y1, split_x2..])
                                .assign(&(&w + &v + todo[1].0.dot_par_strassen(todo[1].1)))
                        },
                    )
                },
                || {
                    rayon::join(
                        || {
                            out3.slice_mut(s![split_y1.., ..split_x2])
                                .assign(&(&w + &u + todo[2].0.dot_par_strassen(todo[2].1)))
                        },
                        || {
                            out4.slice_mut(s![split_y1.., split_x2..])
                                .assign(&(&w + &u + &v))
                        },
                    )
                },
            );

            [out1, out2, out3, out4].map(std::mem::drop);

            out
        }
    }
}

pub trait FastPow<T> {
    fn pow(&mut self, e: usize);
    fn pow_par(&mut self, e: usize);
    fn pow_par_strassen(&mut self, e: usize);
}
impl<T: LinalgScalar> FastPow<T> for Array2<T>
where
    for<'a> ArrayView2<'a, T>: DotPar<Output = Self>,
{
    fn pow(&mut self, mut e: usize) {
        assert!(self.is_square());
        let mut y = Self::from_diag_elem(self.shape()[0], T::one());
        if e == 0 {
            *self = y;
            return;
        }
        while e > 1 {
            if e % 2 == 1 {
                y = self.dot(&y);
                e -= 1;
            }
            *self = self.dot(&*self);
            e /= 2;
        }
        *self = self.dot(&y);
    }
    fn pow_par(&mut self, mut e: usize) {
        assert!(self.is_square());
        let mut y = Self::from_diag_elem(self.shape()[0], T::one());
        if e == 0 {
            *self = y;
            return;
        }
        while e > 1 {
            if e % 2 == 1 {
                y = self.view().dot_par(y.view());
                e -= 1;
            }
            *self = self.view().dot_par(self.view());
            e /= 2;
        }
        *self = self.view().dot_par(y.view());
    }
    fn pow_par_strassen(&mut self, mut e: usize) {
        assert!(self.is_square());
        let mut y = Self::from_diag_elem(self.shape()[0], T::one());
        if e == 0 {
            *self = y;
            return;
        }
        while e > 1 {
            if e % 2 == 1 {
                y = self.view().dot_par_strassen(y.view());
                e -= 1;
            }
            *self = self.view().dot_par_strassen(self.view());
            e /= 2;
        }
        *self = self.view().dot_par_strassen(y.view());
    }
}

impl<T: Clone> MatrixTools<T> for Array2<T> {
    type Adjacent<'a> = ArrayBase<ViewRepr<&'a T>, Ix2>
    where
        Self: 'a,
        T: 'a;
    type AdjacentMut<'a> = ArrayBase<ViewRepr<&'a mut T>, Ix2>
    where
        Self: 'a,
        T: 'a;
    type Padded = Self;

    fn adj(&self, x: usize, y: usize, r: usize) -> Self::Adjacent<'_> {
        let nx = x.saturating_sub(r);
        let ny = y.saturating_sub(r);
        self.slice(s![
            ny..ny + limit_index_adj(y, self.shape()[0], r),
            nx..nx + limit_index_adj(x, self.shape()[1], r),
        ])
    }

    fn adj_mut(&mut self, x: usize, y: usize, r: usize) -> Self::AdjacentMut<'_> {
        let nx = x.saturating_sub(r);
        let ny = y.saturating_sub(r);
        self.slice_mut(s![
            ny..ny + limit_index_adj(y, self.shape()[1], r),
            nx..nx + limit_index_adj(x, self.shape()[0], r),
        ])
    }

    fn pad(&self, pad: T) -> Self::Padded {
        Array2::from_shape_vec(
            (self.shape()[0] + 2, self.shape()[1] + 2),
            vec![pad.clone(); self.shape()[0] + 2]
                .into_iter()
                .chain(
                    self.iter()
                        .collect_vec()
                        .chunks_exact(self.shape()[1])
                        .flat_map(|row| {
                            once(pad.clone())
                                .chain(row.iter().cloned().cloned())
                                .chain(once(pad.clone()))
                        })
                        .chain(vec![pad.clone(); self.shape()[0] + 2]),
                )
                .collect_vec(),
        )
        .unwrap()
    }
    fn filled_to_p2(&mut self, fill: T) -> Ix2 {
        let original = (self.shape()[0], self.shape()[1]);
        for _ in 0..original.0.next_power_of_two() - original.0 {
            self.push_row(Array::from_elem([self.shape()[1]], fill.clone()).view())
                .unwrap();
        }
        for _ in 0..original.1.next_power_of_two() - original.1 {
            self.push_column(Array::from_elem([self.shape()[0]], fill.clone()).view())
                .unwrap();
        }
        Ix2(original.0, original.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adj() {
        let cut = Array2::from_shape_fn((6, 5), |(y, x)| y * 6 + x);
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
            cut.adj(4, 1, 1).iter().copied().collect_vec(),
            vec![3, 4, 9, 10, 15, 16]
        );
        assert_eq!(cut.adj(4, 5, 0).iter().copied().collect_vec(), vec![34]);
    }

    #[test]
    fn test_adj_mut() {
        let mut cut = Array2::from_shape_fn((5, 6), |(y, x)| y * 6 + x);
        let mut c = cut.adj_mut(0, 0, 2);
        for y in 0..3 {
            for x in 0..3 {
                c[(x, y)] += 1;
            }
        }
        assert_eq!(
            cut.adj(0, 0, 3).iter().copied().collect_vec(),
            vec![1, 2, 3, 3, 7, 8, 9, 9, 13, 14, 15, 15, 18, 19, 20, 21]
        );
    }

    #[test]
    fn test_pad() {
        let cut = Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap();
        assert_eq!(
            cut.pad(42),
            Array2::from_shape_vec(
                (4, 4),
                vec![42, 42, 42, 42, 42, 1, 2, 42, 42, 3, 4, 42, 42, 42, 42, 42]
            )
            .unwrap()
        );
    }

    #[test]
    fn test_dot_par() {
        let cut = Array2::from_shape_fn((500, 500), |(x, y)| (x + y) as isize / 500);
        let out_par = cut.view().dot_par(cut.view());
        let out_strassen = cut.view().dot_par_strassen(cut.view());
        let expected = cut.dot(&cut);
        assert_eq!(out_par, expected);
        assert_eq!(out_strassen, expected);
    }

    #[test]
    fn test_pow() {
        let cut = Array2::from_shape_fn((1_00, 1_00), |(x, y)| (x + y) as isize / 1_000);
        let mut actual = cut.clone();
        actual.pow(25);
        let mut actual_par_strassen = cut.clone();
        actual_par_strassen.pow_par_strassen(25);
        let mut actual_par = cut.clone();
        actual_par.pow_par(25);
        let mut expected = cut.clone();
        for _ in 0..25 - 1 {
            expected = expected.dot(&cut);
        }
        assert_eq!(actual, expected);
        assert_eq!(actual_par_strassen, expected);
        assert_eq!(actual_par, expected);
    }

    #[test]
    fn test_fill() {
        let mut cut = Array::from_shape_fn((5, 7), |(y, x)| y * 7 + x);
        let o = cut.filled_to_p2(0);
        assert_eq!(o, Ix2(5, 7));
        assert_eq!(
            cut,
            Array::from_shape_fn((8, 8), |(y, x)| if y < 5 && x < 7 { y * 7 + x } else { 0 })
        );
    }
}
