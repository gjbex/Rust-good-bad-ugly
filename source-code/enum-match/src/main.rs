mod simpson;
mod gauss;

use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
enum QuadratureMethod {
    Simpson,
    Gauss,
}

#[derive(Parser, Debug)]
#[command(version, about="Numerical integration using different methods")]
struct Args {
    /// Integration method to use
    #[arg(short, long, default_value = "simpson")]
    method: QuadratureMethod,
}

fn main() {
    let args = Args::parse();
    let f = |x: f64| x.sin(); // Example function to integrate
    let a = 0.0; // Lower limit
    let b = std::f64::consts::PI; // Upper limit

    // Perform integration based on the selected
    // method
    let result = match args.method {
        QuadratureMethod::Simpson => simpson::quad(f, a, b, 1000), // Using 1000 subdivisions for Simpson's rule
        QuadratureMethod::Gauss => gauss::quad(f, a, b),
    };

    println!("The integral of sin(x) from {a:.1} to {b:.5} is approximately {result:.5}");
}
