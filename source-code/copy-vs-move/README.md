# Copy versus move

This example introduces ownership transfer for vector-like data.  Compared to
`../enum-match`, it shows how to:

* pass an owned `Vec<f64>` to a function;
* see that passing a `Vec<f64>` by value moves ownership into the function;
* use a borrowed slice, `&[f64]`, when a function only needs read-only access;
* keep the original vector usable after a borrowed function call;
* choose a function signature based on whether the function should own or only
  inspect the data.

The example intentionally names one function `mean_wrong` to highlight that
taking ownership of the vector is the wrong API for a read-only computation.


## What is it?

1. `src/main.rs`: main source file for the application.  It computes the mean
   of one vector by moving it into a function, then computes the mean of another
   vector by borrowing it as a slice.
1. `Cargo.toml`: configuration file for the Rust package manager.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application with:

```bash
cargo run
```

The source contains a commented-out line that would fail to compile because the
first vector has been moved into `mean_wrong`.
