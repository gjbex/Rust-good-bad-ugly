use ndarray::{Array1, Array2, array, s};
use ndarray_linalg::{SVD, error::LinalgError};

const RECONSTRUCTION_TOLERANCE: f64 = 1.0e-12;

fn reconstruct_from_svd(matrix: &Array2<f64>) -> Result<(Array1<f64>, Array2<f64>), LinalgError> {
    let (u, singular_values, vt) = matrix.svd(true, true)?;
    let u = u.expect("left singular vectors were requested");
    let vt = vt.expect("right singular vectors were requested");

    let compact_dimension = singular_values.len();
    let u = u.slice(s![.., ..compact_dimension]);
    let vt = vt.slice(s![..compact_dimension, ..]);
    let sigma = Array2::from_diag(&singular_values);
    let reconstructed = u.dot(&sigma).dot(&vt);

    Ok((singular_values, reconstructed))
}

fn error_measures(difference: &Array2<f64>) -> (f64, f64) {
    let max_absolute_error = difference
        .iter()
        .fold(0.0_f64, |largest, &value| largest.max(value.abs()));
    let frobenius_error = difference.mapv(|value| value * value).sum().sqrt();

    (max_absolute_error, frobenius_error)
}

fn main() -> Result<(), LinalgError> {
    let original = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 7.0],
        [2.0, 6.0, 4.0],
        [3.0, 1.0, 8.0],
    ];

    let (singular_values, reconstructed) = reconstruct_from_svd(&original)?;
    let difference = &reconstructed - &original;
    let (max_absolute_error, frobenius_error) = error_measures(&difference);

    println!("Original matrix:\n{original}");
    println!("Singular values:\n{singular_values:.6}");
    println!("Reconstructed matrix:\n{reconstructed:.12}");
    println!("Difference matrix:\n{difference:.3e}");
    println!("Maximum absolute error: {max_absolute_error:.3e}");
    println!("Frobenius error: {frobenius_error:.3e}");

    assert!(
        max_absolute_error < RECONSTRUCTION_TOLERANCE,
        "reconstruction error {max_absolute_error:e} exceeds tolerance"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svd_reconstructs_a_rectangular_matrix() {
        let original = array![
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 7.0],
            [2.0, 6.0, 4.0],
            [3.0, 1.0, 8.0],
        ];

        let (singular_values, reconstructed) = reconstruct_from_svd(&original).unwrap();
        let difference = &reconstructed - &original;
        let (max_absolute_error, frobenius_error) = error_measures(&difference);

        assert_eq!(singular_values.len(), original.ncols());
        assert_eq!(reconstructed.dim(), original.dim());
        assert!(max_absolute_error < RECONSTRUCTION_TOLERANCE);
        assert!(frobenius_error < RECONSTRUCTION_TOLERANCE);
    }
}
