# Random numbers

This example generates reproducible random numbers.  Compared to
`../iterators`, it shows how to:

* use a seedable random number generator;
* choose a named RNG algorithm, `ChaCha12Rng`, for reproducible streams;
* generate values from a uniform distribution;
* sample from non-uniform distributions with the `rand_distr` crate;
* use a `ValueEnum` for the command-line distribution choice;
* separate the CLI choice from the runtime distribution object with two enums;
* write a generic method over any RNG that implements `Rng`.

The uniform distribution samples values in the half-open interval `[0.0, 1.0)`.


## What is it?

1. `src/main.rs`: main source file for the application.  It parses the number
   of samples, seed, and distribution from the command line, constructs a
   reproducible RNG, and prints one sample per line.
1. `show-distribution.py`: small Python helper script that reads values from
   standard input and displays an interactive Plotly histogram.
1. `Cargo.toml`: configuration file for the Rust package manager.  It specifies
   the dependencies on `clap`, `rand`, `rand_chacha`, and `rand_distr`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Generate five uniform samples with the default seed:

```bash
cargo run -- --count 5 --distribution uniform
```

Generate five normally distributed samples with an explicit seed:

```bash
cargo run -- --count 5 --seed 42 --distribution normal
```

Show the accepted command-line values:

```bash
cargo run -- --help
```

Visualize a larger sample:

```bash
cargo run -- --count 1000 --seed 42 --distribution normal | ./show-distribution.py
```
