mod matrix;
use matrix::Matrix;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Create and play with matrices")]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    rows: usize,

    #[arg(short, long, default_value_t = 4)]
    cols: usize,
}

fn main() {
    let args = Args::parse();
    let mut matrix = Matrix::new(args.rows, args.cols);

    // Fill the matrix with some values
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            matrix.set(i, j, (i * matrix.cols() + j) as f64);
        }
    }

    // Print the matrix
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            print!("{:5.1} ", matrix.get(i, j));
        }
        println!();
    }
}
