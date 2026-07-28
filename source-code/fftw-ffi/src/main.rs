use std::{error::Error, f64::consts::TAU};

use fftw_ffi::RealFft;
use num_complex::Complex64;

const SIGNAL_LENGTH: usize = 64;
const EXPECTED_PEAKS: [usize; 2] = [3, 7];
const ROUND_TRIP_TOLERANCE: f64 = 1.0e-12;

fn create_signal(length: usize) -> Vec<f64> {
    (0..length)
        .map(|index| {
            let phase = TAU * index as f64 / length as f64;
            (3.0 * phase).sin() + 0.5 * (7.0 * phase).cos()
        })
        .collect()
}

fn dominant_bins(spectrum: &[Complex64], count: usize) -> Vec<usize> {
    let mut bins: Vec<_> = spectrum
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, value)| (index, value.norm_sqr()))
        .collect();
    bins.sort_by(|left, right| right.1.total_cmp(&left.1));

    let mut indices: Vec<_> = bins
        .into_iter()
        .take(count)
        .map(|(index, _)| index)
        .collect();
    indices.sort_unstable();
    indices
}

fn error_measures(original: &[f64], reconstructed: &[f64]) -> (f64, f64) {
    let (max_absolute_error, squared_error) = original.iter().zip(reconstructed).fold(
        (0.0_f64, 0.0_f64),
        |(largest, sum), (&expected, &actual)| {
            let difference = actual - expected;
            (largest.max(difference.abs()), sum + difference.powi(2))
        },
    );
    let root_mean_square_error = (squared_error / original.len() as f64).sqrt();
    (max_absolute_error, root_mean_square_error)
}

fn main() -> Result<(), Box<dyn Error>> {
    let signal = create_signal(SIGNAL_LENGTH);
    let mut transform = RealFft::new(signal.len())?;
    let spectrum = transform.forward(&signal)?;
    let peaks = dominant_bins(&spectrum, EXPECTED_PEAKS.len());
    let reconstructed = transform.inverse(&spectrum)?;
    let (max_absolute_error, root_mean_square_error) = error_measures(&signal, &reconstructed);

    println!("Signal length: {}", signal.len());
    println!(
        "Non-redundant spectrum length: {}",
        transform.spectrum_len()
    );
    println!("Dominant frequency bins: {peaks:?}");
    println!("Maximum round-trip error: {max_absolute_error:.3e}");
    println!("RMS round-trip error: {root_mean_square_error:.3e}");

    assert_eq!(peaks, EXPECTED_PEAKS);
    assert!(max_absolute_error < ROUND_TRIP_TOLERANCE);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_frequencies_and_reconstructs_the_signal() {
        let signal = create_signal(SIGNAL_LENGTH);
        let mut transform = RealFft::new(signal.len()).unwrap();

        let spectrum = transform.forward(&signal).unwrap();
        assert_eq!(spectrum.len(), SIGNAL_LENGTH / 2 + 1);
        assert_eq!(
            dominant_bins(&spectrum, EXPECTED_PEAKS.len()),
            EXPECTED_PEAKS
        );

        let reconstructed = transform.inverse(&spectrum).unwrap();
        let (max_absolute_error, root_mean_square_error) = error_measures(&signal, &reconstructed);
        assert!(max_absolute_error < ROUND_TRIP_TOLERANCE);
        assert!(root_mean_square_error < ROUND_TRIP_TOLERANCE);
    }
}
