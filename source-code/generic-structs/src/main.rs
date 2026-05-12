mod matrix;

use clap::Parser;
use matrix::Matrix;

#[derive(Parser, Debug)]
#[command(version, about = "Create and play with generic matrices")]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    rows: usize,

    #[arg(short, long, default_value_t = 4)]
    cols: usize,
}

fn main() {
    let args = Args::parse();
    let mut matrix = Matrix::new(args.rows, args.cols, 0.0_f64);

    // Fill the floating-point matrix with some values.
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            matrix
                .set(i, j, (i * matrix.cols() + j) as f64)
                .expect("loop indices should be in bounds");
        }
    }

    // Print the floating-point matrix.
    for i in 0..matrix.rows() {
        for j in 0..matrix.cols() {
            let value = matrix.get(i, j).expect("loop indices should be in bounds");
            print!("{value:5.1} ");
        }
        println!();
    }

    let mut integer_matrix = Matrix::new(2, 2, 0_i32);
    integer_matrix
        .set(0, 0, 1)
        .expect("index should be in bounds");
    integer_matrix
        .set(1, 1, 2)
        .expect("index should be in bounds");
    println!(
        "integer matrix diagonal: {}, {}",
        integer_matrix.get(0, 0).expect("index should be in bounds"),
        integer_matrix.get(1, 1).expect("index should be in bounds")
    );
}
