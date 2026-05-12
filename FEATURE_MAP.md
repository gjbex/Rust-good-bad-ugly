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
| External crates in `Cargo.toml` | `source-code/hello-clap/Cargo.toml`, `source-code/complex-numbers/Cargo.toml` |
| Reproducible dependency lockfiles | `source-code/*/Cargo.lock` |
| Release builds and benchmarking context | `source-code/julia-set/benchmark.sh` |

## Command-Line Interfaces

| Feature | Where to look |
|---|---|
| `clap::Parser` derive | `source-code/hello-clap/src/main.rs` |
| Typed command-line argument struct | `source-code/hello-clap/src/main.rs` |
| Argument defaults | `source-code/numerical-function/src/main.rs`, `source-code/enum-match/src/main.rs` |
| Restricted argument values with `ValueEnum` | `source-code/enum-match/src/main.rs` |
| File path argument | `source-code/iterators/src/main.rs` |

## Basic Language Constructs

| Feature | Where to look |
|---|---|
| `main` function | `source-code/hello-world/src/main.rs` |
| Function definitions | `source-code/numerical-function/src/main.rs` |
| Numeric literals and type conversion | `source-code/numerical-function/src/main.rs` |
| Loops over integer ranges | `source-code/numerical-function/src/main.rs` |
| Formatted output | `source-code/numerical-function/src/main.rs`, `source-code/basic-types/src/main.rs` |
| Mutable bindings with `mut` | `source-code/mutable-variables/src/main.rs` |
| Primitive scalar types | `source-code/basic-types/src/main.rs` |
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
| Matching on enum variants | `source-code/enum-match/src/main.rs` |
| Splitting code into modules with `mod` | `source-code/enum-match/src/main.rs`, `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |
| Passing functions or closures as arguments | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |
| Documentation comments | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |
| Unit tests with `#[cfg(test)]` | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |
| Numerical reference tests | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs` |

## Ownership, Borrowing, And Slices

| Feature | Where to look |
|---|---|
| Mutable references with `&mut` | `source-code/mutable-borrowing/src/main.rs` |
| Writing through a mutable reference | `source-code/mutable-borrowing/src/main.rs` |
| Borrowed slices `&[T]` | `source-code/borrowing-vectors/src/main.rs` |
| Mutable slices `&mut [T]` | `source-code/borrowing-vectors/src/main.rs` |
| Iterating over slices | `source-code/borrowing-vectors/src/main.rs` |
| Borrow-checker conflict examples | `source-code/borrowing-vectors/src/main.rs` |
| Copying scalar values out of iterators with `copied` | `source-code/iterators/src/main.rs` |

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

## Error Handling

| Feature | Where to look |
|---|---|
| `Option` for possibly absent values | `source-code/error-handling/src/matrix.rs` |
| `Result` for fallible operations | `source-code/error-handling/src/matrix.rs`, `source-code/iterators/src/main.rs` |
| Converting `Option` to `Result` with `ok_or_else` | `source-code/error-handling/src/matrix.rs` |
| Propagating errors with `?` | `source-code/error-handling/src/matrix.rs`, `source-code/iterators/src/main.rs` |
| Handling errors at the call site | `source-code/error-handling/src/main.rs` |
| Returning boxed dynamic errors from `main` | `source-code/iterators/src/main.rs` |

## Generics And Trait Bounds

| Feature | Where to look |
|---|---|
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

## Data I/O And Serialization

| Feature | Where to look |
|---|---|
| CSV input | `source-code/iterators/src/main.rs`, `source-code/iterators/data.txt` |
| `serde::Deserialize` derive | `source-code/iterators/src/main.rs` |
| Deserializing rows into a struct | `source-code/iterators/src/main.rs` |
| Reading an input path from the CLI | `source-code/iterators/src/main.rs` |

## Larger Numerical Examples

| Feature | Where to look |
|---|---|
| Julia set as an end-to-end numerical example | `source-code/julia-set/` |
| Custom 2D matrix in a numerical application | `source-code/julia-set/julia-set-baseline/` |
| Dynamic multidimensional arrays with `mdarray` | `source-code/julia-set/julia-set-mdarray/` |
| `mdarray` expression evaluation | `source-code/julia-set/julia-set-mdarray-expr-eval/` |
| Release-build comparison with `hyperfine` | `source-code/julia-set/benchmark.sh` |
| Plotting output with Python | `source-code/julia-set/view-plot.py` |

## Gaps To Add Later

These features are not yet covered, or are only lightly touched:

- moves and ownership transfer beyond simple borrowing;
- deeper pattern matching: `if let`, `while let`, tuple or struct destructuring;
- defining and implementing custom traits;
- explicit lifetime parameters;
- custom error enums and richer error context;
- `lib.rs`, integration tests, and reusable library-plus-binary layout;
- standard trait implementations such as `Display`, `Index`, and `From`;
- hash collections such as `HashMap` and `HashSet`;
- JSON/TOML configuration and buffered file I/O;
- data parallelism with Rayon;
- FFI, `unsafe`, async Rust, and smart pointers.
