# Structs and methods

This example implements a small softened gravitational n-body simulation.  It
is intended as a richer example of defining a struct with associated functions
and methods, using methods to keep simulation state encapsulated, and writing
simulation diagnostics to files.

It shows how to:

* define a `System` struct with private fields;
* implement an associated constructor, `System::new`;
* implement methods that inspect and update the system;
* update positions and velocities with a velocity Verlet time step;
* parse simulation parameters with `clap`;
* write CSV output using `csv` and `serde::Serialize`;
* visualize diagnostics and particle states with Python scripts.


## What is it?

1. `src/main.rs`: main source file for the application.  It parses command-line
   arguments, runs the simulation, and optionally writes CSV files for the
   energy evolution and particle states.
1. `src/system.rs`: definition and implementation of the `System` struct.  It
   stores particle positions, velocities, masses, and the gravitational
   softening length, and provides methods for time integration, energies,
   center of mass, and particle-state inspection.
1. `visualize-evolution.py`: Python script that reads the evolution CSV file
   and displays energy and center-of-mass displacement plots side by side.
1. `animate-states.py`: Python script that reads the particle-state CSV file
   and creates an interactive 3D Plotly animation.
1. `Cargo.toml`: configuration file for the Rust package manager.  It specifies
   the dependencies on `clap`, `csv`, `rand`, `rand_distr`, and `serde`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the simulation with the default parameters:

```bash
cargo run
```

Save energy and center-of-mass evolution:

```bash
cargo run -- --save-evolution evolution.csv
```

Save particle states as CSV, one row per particle per time step:

```bash
cargo run -- --save-states states.csv
```

Save both outputs:

```bash
cargo run -- --steps 200 --save-evolution evolution.csv --save-states states.csv
```

Visualize the energy evolution:

```bash
./visualize-evolution.py evolution.csv
```

Animate the particle states:

```bash
./animate-states.py states.csv --output animation.html
```

The Python scripts require additional Python packages:

```bash
python3 -m pip install matplotlib pandas plotly
```
