# Structs and methods

This example introduces a small user-defined matrix type.  Compared to
`../borrowing-vectors`, it shows how to:

* define a `struct`;
* keep fields private and expose accessor methods;
* implement methods with an `impl` block;
* store two-dimensional data in a flat `Vec<f64>`.

Note: this is for demonstration purposes only, for serious applications,
consider using crates like `mdarray`, `faer` or `nalgebra` for matrix
representation and operations.


## What is it?

1. `src/main.rs`: main source file for the application.  It creates a matrix,
   fills it with values, and prints it.
1. `src/matrix.rs`: module that defines the `Matrix` structure and its methods.
1. `Cargo.toml`: configuration file for the Rust package manager, including the
   dependency on `clap`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application, which specifies the exact versions
   of all dependencies used in the project.


## How to use?

Run the application with the following command to see the help message:

```bash
cargo run --  --help
```
