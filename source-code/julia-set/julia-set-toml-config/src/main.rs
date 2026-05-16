mod matrix;

use std::{error::Error, fs, path::PathBuf};

use clap::Parser;
use matrix::Matrix;
use num_complex::Complex64;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "Julia Set Generator", version)]
struct Args {
    config_file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Config {
    max_iterations: usize,
    width: usize,
    height: usize,
    c_real: f64,
    c_imag: f64,
}

fn read_config(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    let config_text = fs::read_to_string(path)?;
    let config = toml::from_str(&config_text)?;
    Ok(config)
}

fn initialize_z(rows: usize, cols: usize) -> Matrix<Complex64> {
    let mut z = Matrix::new(rows, cols, Complex64::new(0.0, 0.0));
    let domain_min = -2.0;
    let domain_max = 2.0;
    let delta_re = (domain_max - domain_min) / (cols as f64);
    let delta_im = (domain_max - domain_min) / (rows as f64);
    for i in 0..rows {
        for j in 0..cols {
            let z_value = Complex64::new(
                domain_min + j as f64 * delta_re,
                domain_min + i as f64 * delta_im,
            );
            z.set(i, j, z_value)
                .expect("loop indices should be in bounds");
        }
    }
    z
}

fn iterate_z_value(z: Complex64, c: Complex64, max_iterations: usize) -> usize {
    let mut z_n = z;
    for n in 0..max_iterations {
        if z_n.norm() > 2.0 {
            return n;
        }
        z_n = z_n * z_n + c;
    }
    max_iterations
}

fn iterate_z_matrix(z: &Matrix<Complex64>, c: Complex64, max_iterations: usize) -> Matrix<usize> {
    let mut result = Matrix::new(z.rows(), z.cols(), 0);
    for i in 0..z.rows() {
        for j in 0..z.cols() {
            let z_value = *z.get(i, j).expect("loop indices should be in bounds");
            let iterations = iterate_z_value(z_value, c, max_iterations);
            result
                .set(i, j, iterations)
                .expect("loop indices should be in bounds");
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = read_config(args.config_file)?;
    let c = Complex64::new(config.c_real, config.c_imag);
    let z = initialize_z(config.height, config.width);
    let result = iterate_z_matrix(&z, c, config.max_iterations);
    for i in 0..result.rows() {
        for j in 0..result.cols() {
            print!(
                "{:3} ",
                result.get(i, j).expect("loop indices should be in bounds")
            );
        }
        println!();
    }
    Ok(())
}
