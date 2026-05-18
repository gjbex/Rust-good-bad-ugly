# Rust Feature Map

This map complements the per-example READMEs in `source-code`.  It lists Rust
features and points to the directories or source files that illustrate them.
The examples are ordered roughly by the training sequence in
`source-code/README.md`.

## Tooling And Project Structure

| Feature | Where to look |
|---|---|
| Minimal Rust binary | `source-code/hello-world/src/main.rs` |
| Cargo project layout | `source-code/hello-world/` |
| Building and running with Cargo | `source-code/hello-world/README.md` |
| External crates in `Cargo.toml` | `source-code/hello-clap/Cargo.toml`, `source-code/complex-numbers/Cargo.toml`, `source-code/random-numbers/Cargo.toml`, `source-code/julia-set/julia-set-toml-config/Cargo.toml`, `source-code/n-body-simulation/Cargo.toml` |
| Reproducible dependency lockfiles | `source-code/*/Cargo.lock` |
| Release builds and benchmarking context | `source-code/julia-set/benchmark.sh` |

## Command-Line Interfaces

| Feature | Where to look |
|---|---|
| `clap::Parser` derive | `source-code/hello-clap/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs`, `source-code/n-body-simulation/src/main.rs` |
| Typed command-line argument struct | `source-code/hello-clap/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs`, `source-code/n-body-simulation/src/main.rs` |
| Argument defaults | `source-code/numerical-function/src/main.rs`, `source-code/enum-match/src/main.rs`, `source-code/n-body-simulation/src/main.rs` |
| Restricted argument values with `ValueEnum` | `source-code/enum-match/src/main.rs`, `source-code/random-numbers/src/main.rs`, `source-code/user-defined-trait/src/main.rs` |
| File path argument | `source-code/iterators/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Numeric CLI parameters for reproducible runs | `source-code/random-numbers/src/main.rs`, `source-code/julia-set/julia-set-baseline/src/main.rs` |
| Optional output file arguments | `source-code/n-body-simulation/src/main.rs` |

## Basic Language Constructs

| Feature | Where to look |
|---|---|
| `main` function | `source-code/hello-world/src/main.rs` |
| Function definitions | `source-code/numerical-function/src/main.rs`, `source-code/control-flow/src/main.rs` |
| Numeric literals and type conversion | `source-code/numerical-function/src/main.rs`, `source-code/math/src/main.rs` |
| Integer arithmetic operators | `source-code/math/src/main.rs` |
| Integer division and remainder | `source-code/math/src/main.rs` |
| Euclidean integer division and remainder | `source-code/math/src/main.rs` |
| Floating-point arithmetic operators | `source-code/math/src/main.rs` |
| Mathematical methods on floating-point values | `source-code/math/src/main.rs` |
| Floating-point rounding and absolute-value methods | `source-code/math/src/main.rs` |
| Conditional execution with `if` and `else` | `source-code/control-flow/src/main.rs` |
| `while` loops | `source-code/control-flow/src/main.rs` |
| Simple `for` loops over integer ranges | `source-code/control-flow/src/main.rs`, `source-code/numerical-function/src/main.rs` |
| Inclusive ranges with `..=` | `source-code/control-flow/src/main.rs` |
| Nested `for` loops | `source-code/control-flow/src/main.rs` |
| Formatted output | `source-code/numerical-function/src/main.rs`, `source-code/basic-types/src/main.rs`, `source-code/control-flow/src/main.rs` |
| Mutable bindings with `mut` | `source-code/mutable-variables/src/main.rs`, `source-code/control-flow/src/main.rs` |
| Primitive scalar types | `source-code/basic-types/src/main.rs` |
| Floating-point limits and precision constants | `source-code/basic-types/src/main.rs` |
| Mathematical constants for `f32` and `f64` | `source-code/basic-types/src/main.rs` |
| Type introspection for examples | `source-code/no-double-promotion/src/main.rs` |
| Floating-point literal context and no double promotion | `source-code/no-double-promotion/src/main.rs` |

## Scientific Scalar Types

| Feature | Where to look |
|---|---|
| Complex numbers with `num-complex` | `source-code/complex-numbers/src/main.rs` |
| Importing external types with `use` | `source-code/complex-numbers/src/main.rs` |
| Complex arithmetic and norms | `source-code/complex-numbers/src/main.rs` |

## Enums, Matching, Modules, Docs, And Tests

| Feature | Where to look |
|---|---|
| Defining an `enum` | `source-code/enum-match/src/main.rs` |
| Matching on enum variants | `source-code/enum-match/src/main.rs`, `source-code/random-numbers/src/main.rs` |
| Splitting code into modules with `mod` | `source-code/enum-match/src/main.rs`, `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs`, `source-code/user-defined-trait/src/main.rs` |
| Passing functions or closures as arguments | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs`, `source-code/user-defined-trait/src/quadrature.rs` |
| Enum-as-factory pattern | `source-code/random-numbers/src/main.rs` |
| Wrapping different concrete types in one enum | `source-code/random-numbers/src/main.rs` |
| Documentation comments | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |
| Unit tests with `#[cfg(test)]` | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |
| Numerical reference tests | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |

## Ownership, Borrowing, And Slices

| Feature | Where to look |
|---|---|
| `Copy` scalar assignment | `source-code/copy-vs-move/src/main.rs` |
| Moving owned vectors by assignment | `source-code/copy-vs-move/src/main.rs` |
| Moving owned vectors into functions | `source-code/copy-vs-move/src/main.rs` |
| Explicit cloning of vector data | `source-code/copy-vs-move/src/main.rs` |
| Borrowing a vector with `&Vec<T>` | `source-code/copy-vs-move/src/main.rs` |
| Borrowed slices `&[T]` | `source-code/copy-vs-move/src/main.rs`, `source-code/borrowing-vectors/src/main.rs` |
| Mutable references with `&mut` | `source-code/mutable-borrowing/src/main.rs` |
| Writing through a mutable reference | `source-code/mutable-borrowing/src/main.rs` |
| Mutable slices `&mut [T]` | `source-code/copy-vs-move/src/main.rs`, `source-code/borrowing-vectors/src/main.rs` |
| In-place mutation with `iter_mut` | `source-code/copy-vs-move/src/main.rs`, `source-code/borrowing-vectors/src/main.rs` |
| Iterating over slices | `source-code/borrowing-vectors/src/main.rs` |
| Borrow-checker conflict examples | `source-code/borrowing-vectors/src/main.rs`, `source-code/copy-vs-move/src/main.rs` |
| Collected references that keep the source collection borrowed | `source-code/copy-vs-move/src/main.rs` |
| Copying scalar values out of iterators with `copied` | `source-code/copy-vs-move/src/main.rs`, `source-code/iterators/src/main.rs` |
| Ownership transfer through return values | `source-code/copy-vs-move/src/main.rs` |

## Structs And Methods

| Feature | Where to look |
|---|---|
| Defining a `struct` | `source-code/structs-and-methods/src/matrix.rs` |
| Private fields with public methods | `source-code/structs-and-methods/src/matrix.rs` |
| Associated constructor `new` | `source-code/structs-and-methods/src/matrix.rs` |
| `impl` blocks | `source-code/structs-and-methods/src/matrix.rs` |
| Getter methods | `source-code/structs-and-methods/src/matrix.rs` |
| Mutable methods | `source-code/structs-and-methods/src/matrix.rs` |
| Flat vector storage for 2D data | `source-code/structs-and-methods/src/matrix.rs` |
| Struct encapsulating simulation state | `source-code/n-body-simulation/src/system.rs` |
| Associated constructor for a simulation state | `source-code/n-body-simulation/src/system.rs` |
| Methods for updating and inspecting simulation state | `source-code/n-body-simulation/src/system.rs` |
| Structure-of-arrays storage | `source-code/n-body-simulation/src/system.rs` |

## Error Handling

| Feature | Where to look |
|---|---|
| `Option` for possibly absent values | `source-code/error-handling/src/matrix.rs` |
| `Result` for fallible operations | `source-code/error-handling/src/matrix.rs`, `source-code/iterators/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Converting `Option` to `Result` with `ok_or_else` | `source-code/error-handling/src/matrix.rs` |
| Propagating errors with `?` | `source-code/error-handling/src/matrix.rs`, `source-code/iterators/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Handling errors at the call site | `source-code/error-handling/src/main.rs` |
| Returning boxed dynamic errors from `main` | `source-code/iterators/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |

## Generics And Trait Bounds

| Feature | Where to look |
|---|---|
| Defining a user-defined trait | `source-code/user-defined-trait/src/quadrature.rs` |
| Implementing a user-defined trait for concrete types | `source-code/user-defined-trait/src/simpson.rs`, `source-code/user-defined-trait/src/gauss.rs` |
| Trait objects with `dyn Trait` | `source-code/user-defined-trait/src/main.rs` |
| Boxed trait objects | `source-code/user-defined-trait/src/main.rs` |
| Generic structs with type parameters | `source-code/generic-structs/src/matrix.rs` |
| Generic `impl<T>` blocks | `source-code/generic-structs/src/matrix.rs` |
| Trait bounds on methods | `source-code/generic-structs/src/matrix.rs` |
| Borrowing generic elements as `Option<&T>` | `source-code/generic-structs/src/matrix.rs` |
| Storing different element types through one abstraction | `source-code/generic-structs/src/main.rs` |

## Iterators And Collections

| Feature | Where to look |
|---|---|
| `Vec<T>` storage | `source-code/borrowing-vectors/src/main.rs`, `source-code/iterators/src/main.rs` |
| `iter` and `iter_mut` | `source-code/borrowing-vectors/src/main.rs` |
| `filter` | `source-code/iterators/src/main.rs` |
| `map` | `source-code/iterators/src/main.rs` |
| `zip` | `source-code/iterators/src/main.rs` |
| `unzip` | `source-code/iterators/src/main.rs` |
| `enumerate` | `source-code/iterators/src/main.rs` |
| `sum` | `source-code/borrowing-vectors/src/main.rs`, `source-code/iterators/src/main.rs` |
| `collect` with explicit result type | `source-code/iterators/src/main.rs` |
| Owned values versus borrowed values in iterator results | `source-code/iterators/src/main.rs` |
| Optional writers with `Option` and `if let` | `source-code/n-body-simulation/src/main.rs` |

## Data I/O And Serialization

| Feature | Where to look |
|---|---|
| CSV input | `source-code/iterators/src/main.rs`, `source-code/iterators/data.txt` |
| `serde::Deserialize` derive | `source-code/iterators/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| `serde::Serialize` derive | `source-code/n-body-simulation/src/main.rs` |
| Deserializing rows into a struct | `source-code/iterators/src/main.rs` |
| Writing CSV records | `source-code/n-body-simulation/src/main.rs` |
| Serializing typed records to CSV | `source-code/n-body-simulation/src/main.rs` |
| Reading an input path from the CLI | `source-code/iterators/src/main.rs` |
| TOML configuration file for run parameters | `source-code/julia-set/julia-set-toml-config/src/main.rs`, `source-code/julia-set/julia-set-toml-config/julia-set.toml` |
| Deserializing a TOML file into a typed config struct | `source-code/julia-set/julia-set-toml-config/src/main.rs` |

## Random Numbers And Distributions

| Feature | Where to look |
|---|---|
| Seedable random number generator | `source-code/random-numbers/src/main.rs` |
| Reproducible named RNG algorithm with `ChaCha12Rng` | `source-code/random-numbers/src/main.rs` |
| Uniform random values in `[0.0, 1.0)` | `source-code/random-numbers/src/main.rs` |
| Sampling from normal distributions with `rand_distr` | `source-code/random-numbers/src/main.rs` |
| Distribution choice parsed by `clap::ValueEnum` | `source-code/random-numbers/src/main.rs` |
| Runtime distribution enum with a common `sample` method | `source-code/random-numbers/src/main.rs` |
| Generic method over any RNG implementing `Rng` | `source-code/random-numbers/src/main.rs` |
| Random initial conditions for a simulation | `source-code/n-body-simulation/src/system.rs` |

## Python Visualization Helpers

| Feature | Where to look |
|---|---|
| Reading numeric output from standard input | `source-code/random-numbers/show-distribution.py`, `source-code/julia-set/view-fractal.py` |
| Pipe Rust program output into Python visualization | `source-code/random-numbers/README.md`, `source-code/julia-set/README.md` |
| Histogram visualization with `matplotlib` | `source-code/random-numbers/show-distribution.py` |
| 2D image visualization with `matplotlib` and `numpy.loadtxt` | `source-code/julia-set/view-fractal.py` |
| CSV diagnostics visualization with `matplotlib` | `source-code/n-body-simulation/visualize-evolution.py` |
| Interactive 3D animation with Plotly | `source-code/n-body-simulation/animate-states.py` |

## Larger Numerical Examples

| Feature | Where to look |
|---|---|
| Julia set as an end-to-end numerical example | `source-code/julia-set/` |
| Custom 2D matrix in a numerical application | `source-code/julia-set/julia-set-baseline/` |
| Configuration-driven Julia-set runs | `source-code/julia-set/julia-set-toml-config/` |
| Dynamic multidimensional arrays with `mdarray` | `source-code/julia-set/julia-set-mdarray/` |
| `mdarray` expression evaluation | `source-code/julia-set/julia-set-mdarray-expr-eval/` |
| N-body simulation with velocity Verlet integration | `source-code/n-body-simulation/` |
| Softened gravitational interactions | `source-code/n-body-simulation/src/system.rs` |
| Release-build comparison with `hyperfine` | `source-code/julia-set/benchmark.sh` |
| Plotting output with Python | `source-code/julia-set/view-fractal.py`, `source-code/random-numbers/show-distribution.py` |

## Gaps To Add Later

These features are not yet covered, or are only lightly touched:

- deeper pattern matching: `while let`, tuple or struct destructuring;
- explicit lifetime parameters;
- custom error enums and richer error context;
- `lib.rs`, integration tests, and reusable library-plus-binary layout;
- standard trait implementations such as `Display`, `Index`, and `From`;
- hash collections such as `HashMap` and `HashSet`;
- JSON data examples and broader serialization workflows;
- data parallelism with Rayon;
- FFI, `unsafe`, async Rust, and smart pointers.
