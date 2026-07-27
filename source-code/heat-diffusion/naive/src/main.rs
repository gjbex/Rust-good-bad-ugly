mod heat_diffusion;

use clap::Parser;
use heat_diffusion::System;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "100")]
    grid_size: usize,
    #[clap(short, long, default_value = "0.1")]
    alpha: f64,
    #[clap(short = 'T', long, default_value = "100.0")]
    spot_temperature: f64,
    #[clap(short = 'r', long, default_value = "5")]
    spot_radius: usize,
    #[clap(short, long, default_value = "20.0")]
    boundary_temperature: f64,
    #[clap(short, long, default_value = "0.01")]
    dt: f64,
    #[clap(short, long, default_value = "1000")]
    steps: usize,
    #[clap(short, long, default_value = "0.001")]
    tolerance: f64,
    #[clap(long)]
    show: bool,
}

fn main() {
    let args = Args::parse();

    let mut simulation = System::new(args.grid_size, args.alpha);
    simulation
        .initialize_grid(
            args.spot_temperature,
            args.spot_radius,
            args.boundary_temperature,
        )
        .expect("Failed to initialize grid");

    let steps_taken = simulation.run_simulation(args.dt, args.steps, args.tolerance);
    println!("Simulation completed in {} steps.", steps_taken);
    if args.show {
        println!("{simulation}");
    }
}
