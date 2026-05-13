# Copy versus move

This example introduces ownership transfer for vector-like data.  Compared to
`../enum-match`, it shows how to:

* pass an owned `Vec<f64>` to a function;
* distinguish cheap scalar copies from moves of owned data structures;
* see that passing a `Vec<f64>` by value moves ownership into the function;
* use `clone` to make an explicit copy of vector data;
* borrow a vector with `&Vec<f64>` and compare that to borrowing a slice;
* use a borrowed slice, `&[f64]`, when a function only needs read-only access;
* mutate borrowed data in place with `&mut [f64]` and `iter_mut`;
* see why collecting references keeps the original vector borrowed;
* use `copied` to collect independent scalar values instead of references;
* receive ownership from a function return value;
* keep the original vector usable after a borrowed function call;
* choose a function signature based on whether the function should own or only
  inspect the data.

The example intentionally names one function `mean_move` to highlight that
taking ownership of the vector is usually the wrong API for a read-only
computation.


## What is it?

1. `src/main.rs`: main source file for the application.  It contrasts copying
   scalar values, moving vectors, cloning vectors, borrowing vectors and slices,
   mutating slices, collecting references, collecting copied values, and
   receiving ownership from a function return value.
1. `Cargo.toml`: configuration file for the Rust package manager.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application with:

```bash
cargo run
```

The source contains commented-out lines that would fail to compile because a
value has been moved or because a vector is still borrowed through collected
references.
