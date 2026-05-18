# Mutable borrowing

This example introduces mutable borrowing.  Compared to `../no-double-promotion`,
it shows how to:

* pass a mutable reference to a function with `&mut`;
* receive a mutable reference as a function parameter;
* write through a mutable reference with dereferencing, e.g., `*dxdt = -x`;
* make mutation visible at the call site;
* use a mutable reference to simulate a C++ non-const reference parameter.

Note that this application is intentionally avoiding a more idiomatic approach
to demonstrate the concept of mutable borrowing in Rust. In a real-world
application, the function would return the computed value instead of modifying
the input parameter directly.


## What is it?

1. `src/main.rs`: main source file for the application.  It computes a simple
   time integration and writes the derivative through a mutable reference.
1. `Cargo.toml`: configuration file for the Rust package manager.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application, which specifies the exact versions
   of all dependencies used in the project.


## How to use?

Run the application with the following command:

```bash
cargo run
```
