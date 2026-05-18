use clap::Parser;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Values {
    x: f64,
    y: f64,
}

#[derive(Parser, Debug)]
#[command(author, version, about= "Process data file")]
struct Args {
    /// Input CSV file containing x and y values
    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut reader = csv::Reader::from_path(args.file)?;
    for result in reader.deserialize() {
        let value: Values = result?;
        xs.push(value.x);
        ys.push(value.y);
    }
    println!("x values: {:?}", xs);
    println!("y values: {:?}", ys);

    // filter out x values larger than or equal to 10.0
    let filtered_xs: Vec<f64> = xs
        .iter()
        .copied()
        .filter(|x| *x >= 10.0)
        .collect();
    println!("Filtered x values (>= 10.0): {:?}", filtered_xs);

    // treat (x, y) pairs, filter those for which x is larger than or equal to 10.0
    let filtered_pairs: Vec<(f64, f64)> = xs
        .iter()
        .copied()
        .zip(ys.iter().copied())
        .filter(|(x, _)| *x >= 10.0)
        .collect();
    println!("Filtered (x, y) pairs (x >= 10.0): {:?}", filtered_pairs);

    // unpack the filtered pairs into separate vectors
    let (filtered_xs_unpacked, filtered_ys_unpacked): (Vec<f64>, Vec<f64>) = filtered_pairs
        .iter()
        .copied()
        .unzip();
    println!("Unpacked filtered x values: {:?}", filtered_xs_unpacked);
    println!("Unpacked filtered y values: {:?}", filtered_ys_unpacked);

    // compute the cube of the x values
    let cubed_xs: Vec<f64> = xs
        .iter()
        .copied()
        .map(|x| x.powi(3))
        .collect();
    println!("Cubed x values: {:?}", cubed_xs);

    // compute the sum of the y values
    let sum_y: f64 = ys.iter().sum();
    println!("Sum of y values: {}", sum_y);

    // create pairs of (i, x) values where i is the index of the x value
    let indexed_xs: Vec<(usize, f64)> = xs
        .iter()
        .copied()
        .enumerate()
        .collect();
    println!("Indexed x values: {:?}", indexed_xs);

    // print the index and x value for each x value
    for (i, y) in ys.iter().enumerate() {
        println!("Index: {i}, y value: {y:.1}");
    }
    Ok(())
}
