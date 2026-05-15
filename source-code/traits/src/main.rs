mod matrix;

use clap::Parser;
use matrix::Matrix;

#[derive(Parser, Debug)]
#[command(version, about = "Create and play with matrices using traits")]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    rows: usize,

    #[arg(short, long, default_value_t = 4)]
    cols: usize,
}

fn main() {
    let args = Args::parse();
    let mut matrix = Matrix::new(args.rows, args.cols, 0.0_f64);

    for row in 0..matrix.rows() {
        for col in 0..matrix.cols() {
            matrix[(row, col)] = (row * matrix.cols() + col) as f64;
        }
    }

    println!("floating-point matrix:");
    println!("{matrix}");

    let integer_matrix =
        Matrix::try_from(vec![vec![1, 0], vec![0, 2]]).expect("all rows have the same length");

    println!("integer matrix:");
    println!("{integer_matrix}");
    println!(
        "integer matrix diagonal: {}, {}",
        integer_matrix[(0, 0)],
        integer_matrix[(1, 1)]
    );
}
