use std::error::Error;

use clap::Parser;
use cpp_interpolation_ffi::LinearInterpolator;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Interpolate tabulated data with a wrapped C++ library"
)]
struct Args {
    /// Coordinate at which to evaluate the interpolator
    #[arg(short, long, default_value_t = 1.5)]
    query: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let coordinates = [0.0, 1.0, 2.0, 4.0];
    let values = [0.0, 1.0, 4.0, 16.0];
    let interpolator = LinearInterpolator::new(&coordinates, &values)?;
    let value = interpolator.evaluate(args.query)?;

    println!("Interpolated value at x = {}: {value}", args.query);
    Ok(())
}
