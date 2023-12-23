use itertools::Itertools;
use nalgebra::{Dyn, OMatrix, OVector};

pub struct EigenDecomposition {
    pub eigenvalues: OVector<f64, Dyn>,
    pub eigenvectors: OMatrix<f64, Dyn, Dyn>,
}
impl EigenDecomposition {
    pub fn max_epair(a: &OMatrix<f64, Dyn, Dyn>) -> (f64, OVector<f64, Dyn>) {
        assert!(a.is_square());
        let mut evector = OVector::<f64, Dyn>::from_fn(a.shape().0, |i, _| i as f64).normalize();
        let mut evalue = 1.0f64;
        loop {
            let Some(ev) = (a - &OMatrix::<f64, Dyn, Dyn>::from_diagonal_element(
                a.shape().0,
                a.shape().0,
                evalue,
            ))
                .try_inverse()
                .map(|m| (m * &evector).normalize())
            else {
                break;
            };
            evector = ev;
            let lv = a * &evector;
            evalue = (evector.transpose() * &lv).into_scalar();
            if (lv - evalue * &evector).norm() < 1e-6 {
                break;
            }
        }
        (evalue, evector)
    }

    pub fn from_symmetric_real(a: &OMatrix<f64, Dyn, Dyn>) -> Self {
        assert!(a.is_square());
        let (_, max_v) = Self::max_epair(a);
        let mut basis = (0..a.shape().0)
            .cartesian_product(0..a.shape().0)
            .map(|(i, j)| if i == j { 1.0 } else { 0.0 })
            .collect_vec();
        for i in (0..a.shape().0).rev() {
            basis.insert(i * a.shape().0, max_v[i]);
        }
        let basis =
            OMatrix::<f64, Dyn, Dyn>::from_vec(a.shape().0 + 1, a.shape().0, basis).transpose();
        let basis = basis.qr().q();
        let mut done = 1;
        let mut evalues = (basis.transpose() * a * &basis).diagonal();
        while done < a.shape().0 - 1 {
            // rotate around basis.columns_range(..done) by 1 deg clockwise
            let mut lambda = basis.transpose() * a * &basis;
            evalues = lambda.diagonal();
            lambda.fill_diagonal(0.0);
            if lambda.columns_range(..done + 1).norm() < 1e-6 {
                done += 1;
            }
        }
        return Self {
            eigenvalues: evalues,
            eigenvectors: basis,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epair() {
        let cut: OMatrix<f64, Dyn, Dyn> =
            OMatrix::<f64, Dyn, Dyn>::from_vec(2, 2, vec![1.0f64, 1.0, 1.0, 0.0]).transpose();
        let (l, v) = EigenDecomposition::max_epair(&cut);
        assert!((&cut * &v - l * &v).norm() < 1e-6);
        assert!((l - (1.0 + 5.0f64.sqrt()) / 2.0).abs() < 1e-6);

        EigenDecomposition::from_symmetric_real(&cut);
    }
}
