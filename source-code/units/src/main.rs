use clap::Parser;
use uom::si::f64::{Length, Time};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    x0: f64,

    #[clap(short, long)]
    t0: f64,

    #[clap(short, long)]
    delta_t: f64,
}

#[derive(Debug)]
struct SimulationParameters {
    x0: Length,
    t0: Time,
    delta_t: Time,
}

impl From<Args> for SimulationParameters {
    fn from(args: Args) -> Self {
        Self {
            x0: Length::new::<uom::si::length::meter>(args.x0),
            t0: Time::new::<uom::si::time::second>(args.t0),
            delta_t: Time::new::<uom::si::time::second>(args.delta_t),
        }
    }
}

fn simulate(t0: Time, delta_t: Time, x0: Length) -> Vec<(f64, f64)> {
    let t0 = t0.get::<uom::si::time::second>();
    let delta_t = delta_t.get::<uom::si::time::second>();
    let x0 = x0.get::<uom::si::length::meter>();

    let mut results = Vec::new();
    for i in 0..10 {
        let t = t0 + delta_t * i as f64;
        let x = x0 + t * 2.0; // Example: x = x0 + 2*t
        results.push((t, x));
    }
    results
}

fn main() {
    let args = Args::parse();
    let sim_params = SimulationParameters::from(args);
    println!("Parameters: {:?}", sim_params);
    let results = simulate(sim_params.t0, sim_params.delta_t, sim_params.x0);
    println!("Time (s), Position (m)");
    for (t, x) in results {
        println!("{t:.2}, {x:.2}");
    }
}
