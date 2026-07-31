use clap::Parser;
use hdf5_snapshot::{gaussian_snapshot, inspect_snapshot, write_snapshot};
use std::path::PathBuf;

const NX: usize = 65;
const NY: usize = 49;

#[derive(Debug, Parser)]
#[command(about = "Write and inspect a scientific HDF5 snapshot")]
struct Cli {
    /// HDF5 file to create or replace.
    #[arg(long, default_value = "temperature-snapshot.h5")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let snapshot = gaussian_snapshot(NX, NY);

    write_snapshot(&cli.output, &snapshot, 12.5, 42)?;
    let info = inspect_snapshot(&cli.output)?;
    let center_mean = info.center.iter().sum::<f64>() / info.center.len() as f64;

    println!("Wrote {}", cli.output.display());
    println!("Description: {}", info.description);
    println!("Temperature dataset: {:?} {}", info.shape, info.units);
    println!("Chunk shape: {:?}", info.chunk_shape);
    println!("Filters: {:?}", info.filters);
    println!(
        "Simulation time: {:.1} s (step {})",
        info.time_seconds, info.step
    );
    println!(
        "Temperature range: {:.3} to {:.3} {}",
        info.minimum_temperature, info.maximum_temperature, info.units
    );
    println!("Central hyperslab mean: {center_mean:.3} {}", info.units);

    Ok(())
}
