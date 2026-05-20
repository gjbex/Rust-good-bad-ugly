# Getting Started With Rust Projects

This module introduces the basic structure of a Rust project and the everyday
commands used to build, run, inspect, and extend small Rust programs. It is
intended as the first hands-on module before moving on to Rust's type system,
ownership model, control flow, structs, traits, and numerical examples.

The emphasis is practical: by the end of the module, participants should be
comfortable opening an existing Rust project, recognizing its main files, using
Cargo, adding a dependency, and making a small change with confidence.

## Learning Objectives

After completing this module, participants should be able to:

- Explain the role of `cargo` in a Rust project.
- Recognize the purpose of `Cargo.toml`, `Cargo.lock`, and `src/main.rs`.
- Build and run a Rust binary project.
- Read compiler diagnostics well enough to fix simple mistakes.
- Add a crate dependency and use it from the program.
- Distinguish between a minimal Rust program and a small command-line tool.
- Locate the relevant examples in this repository for further practice.

## Prerequisites

Participants should have:

- A working shell.
- Rust installed through `rustup` or the system package manager.
- Basic familiarity with editing source files.
- Basic command-line experience.

Check the Rust installation with:

```bash
rustc --version
cargo --version
```

If both commands report a version, the environment is ready for this module.

## Project Structure

A minimal Rust binary project has the following shape:

```text
project-name/
├── Cargo.toml
└── src/
    └── main.rs
```

The important files are:

- `Cargo.toml`: project metadata and dependency declarations.
- `Cargo.lock`: exact dependency versions selected by Cargo.
- `src/main.rs`: the entry point for a binary program.

For the examples in this repository, each directory under `source-code/` is a
separate Cargo project. That means commands such as `cargo run` and
`cargo check` should normally be run from inside the example directory.

## Starting Your Own Project

Cargo can create the standard project structure for you.

To create a new binary project in a new directory, use:

```bash
cargo new my-project
cd my-project
cargo run
```

This creates a directory containing `Cargo.toml` and `src/main.rs`, initializes
a Git repository by default, and gives you a small program that can be built
and run immediately.

If you already have an existing directory and want to turn it into a Cargo
project, use `cargo init` from inside that directory:

```bash
mkdir my-existing-project
cd my-existing-project
cargo init
cargo run
```

The practical difference is:

- `cargo new` creates a new project directory.
- `cargo init` turns the current directory into a Cargo project.

For the examples in this repository, the projects have already been created, so
the usual workflow is to enter one of the existing directories and run Cargo
commands there. For new experiments outside the repository, `cargo new` is
usually the simplest starting point.

## First Example: Hello World

Start with the minimal example:

```bash
cd source-code/hello-world
cargo run
```

The important parts of `src/main.rs` are:

```rust
fn main() {
    println!("Hello, world!");
}
```

This illustrates three basic ideas:

- `main` is the function where a binary program starts.
- `println!` writes formatted text to standard output.
- The `!` marks `println!` as a macro, not an ordinary function.

Useful follow-up commands are:

```bash
cargo check
cargo build
cargo run
```

`cargo check` is often the fastest command while developing because it checks
whether the code compiles without producing a final executable.

## Reading Compiler Diagnostics

Rust compiler messages are an important part of the development workflow. They
usually identify:

- The file and line where the compiler found a problem.
- The relevant expression or declaration.
- The reason the code is invalid.
- A suggested fix, when the compiler can infer one.

For example, removing the semicolon from a statement or misspelling a function
name will produce a diagnostic with a source location. Participants should get
used to reading the first error carefully before changing several things at
once.

Suggested exercise:

1. Introduce a small syntax error in `source-code/hello-world/src/main.rs`.
2. Run `cargo check`.
3. Read the diagnostic.
4. Fix the error.
5. Run `cargo check` again.

## Adding Command-Line Arguments

The next step is a small command-line program. This repository uses `clap` for
examples that parse command-line options.

See:

```text
source-code/hello-clap
```

Run it with:

```bash
cd source-code/hello-clap
cargo run -- --help
```

The `--` separates arguments passed to Cargo from arguments passed to the
program being run.

For example:

```bash
cargo run -- --name Rust
```

The main project-level change is that `Cargo.toml` contains a dependency:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

The corresponding Rust code derives a parser from a struct definition. This is
an early example of an important Rust pattern: use a type to describe the shape
of valid input, then let a library handle the repetitive parsing work.

## Cargo Commands To Know

The most useful Cargo commands at this stage are:

```bash
cargo check
cargo build
cargo run
cargo test
cargo fmt
cargo doc --open
```

Their roles are:

- `cargo check`: type-check and compile-check the project quickly.
- `cargo build`: build the executable.
- `cargo run`: build and run the executable.
- `cargo test`: run tests.
- `cargo fmt`: format Rust source code.
- `cargo doc --open`: build and open local API documentation.

For small examples, `cargo run` is often enough. For larger edits,
`cargo check` gives a faster edit-check cycle.

## Dependencies And Crates

Rust packages are called crates. A project can depend on crates from
`crates.io` or from other locations.

In this training repository, examples use crates such as:

- `clap` for command-line argument parsing.
- `rand` for random number generation.
- `serde`, `csv`, and `toml` for structured data.
- `num-complex` for complex numbers.
- `ndarray` for multidimensional arrays.

Dependencies are declared in `Cargo.toml`. Cargo resolves versions and records
the exact selected versions in `Cargo.lock`.

Dependencies can be added by editing `Cargo.toml` directly, but `cargo add` is
often more convenient because it updates the manifest using Cargo's normal
dependency syntax. For example, a command-line example that uses `clap` with
derive macros can add the dependency with:

```bash
cargo add clap --features derive
```

This adds the corresponding entry to `Cargo.toml` and updates `Cargo.lock`.
Using `cargo add` is especially useful while learning because it avoids small
formatting mistakes in dependency declarations and makes the dependency change
explicit in the shell history.

For applications and training examples, `Cargo.lock` should normally be kept in
version control because it makes builds more reproducible.

## Suggested Hands-On Work

Use this sequence as a short practical lab.

1. Run the minimal program.

   ```bash
   cd source-code/hello-world
   cargo run
   ```

2. Change the printed message and run it again.

3. Introduce a small error, run `cargo check`, and interpret the diagnostic.

4. Open `source-code/hello-clap` and run:

   ```bash
   cargo run -- --help
   ```

5. Run the same program with one or more command-line options.

6. Inspect `Cargo.toml` and identify the dependency list.

7. Run `cargo fmt` and `cargo check`.

## Discussion Points

This module is a good place to discuss a few habits that carry through the rest
of the training:

- Use `cargo check` frequently.
- Treat compiler diagnostics as part of the workflow, not as a last resort.
- Keep examples as small Cargo projects so they are easy to build and run.
- Prefer established crates for common tasks such as command-line parsing and
  file formats.
- Make configuration and inputs explicit when moving from toy examples to
  numerical programs.

## Connection To Later Modules

The project basics in this module are used throughout the rest of the material:

- Numeric examples build on the same `main.rs` structure.
- Control-flow examples remain small binary crates.
- Struct and trait examples add more files under `src/`.
- Julia set examples introduce more realistic dependencies and I/O.
- The N-body simulation combines command-line parsing, structured output,
  modules, methods, and visualization scripts.

Once participants are comfortable with this module, they are ready to move on
to scalar types, arithmetic, functions, and control flow.
