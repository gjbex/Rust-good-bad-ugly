use std::{
    error::Error,
    f64::consts::TAU,
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use fftw_ffi::RealFft;
use num_complex::Complex64;

const SIGNAL_LENGTH: usize = 64;
const EXPECTED_PEAKS: [usize; 2] = [3, 7];
const ROUND_TRIP_TOLERANCE: f64 = 1.0e-12;

#[derive(Debug, Parser)]
#[command(version, about = "Transform a test signal with FFTW")]
struct Args {
    /// Write the sampled signal to this CSV file
    #[arg(long, value_name = "FILE")]
    signal_output: Option<PathBuf>,

    /// Write the one-sided power spectrum to this CSV file
    #[arg(long, value_name = "FILE")]
    spectrum_output: Option<PathBuf>,
}

fn validate_output_paths(args: &Args) -> io::Result<()> {
    if let (Some(signal_path), Some(spectrum_path)) = (&args.signal_output, &args.spectrum_output)
        && signal_path == spectrum_path
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "signal and spectrum output paths must be different",
        ));
    }
    Ok(())
}

fn create_signal(length: usize) -> Vec<f64> {
    (0..length)
        .map(|index| {
            let phase = TAU * index as f64 / length as f64;
            (3.0 * phase).sin() + 0.5 * (7.0 * phase).cos()
        })
        .collect()
}

fn one_sided_power_spectrum(spectrum: &[Complex64], signal_length: usize) -> Vec<f64> {
    let normalization = (signal_length as f64).powi(2);
    spectrum
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let is_dc = index == 0;
            let is_nyquist = signal_length.is_multiple_of(2) && index == signal_length / 2;
            let one_sided_factor = if is_dc || is_nyquist { 1.0 } else { 2.0 };
            one_sided_factor * value.norm_sqr() / normalization
        })
        .collect()
}

fn dominant_bins(power_spectrum: &[f64], count: usize) -> Vec<usize> {
    let mut bins: Vec<_> = power_spectrum.iter().copied().enumerate().skip(1).collect();
    bins.sort_by(|left, right| right.1.total_cmp(&left.1));

    let mut indices: Vec<_> = bins
        .into_iter()
        .take(count)
        .map(|(index, _)| index)
        .collect();
    indices.sort_unstable();
    indices
}

fn write_series_csv<W: Write>(
    mut writer: W,
    index_label: &str,
    value_label: &str,
    values: &[f64],
) -> io::Result<()> {
    writeln!(writer, "{index_label},{value_label}")?;
    for (index, value) in values.iter().enumerate() {
        writeln!(writer, "{index},{value:.17e}")?;
    }
    writer.flush()
}

fn write_series_file(
    path: &Path,
    index_label: &str,
    value_label: &str,
    values: &[f64],
) -> io::Result<()> {
    let writer = BufWriter::new(File::create(path)?);
    write_series_csv(writer, index_label, value_label, values)
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
    let args = Args::parse();
    validate_output_paths(&args)?;
    let signal = create_signal(SIGNAL_LENGTH);
    let mut transform = RealFft::new(signal.len())?;
    let spectrum = transform.forward(&signal)?;
    let power_spectrum = one_sided_power_spectrum(&spectrum, signal.len());
    let peaks = dominant_bins(&power_spectrum, EXPECTED_PEAKS.len());
    let reconstructed = transform.inverse(&spectrum)?;
    let (max_absolute_error, root_mean_square_error) = error_measures(&signal, &reconstructed);

    if let Some(path) = args.signal_output {
        write_series_file(&path, "sample", "value", &signal)?;
        println!("Signal written to {}", path.display());
    }
    if let Some(path) = args.spectrum_output {
        write_series_file(&path, "frequency_bin", "power", &power_spectrum)?;
        println!("Power spectrum written to {}", path.display());
    }

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
        let power_spectrum = one_sided_power_spectrum(&spectrum, signal.len());
        assert_eq!(spectrum.len(), SIGNAL_LENGTH / 2 + 1);
        assert_eq!(
            dominant_bins(&power_spectrum, EXPECTED_PEAKS.len()),
            EXPECTED_PEAKS
        );
        assert!((power_spectrum[3] - 0.5).abs() < ROUND_TRIP_TOLERANCE);
        assert!((power_spectrum[7] - 0.125).abs() < ROUND_TRIP_TOLERANCE);
        let mean_square =
            signal.iter().map(|value| value * value).sum::<f64>() / signal.len() as f64;
        assert!((power_spectrum.iter().sum::<f64>() - mean_square).abs() < ROUND_TRIP_TOLERANCE);

        let reconstructed = transform.inverse(&spectrum).unwrap();
        let (max_absolute_error, root_mean_square_error) = error_measures(&signal, &reconstructed);
        assert!(max_absolute_error < ROUND_TRIP_TOLERANCE);
        assert!(root_mean_square_error < ROUND_TRIP_TOLERANCE);
    }

    #[test]
    fn writes_indexed_values_as_csv() {
        let mut output = Vec::new();

        write_series_csv(&mut output, "sample", "value", &[1.0, -0.25]).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "sample,value\n0,1.00000000000000000e0\n1,-2.50000000000000000e-1\n"
        );
    }

    #[test]
    fn rejects_one_path_for_both_outputs() {
        let path = PathBuf::from("output.csv");
        let args = Args {
            signal_output: Some(path.clone()),
            spectrum_output: Some(path),
        };

        assert_eq!(
            validate_output_paths(&args).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
    }
}
