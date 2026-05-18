# Julia set

This directory contains several Rust applications that compute the Julia set.


## What is it?

1. `julia-set-baseline`: baseline implementation of the Julia set, using a
   single thread, and a custom matrix implementation.
1. `julia-set-mdarray`: implementation of the Julia set, using a single thread,
   and the `mdarray` crate for matrix operations.
1. `julia-set-mdarray-expr-eval`: implementation of the Julia set, using a
   single thread, the `mdarray` crate for matrix operations, and expression
   evaluation for matrix operations.
1. `julia-set-toml-config`: implementation of the Julia set, using a single
   thread, the custom matrix implementation from the baseline, and a TOML
   configuration file for run parameters.
1. `view-fractal.py`: Python script to visualize the output of the Julia set
   applications using Plotly.  It reads from standard input by default, so
   output from the Rust applications can be piped directly into it.


## How to use?

Run one of the implementations and pipe its output to the visualization script:

```bash
cd julia-set-baseline
cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

The same approach works for the `mdarray` implementation:

```bash
cd julia-set-mdarray
cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

The TOML configuration implementation takes a configuration file name instead
of individual run parameters:

```bash
cd julia-set-toml-config
cargo run --release -- julia-set.toml | ../view-fractal.py
```

The visualization script can still read from a file:

```bash
./view-fractal.py julia-set-baseline/julia-set.txt
```

Compare all implementations with release builds using `hyperfine`:

```bash
./benchmark.sh
```
