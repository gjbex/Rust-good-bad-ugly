# Mutable borrowing

By default, function parameters in Rust are immutable, meaning that once a
value is assigned to a variable, it cannot be changed. However, Rust provides a
way to make variables mutable using the `mut` keyword. This allows you to
change the value of a variable after it has been initialized.

However, if the variable passed to the function needs to be modified, you can
use mutable references. A mutable reference allows you to borrow a value
mutably, meaning that you can change the value through the reference. To create
a mutable reference, you can use the `&mut` syntax.

Note that this application is intensionally avoiding a more idiomatic approach
to demonstrate the concept of mutable borrowing in Rust. In a real-world
appliation, the function would return the computed value instead of modifying
the input parameter directly.


## What is it?

1. `src/main.rs`: main source file for the application.
1. `Cargo.toml`: configuration file for the Rust package manager, including the
   dependency on `clap`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application, which specifies the exact versions
   of all dependencies used in the project.


## How to use?

Run the application with the following command:

```bash
cargo run
```
