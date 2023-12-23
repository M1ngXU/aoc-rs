use itertools::Itertools;
use nalgebra::{Dyn, OMatrix, OVector};
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub struct EigenDecomposition {
    pub eigenvalues: Vec<f64>,
    pub eigenvectors: Vec<OVector<f64, Dyn>>,
}
impl EigenDecomposition {
    pub fn from_symmetric_real(a: &OMatrix<f64, Dyn, Dyn>) -> Self {
        assert!(a.is_square());
        let mut rng = thread_rng();
        let distr = Normal::new(0.0, 1.0).unwrap();
        let mut basis = Vec::<OVector<f64, Dyn>>::new();
        let mut evalues = Vec::<f64>::new();
        let mut evector = OVector::<f64, Dyn>::new_random(a.shape().0);
        let mut last: f64 = 9999.99;
        let mut evalue: f64 = distr.sample(&mut rng);
        while basis.len() < a.shape().0 {
            // Power iteration
            //
            // let ev = a * &evector;
            // let ev = &ev
            //     - basis
            //         .iter()
            //         .map(|b| b * (b.transpose() * &ev))
            //         .fold(OVector::<f64, Dyn>::zeros(a.shape().0), |a, b| a + b);
            // let ev = ev.normalize();
            // let norm = (&ev - &evector).norm();
            // evector = ev;
            // if norm < 1e-8 {
            //     evalues.push((evector.transpose() * a * &evector).into_scalar());
            //     basis.push(evector);
            //     evector = OVector::<f64, Dyn>::new_random(a.shape().0);
            //     // println!("direct");
            // } else if (last.abs() - norm.abs()).abs() < 1e-8 {
            //     // println!("last: {}, norm: {}", last, norm);
            //     evalues.push((evector.transpose() * a * &evector).into_scalar());
            //     basis.push(evector);
            //     evector = OVector::<f64, Dyn>::new_random(a.shape().0);
            // }
            // last = norm;

            // Rayleigh quotient iteration
            //
            let mut clone = a.clone();
            clone.set_diagonal(&clone.diagonal().map(|x| x - evalue));
            let Some(ev) = clone.try_inverse().map(|m| (m * &evector).normalize()) else {
                evalues.push(evalue);
                basis.push(evector);
                evector = OVector::<f64, Dyn>::new_random(a.shape().0);
                evalue = distr.sample(&mut rng);
                continue;
            };
            let ev = &ev
                - basis
                    .iter()
                    .map(|b| b * (b.transpose() * &ev))
                    .fold(OVector::<f64, Dyn>::zeros(a.shape().0), |a, b| a + b);
            evector = ev;
            let lv = a * &evector;
            evalue = (evector.transpose() * &lv).into_scalar();
            let norm = (lv - evalue * &evector).norm();
            if norm < 1e-8 {
                evalues.push(evalue);
                basis.push(evector);
                evector = OVector::<f64, Dyn>::new_random(a.shape().0);
                evalue = distr.sample(&mut rng);
            } else if (last.abs() - norm.abs()).abs() < 1e-8 {
                evalues.push(evalue);
                basis.push(evector);
                evector = OVector::<f64, Dyn>::new_random(a.shape().0);
                evalue = distr.sample(&mut rng);
            }
            last = norm;
        }
        Self {
            eigenvalues: evalues,
            eigenvectors: basis,
        }

        // Power iteration with all vectors at once, ortho-normalize at each step
        //
        // let mut eigenbasis = OMatrix::<f64, Dyn, Dyn>::new_random(a.shape().0, a.shape().0);
        // eigenbasis = eigenbasis.qr().q();
        // loop {
        //     let new_basis = a * &eigenbasis;
        //     let mut x = new_basis.ad_mul(&new_basis);
        //     x.fill_diagonal(0.0);
        //     if x.abs().sum() < 1e-8 {
        //         eigenbasis = new_basis.clone();
        //         eigenbasis = OMatrix::from_columns(
        //             &eigenbasis
        //                 .column_iter_mut()
        //                 .map(|c| c.normalize())
        //                 .collect_vec(),
        //         );
        //         let lambda = eigenbasis.transpose() * a * &eigenbasis;
        //         return Self {
        //             eigenvalues,
        //             eigenvectors: lambda
        //             .diagonal()
        //             .into_iter()
        //             .copied()
        //             .collect_vec()
        //                 .column_iter()
        //                 .map(|c| c.clone_owned())
        //                 .collect_vec(),
        //         };
        //     }
        //     eigenbasis = new_basis.qr().q();
        // }
    }
}

pub struct CompactSVD {
    pub u: OMatrix<f64, Dyn, Dyn>,
    pub s: OMatrix<f64, Dyn, Dyn>,
    pub v: OMatrix<f64, Dyn, Dyn>,
}
impl CompactSVD {
    pub fn new(a: &OMatrix<f64, Dyn, Dyn>) -> Self {
        let ed = EigenDecomposition::from_symmetric_real(&(a * a.transpose()));
        let sorted_epair = ed
            .eigenvalues
            .into_iter()
            .zip_eq(ed.eigenvectors)
            .sorted_by(|(a, _), (b, _)| a.total_cmp(b).reverse())
            .collect_vec();
        let relevant = sorted_epair
            .iter()
            .position(|(l, _)| l.abs() < 1e-10)
            .unwrap_or(a.shape().0.min(a.shape().1));
        let mut u = OMatrix::<f64, Dyn, Dyn>::zeros(a.shape().0, relevant);
        let mut s = OMatrix::<f64, Dyn, Dyn>::zeros(relevant, relevant);
        let mut ssqrt = OMatrix::<f64, Dyn, Dyn>::zeros(relevant, relevant);
        for (i, (l, v)) in sorted_epair.into_iter().take(relevant).enumerate() {
            u.set_column(i, &v);
            s[(i, i)] = l.sqrt();
            ssqrt[(i, i)] = l.sqrt().recip();
        }
        let v = a.transpose() * &u * &ssqrt;
        Self { u, s, v }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use itertools::Itertools;
    use nalgebra::Matrix2;

    use super::*;

    #[test]
    fn test_epair() {
        let cut: OMatrix<f64, Dyn, Dyn> =
            OMatrix::<f64, Dyn, Dyn>::from_vec(2, 2, vec![1.0f64, 1.0, 1.0, 0.0]).transpose();
        let EigenDecomposition {
            mut eigenvalues,
            mut eigenvectors,
        } = EigenDecomposition::from_symmetric_real(&cut);
        if eigenvalues[0] < eigenvalues[1] {
            eigenvalues.reverse();
            eigenvectors.reverse();
        }
        assert!((eigenvalues[0] - (1.0 + 5.0f64.sqrt()) / 2.0).abs() < 1e-6);
        assert!((eigenvalues[1] - (1.0 - 5.0f64.sqrt()) / 2.0).abs() < 1e-6);
        let mut v = Matrix2::zeros();
        v.set_column(0, &eigenvectors[0]);
        v.set_column(1, &eigenvectors[1]);
        assert!(
            (&cut * &v - &v * Matrix2::new(eigenvalues[0], 0.0, 0.0, eigenvalues[1])).norm() < 1e-6
        );
    }

    #[test]
    fn test_random_symmetric_eigen() {
        let mut this = Duration::ZERO;
        let mut other = Duration::ZERO;
        for _ in 0..100 {
            let a = OMatrix::<f64, Dyn, Dyn>::new_random(10, 10);
            let cut = a.transpose() * &a;
            let start = std::time::Instant::now();
            let ed = cut.clone().symmetric_eigen();
            other += start.elapsed();
            let start = std::time::Instant::now();
            let EigenDecomposition {
                eigenvalues,
                eigenvectors,
            } = EigenDecomposition::from_symmetric_real(&cut);
            this += start.elapsed();
            for ((eval1, evec1), (eval2, evec2)) in eigenvalues
                .clone()
                .into_iter()
                .zip_eq(eigenvectors)
                .sorted_by(|(a, _), (b, _)| a.total_cmp(b))
                .zip(
                    ed.eigenvalues
                        .data
                        .as_vec()
                        .iter()
                        .zip_eq(ed.eigenvectors.column_iter())
                        .sorted_by(|(a, _), (b, _)| a.total_cmp(b)),
                )
            {
                assert!(
                    (eval1 - eval2).abs() < 1e-5,
                    "this: {}, other: {}",
                    OVector::<f64, Dyn>::from_vec(
                        eigenvalues
                            .clone()
                            .into_iter()
                            .sorted_by(|a, b| a.total_cmp(b))
                            .collect_vec()
                    ),
                    OVector::<f64, Dyn>::from_vec(
                        ed.eigenvalues
                            .data
                            .as_vec()
                            .clone()
                            .into_iter()
                            .sorted_by(|a, b| a.total_cmp(b))
                            .collect_vec()
                    ),
                );
                assert!(
                    (evec1.transpose() * &evec1 - (evec1.transpose() * evec2).abs()).into_scalar()
                        < 1e-5,
                    "this: {evec1:?}, other: {:?}",
                    evec2.into_iter().collect_vec()
                );
            }
            println!("this: {:?}, other: {:?}", this, other);
        }
    }

    #[test]
    fn test_random_svd() {
        for _ in 0..10 {
            let cut = OMatrix::<f64, Dyn, Dyn>::new_random(5, 5);
            let svd = cut.clone().svd(false, false);
            let CompactSVD { u, s, v } = CompactSVD::new(&cut);
            for (s1, s2) in svd
                .singular_values
                .into_iter()
                .zip_eq(s.diagonal().into_iter())
            {
                assert!((s1 - s2).abs() < 1e-6);
            }
            assert!((u * &s * &v.transpose() - cut).norm() < 1e-6);
        }
    }
}
