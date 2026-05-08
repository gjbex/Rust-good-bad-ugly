# Matrices

This example defines a simple matrix, it shows how to:

* define a structure for a matrix;
* implement a function to create a matrix;
* implement a function to get and set values in the matrix;
* use a crate to define the matrix code;
* use a module from the main file to define the matrix code;
* create a matrix and set values in it.

Note: this is for demonstration purposes only, for serious applications,
consider using crates like `mdarray`, `faer` or `nalgebra` for matrix
representation and operations.


## What is it?

1. `src/main.rs`: main source file for the application.
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
