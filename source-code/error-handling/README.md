# Error handling

This example extends the matrix example with explicit error handling for
out-of-bounds indexing.  Compared to `../structs-and-methods`, it shows how to:

* represent a value that may be absent with `Option`;
* represent an operation that may fail with `Result`;
* convert an `Option` into a `Result` with `ok_or_else`;
* use closures such as `|index| ...` and `|| ...`;
* transform the value inside an `Option` with `map`;
* return early from a failing operation with the `?` operator;
* handle errors at the call site with `expect`.

Note: this is for demonstration purposes only, for serious applications,
consider using crates like `mdarray`, `faer` or `nalgebra` for matrix
representation and operations.


## What is it?

1. `src/main.rs`: main source file for the application.  It creates a matrix,
   fills it with values, and handles the checked `get` and `set` operations.
1. `src/matrix.rs`: module that defines the `Matrix` structure and its checked
   access methods.
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
