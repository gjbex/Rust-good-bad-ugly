use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Polynomial Calculator",
    version,
    about = "Calculates the value of a polynomial of degree 2 for the given coefficients on the interval [-5.0, 5.0]"
)]

struct Args {
    /// Coefficient a (for x^2)
    #[arg(short, long, default_value_t = 1.0)]
    a: f64,

    /// Coefficient b (for x)
    #[arg(short, long, default_value_t = 0.0)]
    b: f64,

    /// Coefficient c (constant term)
    #[arg(short, long, default_value_t = 0.0)]
    c: f64,
}

fn polynomial(x: f64, a: f64, b: f64, c:f64) -> f64 {
    a * x.powi(2) + b * x + c
}

fn main() {
    let args = Args::parse();
    let x_min = -5.0;
    let x_max = 5.0;
    let nr_points = 20;
    let mut x = x_min;
    let delta_x = (x_max - x_min) / (nr_points as f64 - 1.0);
    for i in 0..nr_points {
        let result = polynomial(x, args.a, args.b, args.c);
        println!("{} {}", x, result);
        x += delta_x;
    }
}
