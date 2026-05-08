# Mutable variables

By default, variables in Rust are immutable, meaning that once a value is
assigned to a variable, it cannot be changed. However, Rust provides a way to
make variables mutable using the `mut` keyword. This allows you to change the
value of a variable after it has been initialized.


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
