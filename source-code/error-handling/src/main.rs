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
            matrix
                .set(i, j, (i * matrix.cols() + j) as f64)
                .expect("loop indices should be in bounds");
        }
    }

    // Print the matrix
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            let value = matrix.get(i, j).expect("loop indices should be in bounds");
            print!("{value:5.1} ");
        }
        println!();
    }
}
