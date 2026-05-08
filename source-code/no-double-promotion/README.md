# No double promotion

A potential performance issue in C/C++/Fortran is the double promotion of
floating-point numbers. This can occur when a single-precision float is
promoted to double precision for an operation, and then the result is promoted
back to single precision. This can lead to unnecessary overhead and reduced
performance.  In Rust, this issue is less common due to the strong type system
and explicit handling of floating-point types. The code illustrates that the
compiler interprets the operations correctly and does not perform unnecessary promotions.


## What is it?

1. `src/main.rs`: this application illustrates that the compiler uses single
   precision constants in the context of single precision operations, and does
   not promote them to double.
1. `Cargo.toml`: the configuration file for the Rust project, specifying
   dependencies and metadata.
1. `Cargo.lock`: the lock file that ensures reproducible builds by locking the
   versions of dependencies.


## How to use?

Run the application with:

```bash
cargo run --release
```
