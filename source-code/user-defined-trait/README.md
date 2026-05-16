# User-defined trait

This example computes the numerical integral of `sin(x)` on the interval
`[0, pi]`.  It is based on `../enum-match`, but replaces the direct `match`
between free functions with a user-defined trait implemented by each
quadrature rule.

Compared to `../enum-match`, it shows how to:

* define a trait for shared behavior;
* implement that trait for different concrete types;
* use a trait object, `Box<dyn QuadratureRule>`, when the concrete type is
  selected at run time;
* keep `clap::ValueEnum` for command-line selection;
* pass a closure to a trait method;
* add unit tests for each trait implementation.

The exact integral is 2, so the tests compare the implemented quadrature rules
against that known reference value.


## What is it?

1. `src/main.rs`: main source file for the application.  It parses the
   quadrature method from the command line, selects a boxed trait object, and
   prints the approximate integral.
1. `src/quadrature.rs`: definition of the `QuadratureRule` trait shared by all
   quadrature implementations.
1. `src/simpson.rs`: `Simpson` type and its implementation of
   `QuadratureRule`, including a unit test.
1. `src/gauss.rs`: `GaussLegendre10` type and its implementation of
   `QuadratureRule`, including a unit test.
1. `Cargo.toml`: configuration file for the Rust package manager.  It
   specifies the dependency on `clap`.
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

Choose a different number of Simpson subdivisions:

```bash
cargo run -- --method simpson --subdivisions 2000
```

Show the accepted command-line values:

```bash
cargo run -- --help
```

Run the tests:

```bash
cargo test
```
