use std::num::NonZeroUsize;
use faer::mat::MatRef;
use faer::{Accum, Mat, Par};
use faer::linalg::matmul;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two matrices using faer (parallel)
pub fn matmul(a: &Mat<f64>, b: &Mat<f64>) -> Mat<f64> {
    let mut c = Mat::<f64>::zeros(a.nrows(), b.ncols());
    let nthreads = NonZeroUsize::new(rayon::current_num_threads()).unwrap();
    matmul::matmul(&mut c, Accum::Replace, a, b, 1.0, Par::Rayon(nthreads));
    c
}

/// Multiply two matrices using faer with MatRef inputs (zero-copy from caller, parallel)
pub fn matmul_ref(a: MatRef<f64>, b: MatRef<f64>) -> Mat<f64> {
    let mut c = Mat::<f64>::zeros(a.nrows(), b.ncols());
    let nthreads = NonZeroUsize::new(rayon::current_num_threads()).unwrap();
    matmul::matmul(&mut c, Accum::Replace, a, b, 1.0, Par::Rayon(nthreads));
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
