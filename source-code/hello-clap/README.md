# Hello Clap

This directory shows how to use a crate (`clap`) to handle command-line
arguments in a Rust application.


## What is it?

1. `src/main.rs`: main Rust source file. It will handle several command-line
   arguments using the `clap` crate.
1. `Cargo.toml`: is the manifest file for the Rust project. It specifies the
   dependencies of the project, `clap` version 4.


## How to use?

1. Build the project:

   ```bash
   cargo build
   ```

   Note: this will download the `clap` crate and its dependencies, compile them, and then compile
    the project. The compiled binary will be located in the
   `target/debug` directory.  It also creates a `Cargo.lock` file that locks
   the dependencies of the project.

1. Run the application to get help on the command-line arguments:

   ```bash
   cargo run --  --help
    ```
