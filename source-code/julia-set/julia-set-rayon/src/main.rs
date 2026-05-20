mod matrix;

use clap::Parser;
use matrix::Matrix;
use num_complex::Complex64;
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "Parallel Julia Set Generator", version)]
struct Args {
    #[arg(short, long, default_value_t = 1000)]
    max_iterations: usize,
    #[arg(short = 'x', long, default_value_t = 800)]
    width: usize,
    #[arg(short = 'y', long, default_value_t = 600)]
    height: usize,
    #[arg(short = 'r', long, default_value_t = -0.5125)]
    c_real: f64,
    #[arg(short = 'i', long, default_value_t = 0.5213)]
    c_imag: f64,
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
    let rows = z.rows();
    let cols = z.cols();
    let data: Vec<usize> = (0..rows * cols)
        .into_par_iter()
        .with_min_len(1000)
        .map(|index| {
            let row = index / cols;
            let col = index % cols;
            let z_value = *z.get(row, col).expect("flat index should be in bounds");
            iterate_z_value(z_value, c, max_iterations)
        })
        .collect();

    Matrix::from_vec(rows, cols, data).expect("parallel result should match matrix shape")
}

fn main() {
    let args = Args::parse();
    let c = Complex64::new(args.c_real, args.c_imag);
    let z = initialize_z(args.height, args.width);
    let result = iterate_z_matrix(&z, c, args.max_iterations);
    for i in 0..result.rows() {
        for j in 0..result.cols() {
            print!(
                "{:3} ",
                result.get(i, j).expect("loop indices should be in bounds")
            );
        }
        println!();
    }
}
