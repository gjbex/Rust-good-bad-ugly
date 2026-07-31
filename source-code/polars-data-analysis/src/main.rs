use clap::Parser;
use polars::prelude::*;
use polars_data_analysis::scan_patient_data;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about = "Summarize patient measurements with a lazy Polars query")]
struct Cli {
    /// CSV file containing patient, gender, and condition columns.
    #[arg(long, default_value = "data/patient_metadata.csv")]
    metadata: PathBuf,

    /// CSV file containing patient, dose, date, and temperature columns.
    #[arg(long, default_value = "data/patient_experiment.csv")]
    measurements: PathBuf,

    /// Show Polars' optimized query plan before executing it.
    #[arg(long)]
    show_plan: bool,

    /// Write the resulting table to this Parquet file.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let query = scan_patient_data(&cli.metadata, &cli.measurements)?;

    if cli.show_plan {
        println!("Optimized query plan:\n{}", query.clone().explain(true)?);
    }

    let mut summary = query.collect()?;
    println!("Patient summary:\n{summary}");

    if let Some(output) = cli.output {
        let mut file = File::create(&output)?;
        ParquetWriter::new(&mut file).finish(&mut summary)?;
        println!("Wrote {}", output.display());
    }

    Ok(())
}
