# Traits

This example extends the generic matrix type from `../generic-structs`.
Compared to `../generic-structs`, it shows how to:

* implement a standard trait from the Rust standard library;
* use `Index` to support immutable indexing with `matrix[(row, col)]`;
* use `IndexMut` to support assignment through indexing;
* use `Display` to define how a type is printed with `{}`;
* use `TryFrom` to define a fallible conversion from nested vectors;
* use `IntoIterator` to define iteration over owned and borrowed matrices;
* use explicit lifetimes in trait implementations for borrowed iterators;
* use an associated type, such as `type Output` or `type Error`, required by a
  trait implementation;
* call trait-provided functionality through ordinary syntax, such as indexing,
  assignment, `println!`, `Matrix::try_from`, and `for value in &matrix`.

The indexing traits intentionally panic for out-of-bounds indices, matching the
behavior of Rust slices.  The previous examples keep fallible `get` and `set`
methods to demonstrate explicit error handling.

Note: this is for demonstration purposes only, for serious applications,
consider using crates like `mdarray`, `faer` or `nalgebra` for matrix
representation and operations.


## What is it?

1. `src/main.rs`: main source file for the application.  It creates matrices
   and uses trait-based indexing, mutation, formatting, conversion, and
   iteration.
1. `src/matrix.rs`: module that defines the generic `Matrix<T>` structure and
   implements `Index`, `IndexMut`, `Display`, `TryFrom`, and `IntoIterator`.
1. `Cargo.toml`: configuration file for the Rust package manager, including the
   dependency on `clap`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application:

```bash
cargo run
```

Choose matrix dimensions:

```bash
cargo run -- --rows 2 --cols 3
```

Run the tests:

```bash
cargo test
```
