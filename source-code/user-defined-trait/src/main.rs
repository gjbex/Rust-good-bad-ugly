mod gauss;
mod quadrature;
mod simpson;

use clap::{Parser, ValueEnum};
use gauss::GaussLegendre10;
use quadrature::QuadratureRule;
use simpson::Simpson;

#[derive(Clone, Debug, ValueEnum)]
enum QuadratureMethod {
    Simpson,
    Gauss,
}

#[derive(Parser, Debug)]
#[command(version, about = "Numerical integration using a user-defined trait")]
struct Args {
    /// Integration method to use
    #[arg(short, long, default_value = "simpson")]
    method: QuadratureMethod,

    /// Number of subdivisions for Simpson's rule
    #[arg(short, long, default_value_t = 1000)]
    subdivisions: usize,
}

fn select_rule(args: &Args) -> Box<dyn QuadratureRule> {
    match args.method {
        QuadratureMethod::Simpson => Box::new(Simpson::new(args.subdivisions)),
        QuadratureMethod::Gauss => Box::new(GaussLegendre10),
    }
}

fn main() {
    let args = Args::parse();
    let rule = select_rule(&args);
    let f = |x: f64| x.sin();
    let a = 0.0;
    let b = std::f64::consts::PI;

    let result = rule.integrate(&f, a, b);

    println!(
        "The integral of sin(x) from {a:.1} to {b:.5} is approximately {result:.5} using {} quadrature",
        rule.name()
    );
}
