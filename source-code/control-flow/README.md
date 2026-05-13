# Control flow

This example introduces basic control flow.  Compared to
`../numerical-function`, it shows how to:

* use `if` and `else` to choose between two branches;
* use a `while` loop that continues until a condition changes;
* update mutable function parameters inside a loop;
* use inclusive integer ranges with `..=`;
* use nested `for` loops over integer ranges.

The example computes greatest common divisors with a subtraction-based Euclidean
algorithm.


## What is it?

1. `src/main.rs`: main source file for the application.  It defines a `gcd`
   function using `while`, `if`, and `else`, then prints a small table of
   greatest common divisors using nested `for` loops.
1. `Cargo.toml`: configuration file for the Rust package manager.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application with:

```bash
cargo run
```
