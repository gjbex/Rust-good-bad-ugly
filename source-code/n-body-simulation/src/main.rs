mod system;

use clap::Parser;
use serde::Serialize;
use system::System;

#[derive(Parser, Debug)]
#[command(version, about = "A simple n-body simulation")]
struct Args {
    /// Number of particles in the simulation
    #[arg(long, default_value_t = 100)]
    num_particles: usize,

    /// Random seed for initializing the system
    #[arg(long, default_value_t = 1234)]
    seed: u64,

    /// Delta time for the simulation steps
    #[arg(long, default_value_t = 0.001)]
    delta_time: f64,

    /// Number of simulation steps to run
    #[arg(long = "steps", default_value_t = 100)]
    num_steps: usize,

    /// Gravitational softening length
    #[arg(long, default_value_t = 0.01)]
    softening: f64,

    /// File to save the evolution of the system in CSV format (energy and center of mass at each
    /// step)
    #[arg(long)]
    save_evolution: Option<String>,

    /// File to save the states of the system in CSV format, one row per particle per step
    #[arg(long)]
    save_states: Option<String>,
}

#[derive(Serialize)]
struct EvolutionRecord {
    step: usize,
    potential_energy: f64,
    kinetic_energy: f64,
    total_energy: f64,
    center_of_mass_x: f64,
    center_of_mass_y: f64,
    center_of_mass_z: f64,
}

#[derive(Serialize)]
struct ParticleStateRecord {
    step: usize,
    particle: usize,
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    mass: f64,
}

fn evolution_record(step: usize, system: &System) -> EvolutionRecord {
    let com = system.center_of_mass();
    EvolutionRecord {
        step,
        potential_energy: system.potential_energy(),
        kinetic_energy: system.kinetic_energy(),
        total_energy: system.total_energy(),
        center_of_mass_x: com.0,
        center_of_mass_y: com.1,
        center_of_mass_z: com.2,
    }
}

fn write_evolution_record(
    writer: &mut Option<csv::Writer<std::fs::File>>,
    step: usize,
    system: &System,
) {
    if let Some(writer) = writer.as_mut() {
        writer
            .serialize(evolution_record(step, system))
            .expect("Failed to write evolution record");
    }
}

fn write_state_records(
    writer: &mut Option<csv::Writer<std::fs::File>>,
    step: usize,
    system: &System,
) {
    if let Some(writer) = writer.as_mut() {
        for state in system.particle_states() {
            writer
                .serialize(ParticleStateRecord {
                    step,
                    particle: state.index,
                    x: state.x,
                    y: state.y,
                    z: state.z,
                    vx: state.vx,
                    vy: state.vy,
                    vz: state.vz,
                    mass: state.mass,
                })
                .expect("Failed to write particle state record");
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut system = System::new(args.num_particles, args.seed, args.softening);

    let mut evolution_writer = args
        .save_evolution
        .as_deref()
        .map(|filename| csv::Writer::from_path(filename).expect("Failed to create evolution file"));
    let mut states_writer = args
        .save_states
        .as_deref()
        .map(|filename| csv::Writer::from_path(filename).expect("Failed to create states file"));

    write_evolution_record(&mut evolution_writer, 0, &system);
    write_state_records(&mut states_writer, 0, &system);
    for step in 1..=args.num_steps {
        system.update(args.delta_time);
        write_evolution_record(&mut evolution_writer, step, &system);
        write_state_records(&mut states_writer, step, &system);
    }

    if let Some(mut writer) = evolution_writer {
        writer.flush().expect("Failed to flush evolution file");
    }
    if let Some(mut writer) = states_writer {
        writer.flush().expect("Failed to flush states file");
    }
}
