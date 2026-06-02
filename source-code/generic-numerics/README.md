# Generic Numerics

This example defines a small running-statistics type that accepts several
numeric input types and accumulates the result in one floating-point type.
Compared to `../generic-structs`, it shows how to:

* define a generic struct whose type parameter is constrained by numeric
  traits;
* use the `num-traits` crate for generic numeric conversions;
* keep running state in a struct as values are added;
* accept input values through a generic method;
* return `Option` when a numeric conversion or statistic may be unavailable;
* compute a mean and population standard deviation from accumulated sums.

The example is intentionally small. It demonstrates why scientific code often
needs trait bounds from crates such as `num-traits` instead of relying only on
Rust's built-in numeric casts.


## What is it?

1. `src/main.rs`: main source file for the application. It defines `Stats<T>`,
   adds values of different numeric input types, and prints the count, mean,
   and population standard deviation.
1. `Cargo.toml`: configuration file for the Rust package manager. It specifies
   the dependency on `num-traits`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application:

```bash
cargo run
```
