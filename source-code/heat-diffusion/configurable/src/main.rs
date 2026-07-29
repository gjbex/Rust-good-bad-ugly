mod config;
mod heat_diffusion;

use std::{error::Error, path::PathBuf};

use clap::Parser;
use config::Config;
use heat_diffusion::System;

#[derive(Debug, Parser)]
#[command(version, about = "Run a configured heat-diffusion simulation")]
struct Args {
    /// Path to a TOML configuration file
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,

    /// Print the final temperature grid
    #[arg(long)]
    show: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = Config::load(args.config)?;

    let mut simulation = System::new(config.grid.size, config.material.thermal_diffusivity)?;
    simulation.initialize_grid(
        config.grid.background_temperature,
        config.grid.boundary_temperature,
        &config.initial_condition,
    )?;

    let steps_taken = simulation.run_simulation(
        config.solver.time_step,
        config.solver.max_steps,
        config.solver.tolerance,
    )?;
    println!("Simulation completed in {steps_taken} steps.");
    if args.show {
        println!("{simulation}");
    }
    Ok(())
}
