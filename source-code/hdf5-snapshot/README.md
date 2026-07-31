# HDF5 Temperature Snapshot

This extended example writes and reads a small scientific dataset with the
high-level [`hdf5-metno`](https://crates.io/crates/hdf5-metno) wrapper around
the native HDF5 library. It is an optional extension to the multidimensional
array material rather than a separate learning module.

## What is it?

The application creates a Gaussian temperature field and stores:

- one-dimensional `x` and `y` coordinate datasets;
- a chunked, gzip-compressed two-dimensional temperature dataset;
- string attributes for descriptions and units;
- scalar attributes for simulation time, step, and the valid temperature
  range; and
- a central 3 x 3 hyperslab read directly from the temperature dataset.

The array-generation and HDF5 I/O code live in `src/lib.rs`; `src/main.rs`
supplies the command-line interface and prints a compact inspection report.
Unit tests verify the generated field, complete array round trip, metadata,
storage layout, compression filter, and hyperslab values.

The example links to a system HDF5 installation. On Debian or Ubuntu, install
the library and command-line inspection tools with:

```bash
sudo apt install libhdf5-dev hdf5-tools
```

## How to use?

Run the example from this directory:

```bash
cargo run --release
```

Choose another output path with:

```bash
cargo run --release -- --output snapshot.h5
```

Because HDF5 is language-independent, the resulting file can be inspected
without Rust:

```bash
h5ls -r temperature-snapshot.h5
h5dump -pH temperature-snapshot.h5
```

Run the round-trip tests with:

```bash
cargo test
```

The first build compiles native-library bindings. Build the example before a
live demonstration and verify that the compute node, container, or CI runner
provides a compatible HDF5 library.
