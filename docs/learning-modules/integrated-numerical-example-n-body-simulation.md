# Integrated Numerical Example: N-Body Simulation

This module uses the softened gravitational N-body simulation as an integrated
numerical example. Like the Julia set module, it combines many earlier topics
in one program. Unlike the Julia set example, it evolves a system in time,
tracks diagnostics, and writes structured output for later analysis.

The goal is to study how a small simulation program is organized in Rust:
state representation, random initialization, time integration, diagnostics,
CSV output, and visualization.

## Learning Objectives

After completing this module, participants should be able to:

- Explain how simulation state is stored in a Rust struct.
- Use private fields and methods to encapsulate simulation state.
- Identify where random initialization enters the program.
- Explain the role of gravitational softening.
- Follow a velocity Verlet update step.
- Compute basic simulation diagnostics such as energy and center of mass.
- Write structured CSV output with `csv` and `serde`.
- Distinguish evolution output from particle-state output.
- Use Python helper scripts to inspect numerical results.
- Discuss how time step, softening, and initialization affect diagnostics.

## Prerequisites

Participants should already be comfortable with:

- Structs, methods, and associated functions.
- Ownership and borrowing.
- Vectors and iteration.
- Random number generation with explicit seeds.
- Command-line parsing with `clap`.
- Error handling and `expect`.
- CSV output and basic Python visualization workflows.

The example used in this module is:

- `source-code/n-body-simulation/rust`

## What The Simulation Does

The program simulates particles interacting through softened gravity. Each
particle has:

- position: `x`, `y`, `z`;
- velocity: `vx`, `vy`, `vz`;
- mass.

The system is initialized randomly from a seed. It is then advanced for a fixed
number of time steps. Optionally, the program writes CSV files containing:

- energy and center-of-mass diagnostics at each step;
- particle positions, velocities, and masses at each step.

Run the default simulation with:

```bash
cd source-code/n-body-simulation/rust
cargo run
```

## Command-Line Parameters

The simulation is configured with command-line arguments:

```rust
#[derive(Parser, Debug)]
#[command(version, about = "A simple n-body simulation")]
struct Args {
    #[arg(long, default_value_t = 100)]
    num_particles: usize,

    #[arg(long, default_value_t = 1234)]
    seed: u64,

    #[arg(long, default_value_t = 0.001)]
    delta_time: f64,

    #[arg(long = "steps", default_value_t = 100)]
    num_steps: usize,

    #[arg(long, default_value_t = 0.01)]
    softening: f64,

    #[arg(long)]
    save_evolution: Option<String>,

    #[arg(long)]
    save_states: Option<String>,
}
```

This connects earlier CLI material to a more realistic program. Some
parameters control the model, such as particle count and softening. Others
control the run, such as time step, number of steps, seed, and output files.

## Representing Simulation State

The simulation state is stored in a `System` struct:

```rust
pub struct System {
    xs: Vec<f64>,
    ys: Vec<f64>,
    zs: Vec<f64>,
    vxs: Vec<f64>,
    vys: Vec<f64>,
    vzs: Vec<f64>,
    masses: Vec<f64>,
    softening_length: f64,
}
```

The fields are private. Other code does not directly modify the vectors.
Instead, it uses methods such as:

- `System::new`;
- `update`;
- `potential_energy`;
- `kinetic_energy`;
- `total_energy`;
- `center_of_mass`;
- `particle_states`.

This is the same encapsulation idea from the matrix examples, applied to a
time-dependent physical system.

## Random Initialization

The constructor initializes the system from a seed:

```rust
pub fn new(num_particles: usize, seed: u64, softening_length: f64) -> Self {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    // allocate vectors and sample values
}
```

The example uses distributions for positions, velocities, and masses:

```rust
let position_distribution =
    Uniform::new(0.0, 1.0).expect("position distribution bounds should be valid");
let velocity_distribution =
    Normal::new(0.0, 1.0).expect("velocity distribution parameters should be valid");
let mass_distribution =
    Uniform::new(0.1, 1.0).expect("mass distribution bounds should be valid");
```

The seed is part of the run configuration. Using the same seed and parameters
recreates the same initial condition.

## Gravitational Softening

The acceleration calculation includes a softening length:

```rust
let distance_squared =
    dx * dx + dy * dy + dz * dz + self.softening_length * self.softening_length;
```

Softening prevents the force from becoming extremely large when two particles
come very close together. This is a numerical modeling choice, not just a Rust
implementation detail.

The same softening length is used in the potential-energy calculation:

```rust
let distance =
    (dx * dx + dy * dy + dz * dz + self.softening_length * self.softening_length)
        .sqrt();
```

Using consistent softening in force and energy diagnostics is important when
interpreting energy conservation.

## Acceleration Calculation

The acceleration on one particle is computed by summing contributions from all
other particles:

```rust
fn acceleration_on(&self, index: usize) -> (f64, f64, f64) {
    let mut acceleration = (0.0, 0.0, 0.0);
    for i in 0..self.num_particles() {
        if i != index {
            let dx = self.xs[i] - self.xs[index];
            let dy = self.ys[i] - self.ys[index];
            let dz = self.zs[i] - self.zs[index];
            let distance_squared =
                dx * dx + dy * dy + dz * dz + self.softening_length * self.softening_length;
            let distance = distance_squared.sqrt();
            let acceleration_magnitude = self.masses[i] / (distance_squared * distance);
            acceleration.0 += acceleration_magnitude * dx;
            acceleration.1 += acceleration_magnitude * dy;
            acceleration.2 += acceleration_magnitude * dz;
        }
    }
    acceleration
}
```

The return value is a 3-tuple:

```rust
(f64, f64, f64)
```

This is a compact local representation for the three acceleration components.
In a larger code base, a named vector type might be clearer.

## Velocity Verlet Update

The `update` method uses a velocity Verlet step. First it computes the current
accelerations:

```rust
let accelerations = self.accelerations();
let half_dt_squared = 0.5 * dt * dt;
```

Then it updates positions:

```rust
for i in 0..self.num_particles() {
    let (ax, ay, az) = accelerations[i];
    self.xs[i] += self.vxs[i] * dt + ax * half_dt_squared;
    self.ys[i] += self.vys[i] * dt + ay * half_dt_squared;
    self.zs[i] += self.vzs[i] * dt + az * half_dt_squared;
}
```

After the positions change, it recomputes accelerations:

```rust
let new_accelerations = self.accelerations();
```

Finally it updates velocities using the average of old and new accelerations:

```rust
for i in 0..self.num_particles() {
    let (ax, ay, az) = accelerations[i];
    let (new_ax, new_ay, new_az) = new_accelerations[i];
    self.vxs[i] += 0.5 * (ax + new_ax) * dt;
    self.vys[i] += 0.5 * (ay + new_ay) * dt;
    self.vzs[i] += 0.5 * (az + new_az) * dt;
}
```

This structure is useful pedagogically because it separates the numerical
algorithm into visible stages.

## Diagnostics

The system exposes methods for energy diagnostics:

```rust
pub fn kinetic_energy(&self) -> f64
pub fn potential_energy(&self) -> f64
pub fn total_energy(&self) -> f64
```

It also computes the center of mass:

```rust
pub fn center_of_mass(&self) -> (f64, f64, f64)
```

These diagnostics are not just output decoration. They help evaluate whether
the simulation behaves plausibly. For example, total energy should usually vary
less when the time step is smaller, although the softened model, random initial
conditions, and finite precision all affect the details.

## CSV Output

The program writes structured output using `csv` and `serde::Serialize`.

The evolution output contains one row per time step:

```rust
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
```

The particle-state output contains one row per particle per time step:

```rust
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
```

This distinction matters:

- evolution output is compact and useful for diagnostics;
- state output is larger and useful for animation or detailed inspection.

## Optional Output Files

The output files are optional command-line arguments:

```rust
save_evolution: Option<String>,
save_states: Option<String>,
```

The writers are also optional:

```rust
let mut evolution_writer = args
    .save_evolution
    .as_deref()
    .map(|filename| csv::Writer::from_path(filename).expect("Failed to create evolution file"));
```

The helper function writes only when a writer exists:

```rust
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
```

This is a useful application of `Option`: no output file is not an error. It is
just a valid mode of operation.

## Running And Visualizing

Save diagnostics with:

```bash
cargo run -- --steps 200 --save-evolution evolution.csv
```

Visualize energy and center-of-mass displacement with:

```bash
../visualize-evolution.py evolution.csv
```

Save particle states with:

```bash
cargo run -- --steps 200 --save-states states.csv
```

Animate the particle states with:

```bash
../animate-states.py states.csv --output animation.html
```

The Python scripts use Plotly, and the animation script also uses pandas:

```bash
python3 -m pip install pandas plotly
```

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run the default simulation:

   ```bash
   cd source-code/n-body-simulation/rust
   cargo run
   ```

2. Save evolution diagnostics:

   ```bash
   cargo run -- --steps 200 --save-evolution evolution.csv
   ```

3. Visualize the diagnostics:

   ```bash
   ../visualize-evolution.py evolution.csv
   ```

4. Run the same command with a smaller `--delta-time` and compare total-energy
   variation.

5. Change the seed and compare the energy evolution.

6. Save particle states and create an animation:

   ```bash
   cargo run -- --steps 100 --save-states states.csv
   ../animate-states.py states.csv --output animation.html
   ```

7. Inspect `rust/src/system.rs` and identify which methods only read the system and
   which method mutates it.

8. Change the default softening length on the command line and compare the
   diagnostics.

9. Add one extra column to `EvolutionRecord`, such as `num_particles`, and
   write it to the CSV output.

10. Discuss whether `(f64, f64, f64)` is still a good representation for
    vectors, or whether a named `Vector3` struct would make the code clearer.

## Discussion Points

This module is a good place to emphasize:

- Integrated simulation code combines language features with numerical design
  choices.
- Struct methods are useful for keeping simulation state encapsulated.
- Random initialization should be reproducible through explicit seeds.
- Numerical diagnostics are part of the program design, not an afterthought.
- CSV output makes simulation results easy to inspect with external tools.
- Optional outputs are naturally represented with `Option`.
- Visualization helps reveal behavior that is hard to see from raw numbers.

## Relation To The Julia Set Example

The N-body simulation and Julia set examples are in the same broad category:
both are integrated numerical examples that combine many smaller Rust features.

They emphasize different aspects:

- Julia set: deterministic grid computation, matrix-like output, alternative
  array implementations, and configuration files.
- N-body simulation: time evolution, random initialization, diagnostics,
  structured CSV output, and animation.

Together, they give participants two different views of how Rust can be used
for scientific and technical programs.
