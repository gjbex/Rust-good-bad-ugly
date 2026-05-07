# Hello World

This directory is intended to show how to set up a simple "Hello World"
application using `cargo`.


## What is it?

1. `src/main.rs` is the main Rust source file. It contains a simple program that
   prints "Hello world!" to the console.


## How to use?

1. Initialize a new Rust project using `cargo`:

   ```bash
   cargo init
   ```

   Note: this creates a new file `Cargo.toml`.

1. Build the project:

   ```bash
   cargo build
   ```

   Note: this compiles the Rust code and creates an executable in the
   `target/debug` directory.  It also creates a `Cargo.lock` file that locks
   the dependencies of the project.

1. Run the project:

   ```bash
   cargo run
    ```
