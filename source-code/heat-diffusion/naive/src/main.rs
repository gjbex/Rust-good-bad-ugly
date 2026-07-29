mod heat_diffusion;

use clap::Parser;
use heat_diffusion::System;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    grid_size: usize,
    #[arg(short, long, default_value_t = 0.1)]
    alpha: f64,
    #[arg(short = 'T', long, default_value_t = 100.0)]
    spot_temperature: f64,
    #[arg(short = 'r', long, default_value_t = 5)]
    spot_radius: usize,
    #[arg(short, long, default_value_t = 20.0)]
    boundary_temperature: f64,
    #[arg(short, long, default_value_t = 0.01)]
    dt: f64,
    #[arg(short, long, default_value_t = 1000)]
    steps: usize,
    #[arg(short, long, default_value_t = 0.001)]
    tolerance: f64,
    #[arg(long)]
    show: bool,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();

    let mut simulation = System::new(args.grid_size, args.alpha)?;
    simulation.initialize_grid(
        args.spot_temperature,
        args.spot_radius,
        args.boundary_temperature,
    )?;

    let steps_taken = simulation.run_simulation(args.dt, args.steps, args.tolerance)?;
    println!("Simulation completed in {} steps.", steps_taken);
    if args.show {
        println!("{simulation}");
    }
    Ok(())
}
