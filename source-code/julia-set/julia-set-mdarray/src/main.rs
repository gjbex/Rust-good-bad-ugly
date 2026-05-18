use clap::Parser;
use mdarray::{DArray, DSlice};
use num_complex::Complex64;

#[derive(Parser)]
#[command(name = "Mandelbrot Set Generator", version)]
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

type MatrixC = DArray<Complex64, 2>;
type MatrixCSlice = DSlice<Complex64, 2>;
type MatrixI = DArray<usize, 2>;

fn initialize_z(rows: usize, cols: usize) -> MatrixC {
    let mut z = MatrixC::from_elem([rows, cols], Complex64::new(0.0, 0.0));
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
            z[[i, j]] = z_value;
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

fn iterate_z_matrix(z: &MatrixCSlice, c: Complex64, max_iterations: usize) -> MatrixI {
    let mut result = MatrixI::from_elem([z.dim(0), z.dim(1)], 0);
    for i in 0..z.dim(0) {
        for j in 0..z.dim(1) {
            let z_value = z[[i, j]];
            let iterations = iterate_z_value(z_value, c, max_iterations);
            result[[i, j]] = iterations;
        }
    }
    result
}

fn main() {
    let args = Args::parse();
    let c = Complex64::new(args.c_real, args.c_imag);
    let z = initialize_z(args.height, args.width);
    let result = iterate_z_matrix(&z, c, args.max_iterations);
    for i in 0..result.dim(0) {
        for j in 0..result.dim(1) {
            print!("{:3} ", result[[i, j]]);
        }        println!();
    }
}
