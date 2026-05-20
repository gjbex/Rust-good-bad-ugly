# Learning Module Structure

This document proposes a learning path for the Rust training material in this
repository. The structure follows the existing examples in `source-code/`, but
groups them into teaching modules rather than treating the directory order as
the curriculum itself.

The intended audience is learners who know how to program, but are new to Rust
or new to using Rust for scientific and technical computing.

## Module 1: Getting Started With Rust Projects

Primary examples:

- `source-code/hello-world`
- `source-code/hello-clap`

Topics:

- Rust project layout.
- `Cargo.toml`, `Cargo.lock`, and `src/main.rs`.
- `cargo check`, `cargo build`, and `cargo run`.
- Adding dependencies with `cargo add`.
- Basic command-line parsing with `clap`.
- Reading compiler diagnostics.

Goal:

Participants should be able to open, build, run, and lightly modify a small
Rust binary project.

Related module text:

- `docs/learning-modules/getting-started-with-rust-projects.md`

## Module 2: Scalar Computation And Numeric Basics

Primary examples:

- `source-code/basic-types`
- `source-code/math`
- `source-code/numerical-function`
- `source-code/no-double-promotion`
- `source-code/complex-numbers`

Topics:

- Integer and floating-point types.
- Type inference and explicit type annotations.
- Numeric literals.
- Arithmetic operators.
- Integer division and remainder.
- Floating-point functions and constants.
- Complex numbers through `num-complex`.
- Differences from C and C++ numeric promotion rules.

Goal:

Participants should understand how Rust represents scalar values and how to
write small numerical expressions without relying on implicit conversions.

Related module text:

- `docs/learning-modules/scalar-computation-and-numeric-basics.md`

## Module 3: Control Flow And Program Structure

Primary examples:

- `source-code/control-flow`
- `source-code/numerical-function`
- `source-code/enum-match`

Topics:

- `if` and `else`.
- `while` loops.
- `for` loops over ranges and collections.
- Function definitions.
- Return values.
- Enums.
- `match`.
- Introductory source modules.

Goal:

Participants should be able to write small programs with explicit control flow
and factor repeated work into functions.

Related module text:

- `docs/learning-modules/control-flow-and-program-structure.md`

## Module 4: Ownership, Borrowing, And Mutation

Primary examples:

- `source-code/mutable-variables`
- `source-code/copy-vs-move`
- `source-code/borrowing-vectors`
- `source-code/mutable-borrowing`

Topics:

- Immutable bindings by default.
- Mutable bindings.
- Move semantics.
- Copy types.
- Shared references.
- Mutable references.
- Borrowing slices instead of whole owned containers.

Goal:

Participants should develop a working mental model for ownership and borrowing
before moving on to larger data structures.

Related module text:

- `docs/learning-modules/ownership-borrowing-and-mutation.md`

Optional contrast:

- `source-code/cpp-pitfalls`

This material can be used to show the kinds of memory-safety and concurrency
issues Rust's ownership model is designed to prevent or make explicit.

## Module 5: Data Modeling With Structs And Methods

Primary examples:

- `source-code/structs-and-methods`
- `source-code/generic-structs`

Topics:

- Defining structs.
- Implementing methods with `impl`.
- Associated functions.
- Encapsulation through methods.
- Generic structs.
- Trait bounds on implementations.

Goal:

Participants should be able to define small domain types and attach behavior to
those types using methods.

Related module text:

- `docs/learning-modules/data-modeling-with-structs-and-methods.md`

## Module 6: Reusable Abstractions With Traits

Primary examples:

- `source-code/traits`
- `source-code/user-defined-trait`
- `source-code/generic-structs`

Topics:

- Standard trait implementations.
- `Display`, `Index`, `IndexMut`, and `TryFrom`.
- User-defined traits.
- Trait bounds.
- Trait objects.
- `dyn Trait`.
- Static and dynamic dispatch at a conceptual level.

Goal:

Participants should understand how Rust expresses shared behavior without
classical inheritance.

Related module text:

- `docs/learning-modules/reusable-abstractions-with-traits.md`

## Module 7: Collections, Iterators, And Text Data

Primary examples:

- `source-code/iterators`
- `source-code/hashmap-hashset`

Topics:

- Vectors.
- Iterator adapters.
- `map`, `filter`, `fold`, and `scan`.
- Hash maps and hash sets.
- Reading and writing text files.
- Buffered I/O.

Goal:

Participants should be able to process collections and simple text data using
idiomatic iterator-based Rust.

Related module text:

- `docs/learning-modules/collections-iterators-and-text-data.md`

## Module 8: Error Handling

Primary example:

- `source-code/error-handling`

Topics:

- `Option`.
- `Result`.
- The `?` operator.
- Propagating errors from functions.
- Converting from simple examples to fallible command-line programs.

Goal:

Participants should be able to recognize and write Rust code that handles
missing values and recoverable errors explicitly.

Related module text:

- `docs/learning-modules/error-handling.md`

## Module 9: Project Organization, Libraries, And Tests

Primary examples:

- `source-code/hashmap-hashset`
- `source-code/enum-match`
- `source-code/traits`

Topics:

- Shared library code in `src/lib.rs`.
- Multiple executables in one Cargo package.
- `[[bin]]` entries in `Cargo.toml`.
- Reusing library code from several binaries.
- Unit tests.
- `#[cfg(test)]` test modules.
- Numerical checks with tolerances.

Goal:

Participants should understand how a Cargo package can grow beyond a single
`main.rs`, and how to keep shared code and tests organized as examples become
larger.

Related module text:

- `docs/learning-modules/project-organization-libraries-and-tests.md`

## Module 10: Randomness And Reproducible Runs

Primary example:

- `source-code/random-numbers`

Topics:

- Random number generators.
- Distributions.
- Seeding.
- Reproducibility.
- Producing data for visualization.

Goal:

Participants should understand how to generate random data in a controlled way
and why explicit seeds matter for scientific examples.

Related module text:

- `docs/learning-modules/randomness-and-reproducible-runs.md`

## Module 11: Data Parallelism With Rayon

Primary example:

- `source-code/julia-set/julia-set-rayon`

Topics:

- Data parallelism.
- Rayon parallel iterators.
- `into_par_iter`.
- Parallel `map` and `collect`.
- Avoiding shared mutable state.
- Controlling worker threads with `RAYON_NUM_THREADS`.
- Benchmarking serial and parallel implementations.

Goal:

Participants should understand how to use Rayon for independent per-element
work, and how to reason about when data parallelism is likely to help.

Related module text:

- `docs/learning-modules/data-parallelism-with-rayon.md`

## Module 12: Integrated Numerical Example: Julia Set

Primary example group:

- `source-code/julia-set`

Topics:

- Complex arithmetic.
- Arrays and matrices.
- Command-line configuration.
- TOML configuration files.
- Image-like numerical output.
- Multiple implementations of the same algorithm.
- Comparing implementation styles.

Goal:

Participants should see how the earlier language features combine in a compact
scientific-computing example.

Related module text:

- `docs/learning-modules/integrated-numerical-example-julia-set.md`

Suggested placement:

This module works well after the core language modules, alongside the other
integrated numerical examples.

## Module 13: Integrated Numerical Example: N-Body Simulation

Primary example:

- `source-code/n-body-simulation/rust`

Topics:

- Structs and methods in a larger example.
- Random initialization.
- Command-line parameters.
- CSV output.
- Time integration.
- Energy diagnostics.
- Python visualization helpers.
- Separating simulation state, output, and analysis.

Goal:

Participants should see a small but realistic scientific program that combines
many of the earlier concepts in one place.

Related module text:

- `docs/learning-modules/integrated-numerical-example-n-body-simulation.md`

Suggested placement:

This module should be treated as an integrated numerical example, similar in
role to the Julia set module, rather than as part of the initial feature-by-feature
sequence.

## Optional Module: Rust By Contrast With C++

Primary example:

- `source-code/cpp-pitfalls`

Topics:

- Memory safety.
- Dangling references.
- Data races.
- Numeric conversions.
- The difference between preventing errors and documenting discipline.

Goal:

Participants with a C or C++ background should get a concrete sense of which
problems Rust is designed to move from run time to compile time.

This module can be used near the beginning for motivation or later as a
reflective comparison after ownership and borrowing have been introduced.

## Suggested Teaching Order

A compact course can use this order:

1. Getting Started With Rust Projects.
2. Scalar Computation And Numeric Basics.
3. Control Flow And Program Structure.
4. Ownership, Borrowing, And Mutation.
5. Data Modeling With Structs And Methods.
6. Reusable Abstractions With Traits.
7. Collections, Iterators, And Text Data.
8. Error Handling.
9. Project Organization, Libraries, And Tests.
10. Randomness And Reproducible Runs.
11. Data Parallelism With Rayon.
12. Integrated Numerical Example: Julia Set.
13. Integrated Numerical Example: N-Body Simulation.

For a shorter course, the Julia set example can be used as the main integrated
example and the N-body simulation can be left as an additional integrated
example.

For a course aimed at scientific programmers, the numerical, randomness, Julia
set, and N-body modules should receive more time than the purely syntactic
examples.
