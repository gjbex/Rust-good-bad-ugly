# Generic matrix

This example generalizes the checked matrix type from `../error-handling`.
Compared to `../error-handling`, it shows how to:

* define a generic `struct` with a type parameter, `Matrix<T>`;
* implement methods for all element types with `impl<T>`;
* add a trait bound where an operation needs it, such as `T: Clone`;
* store values of different element types in the same matrix abstraction;
* pass values of type `T` into methods such as `set`;
* return borrowed matrix elements as `Option<&T>` rather than copying values.

Note: this is for demonstration purposes only, for serious applications,
consider using crates like `mdarray`, `faer` or `nalgebra` for matrix
representation and operations.


## What is it?

1. `src/main.rs`: main source file for the application.  It creates a generic
   floating-point matrix and a generic integer matrix.
1. `src/matrix.rs`: module that defines the generic `Matrix<T>` structure and
   its checked access methods.
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
