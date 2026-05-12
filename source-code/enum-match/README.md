# Enum match

This example computes the numerical integral of `sin(x)` on the interval
`[0, pi]`.  Compared to `../no-double-promotion`, it shows how to:

* define an enum to represent a small set of choices;
* use `match` to select behavior based on an enum variant;
* restrict command-line argument values with `clap::ValueEnum`;
* pass a function or closure as an argument to another function;
* split code across multiple source files with `mod`;
* document functions and command-line arguments;
* add unit tests for numerical code.

The exact integral is 2, so the tests compare the implemented quadrature rules
against that known reference value.


## What is it?

1. `src/main.rs`: main source file for the application.  It parses the
   quadrature method from the command line, matches on the selected enum
   variant, and prints the approximate integral.
1. `src/simpson.rs`: implementation of composite Simpson quadrature, including
   a unit test for the integral of `sin(x)` from 0 to `pi`.
1. `src/gauss.rs`: implementation of a fixed 10-point Gauss-Legendre
   quadrature rule, including a unit test for the same integral.
1. `Cargo.toml`: configuration file for the Rust package manager.  It specifies
   the dependency on `clap`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application with the default Simpson method:

```bash
cargo run
```

Select the Gaussian quadrature method:

```bash
cargo run -- --method gauss
```

Show the accepted command-line values:

```bash
cargo run -- --help
```

Run the tests:

```bash
cargo test
```
