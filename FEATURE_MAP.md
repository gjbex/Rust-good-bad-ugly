# Rust Feature Map

This map complements the per-example READMEs in `source-code`.  It is organized
by concept rather than by repository order, so larger examples can appear in
several places when they combine multiple ideas.

## Project Structure And Tooling

| Feature | Where to look |
|---|---|
| Minimal Rust binary | `source-code/hello-world/src/main.rs` |
| Cargo project layout | `source-code/hello-world/` |
| Building and running with Cargo | `source-code/hello-world/README.md` |
| External crates in `Cargo.toml` | `source-code/hello-clap/Cargo.toml`, `source-code/complex-numbers/Cargo.toml`, `source-code/units/Cargo.toml`, `source-code/random-numbers/Cargo.toml`, `source-code/smart-pointers/Cargo.toml`, `source-code/strings/Cargo.toml`, `source-code/structural-matching/Cargo.toml`, `source-code/polars-data-analysis/Cargo.toml`, `source-code/heat-diffusion/naive/Cargo.toml`, `source-code/heat-diffusion/ndarray-features/Cargo.toml`, `source-code/heat-diffusion/configurable/Cargo.toml`, `source-code/svd/Cargo.toml`, `source-code/hdf5-snapshot/Cargo.toml`, `source-code/fftw-ffi/Cargo.toml`, `source-code/fftw-safe/Cargo.toml`, `source-code/cpp-interpolation-ffi/Cargo.toml`, `source-code/julia-set/julia-set-toml-config/Cargo.toml`, `source-code/n-body-simulation/rust/Cargo.toml` |
| Reproducible dependency lockfiles | `source-code/*/Cargo.lock`, `source-code/heat-diffusion/*/Cargo.lock`, `source-code/julia-set/*/Cargo.lock`, `source-code/n-body-simulation/rust/Cargo.lock` |
| Multiple binaries in one package | `source-code/hashmap-hashset/src/generate-data.rs`, `source-code/hashmap-hashset/src/count-nucleotides.rs`, `source-code/hashmap-hashset/src/read-errors.rs` |
| Shared package code in `lib.rs` | `source-code/smart-pointers/src/lib.rs`, `source-code/hashmap-hashset/src/lib.rs`, `source-code/hdf5-snapshot/src/lib.rs`, `source-code/polars-data-analysis/src/lib.rs`, `source-code/fftw-ffi/src/lib.rs`, `source-code/cpp-interpolation-ffi/src/lib.rs` |
| Compiling included C++ sources from `build.rs` | `source-code/cpp-interpolation-ffi/build.rs` |
| Release builds and benchmarking context | `source-code/julia-set/benchmark.sh` |
| Unit tests protecting a numerical refactoring | `source-code/heat-diffusion/naive/src/heat_diffusion.rs`, `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Tolerance-based numerical reconstruction test | `source-code/svd/src/main.rs` |
| Black-box comparison of separate Cargo projects | `source-code/heat-diffusion/test_consistency.sh` |

## Command-Line Interfaces

| Feature | Where to look |
|---|---|
| `clap::Parser` derive | `source-code/hello-clap/src/main.rs`, `source-code/units/src/main.rs`, `source-code/smart-pointers/src/main.rs`, `source-code/polars-data-analysis/src/main.rs`, `source-code/heat-diffusion/naive/src/main.rs`, `source-code/heat-diffusion/ndarray-features/src/main.rs`, `source-code/heat-diffusion/configurable/src/main.rs`, `source-code/hdf5-snapshot/src/main.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/cpp-interpolation-ffi/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs` |
| Typed command-line argument struct | `source-code/hello-clap/src/main.rs`, `source-code/units/src/main.rs`, `source-code/smart-pointers/src/main.rs`, `source-code/polars-data-analysis/src/main.rs`, `source-code/heat-diffusion/naive/src/main.rs`, `source-code/heat-diffusion/ndarray-features/src/main.rs`, `source-code/heat-diffusion/configurable/src/main.rs`, `source-code/hdf5-snapshot/src/main.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/cpp-interpolation-ffi/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs` |
| Argument defaults | `source-code/numerical-function/src/main.rs`, `source-code/enum-match/src/main.rs`, `source-code/smart-pointers/src/main.rs`, `source-code/hdf5-snapshot/src/main.rs`, `source-code/cpp-interpolation-ffi/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs` |
| Restricted argument values with `ValueEnum` | `source-code/enum-match/src/main.rs`, `source-code/random-numbers/src/main.rs`, `source-code/user-defined-trait/src/main.rs` |
| File path arguments | `source-code/iterators/src/main.rs`, `source-code/polars-data-analysis/src/main.rs`, `source-code/heat-diffusion/configurable/src/main.rs`, `source-code/hdf5-snapshot/src/main.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs` |
| Numeric parameters for reproducible runs | `source-code/random-numbers/src/main.rs`, `source-code/julia-set/julia-set-baseline/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs` |
| Optional output file arguments | `source-code/hdf5-snapshot/src/main.rs`, `source-code/polars-data-analysis/src/main.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs` |

## Scalar Types And Arithmetic

| Feature | Where to look |
|---|---|
| Primitive scalar types | `source-code/basic-types/src/main.rs` |
| Integer and unsigned integer ranges | `source-code/basic-types/src/main.rs` |
| Floating-point limits and precision constants | `source-code/basic-types/src/main.rs` |
| Mathematical constants for `f32` and `f64` | `source-code/basic-types/src/main.rs` |
| Numeric literals and type conversion | `source-code/numerical-function/src/main.rs`, `source-code/math/src/main.rs` |
| Floating-point literal context and no double promotion | `source-code/no-double-promotion/src/main.rs` |
| Type introspection for examples | `source-code/no-double-promotion/src/main.rs` |
| Integer arithmetic operators | `source-code/math/src/main.rs` |
| Integer division and remainder | `source-code/math/src/main.rs` |
| Euclidean integer division and remainder | `source-code/math/src/main.rs` |
| Floating-point arithmetic operators | `source-code/math/src/main.rs` |
| Mathematical methods on floating-point values | `source-code/math/src/main.rs` |
| Floating-point rounding and absolute-value methods | `source-code/math/src/main.rs` |
| Complex numbers with `num-complex` | `source-code/complex-numbers/src/main.rs` |
| Complex arithmetic and norms | `source-code/complex-numbers/src/main.rs` |
| Physical quantities with `uom` | `source-code/units/src/main.rs` |
| Boundary conversion between `f64` and typed quantities | `source-code/units/src/main.rs` |

## Functions, Control Flow, And Patterns

| Feature | Where to look |
|---|---|
| `main` function | `source-code/hello-world/src/main.rs` |
| Function definitions | `source-code/numerical-function/src/main.rs`, `source-code/control-flow/src/main.rs` |
| Passing functions or closures as arguments | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs`, `source-code/user-defined-trait/src/quadrature.rs` |
| Conditional execution with `if` and `else` | `source-code/control-flow/src/main.rs` |
| `while` loops | `source-code/control-flow/src/main.rs` |
| Simple `for` loops over integer ranges | `source-code/control-flow/src/main.rs`, `source-code/numerical-function/src/main.rs` |
| Inclusive ranges with `..=` | `source-code/control-flow/src/main.rs` |
| Nested `for` loops | `source-code/control-flow/src/main.rs` |
| Defining an `enum` | `source-code/enum-match/src/main.rs` |
| Matching on enum variants | `source-code/enum-match/src/main.rs`, `source-code/random-numbers/src/main.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Match guards | `source-code/enum-match/src/main.rs`, `source-code/hashmap-hashset/src/count-nucleotides.rs` |
| Tuple destructuring in patterns | `source-code/iterators/src/main.rs`, `source-code/traits/src/matrix.rs`, `source-code/n-body-simulation/rust/src/system.rs` |
| Structural `match` on tuple-shaped state | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| Structural `match` on parsed text fields | `source-code/structural-matching/src/main.rs` |
| Optional behavior with `if let` | `source-code/n-body-simulation/rust/src/main.rs` |
| Formatted output | `source-code/numerical-function/src/main.rs`, `source-code/basic-types/src/main.rs`, `source-code/control-flow/src/main.rs` |

## Ownership, Borrowing, And Mutation

| Feature | Where to look |
|---|---|
| Mutable bindings with `mut` | `source-code/mutable-variables/src/main.rs`, `source-code/control-flow/src/main.rs` |
| `Copy` scalar assignment | `source-code/copy-vs-move/src/main.rs` |
| Moving owned vectors by assignment | `source-code/copy-vs-move/src/main.rs` |
| Moving owned vectors into functions | `source-code/copy-vs-move/src/main.rs` |
| Ownership transfer through return values | `source-code/copy-vs-move/src/main.rs` |
| Explicit cloning of vector data | `source-code/copy-vs-move/src/main.rs` |
| Borrowing a vector with `&Vec<T>` | `source-code/copy-vs-move/src/main.rs` |
| Borrowed slices `&[T]` | `source-code/copy-vs-move/src/main.rs`, `source-code/borrowing-vectors/src/main.rs`, `source-code/fftw-ffi/src/real_fft.rs` |
| Mutable references with `&mut` | `source-code/mutable-borrowing/src/main.rs` |
| Writing through a mutable reference | `source-code/mutable-borrowing/src/main.rs` |
| Mutable slices `&mut [T]` | `source-code/copy-vs-move/src/main.rs`, `source-code/borrowing-vectors/src/main.rs` |
| In-place mutation with `iter_mut` | `source-code/copy-vs-move/src/main.rs`, `source-code/borrowing-vectors/src/main.rs` |
| Borrow-checker conflict examples | `source-code/borrowing-vectors/src/main.rs`, `source-code/copy-vs-move/src/main.rs` |
| Collected references that keep the source collection borrowed | `source-code/copy-vs-move/src/main.rs` |
| Recursive ownership with `Box<T>` | `source-code/smart-pointers/src/tree.rs` |
| Moving non-`Copy` values into recursive structures | `source-code/smart-pointers/src/bst.rs` |
| Borrowed read-only multidimensional array view | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Simultaneous immutable input and mutable output views | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Swapping owned buffers with `std::mem::swap` | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |

## Data Modeling With Structs And Modules

| Feature | Where to look |
|---|---|
| Defining a `struct` | `source-code/structs-and-methods/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs`, `source-code/n-body-simulation/rust/src/system.rs` |
| Private fields with public methods | `source-code/structs-and-methods/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs`, `source-code/n-body-simulation/rust/src/system.rs` |
| Associated constructor `new` | `source-code/structs-and-methods/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs`, `source-code/n-body-simulation/rust/src/system.rs` |
| `impl` blocks | `source-code/structs-and-methods/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs`, `source-code/n-body-simulation/rust/src/system.rs` |
| Getter methods | `source-code/structs-and-methods/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs` |
| Mutable methods | `source-code/structs-and-methods/src/matrix.rs`, `source-code/n-body-simulation/rust/src/system.rs` |
| Flat vector storage for 2D data | `source-code/structs-and-methods/src/matrix.rs` |
| Structure-of-arrays storage | `source-code/n-body-simulation/rust/src/system.rs` |
| Crate-private API with `pub(crate)` | `source-code/smart-pointers/src/tree.rs` |
| Separating data structures from algorithms | `source-code/smart-pointers/src/tree.rs`, `source-code/smart-pointers/src/bst.rs` |
| Splitting code into modules with `mod` | `source-code/enum-match/src/main.rs`, `source-code/user-defined-trait/src/main.rs`, `source-code/n-body-simulation/rust/src/main.rs`, `source-code/heat-diffusion/configurable/src/main.rs` |
| Nested structs for configuration groups | `source-code/heat-diffusion/configurable/src/config.rs` |

## Generics, Traits, And Polymorphism

| Feature | Where to look |
|---|---|
| Generic structs with type parameters | `source-code/generic-structs/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs` |
| Generic `impl<T>` blocks | `source-code/generic-structs/src/matrix.rs` |
| Trait bounds on methods | `source-code/generic-structs/src/matrix.rs`, `source-code/smart-pointers/src/bst.rs` |
| Numeric trait bounds with `num-traits` | `source-code/generic-numerics/src/main.rs` |
| Generic running statistics | `source-code/generic-numerics/src/main.rs` |
| Borrowing generic elements as `Option<&T>` | `source-code/generic-structs/src/matrix.rs` |
| Storing different element types through one abstraction | `source-code/generic-structs/src/main.rs` |
| Defining a user-defined trait | `source-code/user-defined-trait/src/quadrature.rs` |
| Implementing a user-defined trait for concrete types | `source-code/user-defined-trait/src/simpson.rs`, `source-code/user-defined-trait/src/gauss.rs` |
| Trait objects with `dyn Trait` | `source-code/user-defined-trait/src/main.rs` |
| Boxed trait objects | `source-code/user-defined-trait/src/main.rs` |
| Enum-as-factory pattern | `source-code/random-numbers/src/main.rs` |
| Wrapping different concrete types in one enum | `source-code/random-numbers/src/main.rs` |
| Implementing `Index` | `source-code/traits/src/matrix.rs` |
| Implementing `IndexMut` | `source-code/traits/src/matrix.rs` |
| Implementing `Display` | `source-code/traits/src/matrix.rs`, `source-code/smart-pointers/src/tree.rs` |
| Implementing `TryFrom` | `source-code/traits/src/matrix.rs` |
| Implementing `IntoIterator` for owned and borrowed values | `source-code/traits/src/matrix.rs` |

## Collections And Iterator Pipelines

| Feature | Where to look |
|---|---|
| `Vec<T>` storage | `source-code/borrowing-vectors/src/main.rs`, `source-code/iterators/src/main.rs` |
| Iterating over slices | `source-code/borrowing-vectors/src/main.rs` |
| `iter` and `iter_mut` | `source-code/borrowing-vectors/src/main.rs` |
| Copying scalar values out of iterators with `copied` | `source-code/copy-vs-move/src/main.rs`, `source-code/iterators/src/main.rs` |
| `filter` | `source-code/iterators/src/main.rs` |
| `map` | `source-code/iterators/src/main.rs` |
| `zip` | `source-code/iterators/src/main.rs` |
| `unzip` | `source-code/iterators/src/main.rs` |
| `enumerate` | `source-code/iterators/src/main.rs` |
| `sum` | `source-code/borrowing-vectors/src/main.rs`, `source-code/iterators/src/main.rs` |
| `collect` with explicit result type | `source-code/iterators/src/main.rs` |
| Owned values versus borrowed values in iterator results | `source-code/iterators/src/main.rs` |
| Counting values with `HashMap` | `source-code/hashmap-hashset/src/count-nucleotides.rs` |
| Updating counts with the entry API | `source-code/hashmap-hashset/src/count-nucleotides.rs` |
| Collecting unique values with `HashSet` | `source-code/hashmap-hashset/src/count-nucleotides.rs` |
| Owned `String` accumulation | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| Borrowed `&str` parser input | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| Splitting and trimming text fields | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| Parsing numeric values from strings | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |

## DataFrames And Columnar Analysis

| Feature | Where to look |
|---|---|
| Lazy CSV scans | `source-code/polars-data-analysis/src/lib.rs` |
| Selecting an input schema | `source-code/polars-data-analysis/src/lib.rs` |
| DataFrame inner join | `source-code/polars-data-analysis/src/lib.rs` |
| Null filtering with expressions | `source-code/polars-data-analysis/src/lib.rs` |
| Derived DataFrame columns | `source-code/polars-data-analysis/src/lib.rs` |
| Grouped aggregations | `source-code/polars-data-analysis/src/lib.rs` |
| Optimized lazy query plan | `source-code/polars-data-analysis/src/main.rs` |
| In-memory DataFrame unit tests | `source-code/polars-data-analysis/src/lib.rs` |

## Multidimensional Arrays, Stencils, And Linear Algebra

| Feature | Where to look |
|---|---|
| Owned two-dimensional arrays with `Array2` | `source-code/heat-diffusion/naive/src/heat_diffusion.rs`, `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs`, `source-code/svd/src/main.rs`, `source-code/hdf5-snapshot/src/lib.rs` |
| Shape inspection with `dim` | `source-code/heat-diffusion/naive/src/heat_diffusion.rs`, `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Two-dimensional element indexing | `source-code/heat-diffusion/naive/src/heat_diffusion.rs` |
| Array slicing with the `s!` macro | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs`, `source-code/svd/src/main.rs`, `source-code/hdf5-snapshot/src/lib.rs` |
| Mutable array views and `fill` | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Conditional fill with `indexed_iter_mut` | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Read-only array views | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs`, `source-code/svd/src/main.rs` |
| Lockstep array traversal with `Zip` | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Shifted views for a five-point stencil | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Double buffering without per-step allocation | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Reduction fused with an array update | `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Broadcasting row and column coordinate arrays | `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Gaussian field initialization | `source-code/heat-diffusion/configurable/src/heat_diffusion.rs`, `source-code/hdf5-snapshot/src/lib.rs` |
| Behavior-preserving array refactoring | `source-code/heat-diffusion/README.md`, `source-code/heat-diffusion/test_consistency.sh` |
| Singular value decomposition with an extension trait | `source-code/svd/src/main.rs` |
| Constructing a diagonal matrix with `Array2::from_diag` | `source-code/svd/src/main.rs` |
| Matrix multiplication with `dot` | `source-code/svd/src/main.rs` |
| Compact SVD dimensions for a rectangular matrix | `source-code/svd/src/main.rs` |
| Maximum absolute and Frobenius reconstruction errors | `source-code/svd/src/main.rs` |
| System OpenBLAS backend selection | `source-code/svd/Cargo.toml`, `source-code/svd/README.md` |
| HDF5 hyperslab read into `Array2` | `source-code/hdf5-snapshot/src/lib.rs` |

## Error Handling

| Feature | Where to look |
|---|---|
| `Option` for possibly absent values | `source-code/error-handling/src/matrix.rs`, `source-code/generic-structs/src/matrix.rs` |
| `Result` for fallible operations | `source-code/error-handling/src/matrix.rs`, `source-code/iterators/src/main.rs`, `source-code/polars-data-analysis/src/lib.rs`, `source-code/svd/src/main.rs`, `source-code/hdf5-snapshot/src/lib.rs`, `source-code/fftw-ffi/src/real_fft.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/cpp-interpolation-ffi/src/lib.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Converting `Option` to `Result` with `ok_or_else` | `source-code/error-handling/src/matrix.rs` |
| Propagating errors with `?` | `source-code/error-handling/src/matrix.rs`, `source-code/iterators/src/main.rs`, `source-code/polars-data-analysis/src/lib.rs`, `source-code/polars-data-analysis/src/main.rs`, `source-code/svd/src/main.rs`, `source-code/hdf5-snapshot/src/lib.rs`, `source-code/hdf5-snapshot/src/main.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/cpp-interpolation-ffi/src/main.rs` |
| Handling errors at the call site | `source-code/error-handling/src/main.rs` |
| Returning boxed dynamic errors from `main` | `source-code/iterators/src/main.rs`, `source-code/polars-data-analysis/src/main.rs`, `source-code/heat-diffusion/configurable/src/main.rs`, `source-code/hdf5-snapshot/src/main.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs`, `source-code/cpp-interpolation-ffi/src/main.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Validating numerical parameters with `Result` | `source-code/heat-diffusion/naive/src/heat_diffusion.rs`, `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/config.rs` |
| Validating application arguments with `Result` | `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| Custom error enum implementing `Display` and `Error` | `source-code/fftw-ffi/src/real_fft.rs`, `source-code/cpp-interpolation-ffi/src/lib.rs` |

## Native Library Interoperability And Unsafe

| Feature | Where to look |
|---|---|
| Raw C bindings through a `-sys` crate | `source-code/fftw-ffi/Cargo.toml`, `source-code/fftw-ffi/src/real_fft.rs` |
| Home-grown C++ library compiled with `cc` | `source-code/cpp-interpolation-ffi/build.rs`, `source-code/cpp-interpolation-ffi/native/` |
| Stable C facade around a C++ class | `source-code/cpp-interpolation-ffi/native/interpolator_c.h`, `source-code/cpp-interpolation-ffi/native/interpolator_c.cpp` |
| Handwritten raw Rust declarations | `source-code/cpp-interpolation-ffi/src/raw.rs` |
| Opaque C handle for a C++ object | `source-code/cpp-interpolation-ffi/native/interpolator_c.h`, `source-code/cpp-interpolation-ffi/src/raw.rs` |
| Pointer-length pairs for numerical arrays | `source-code/cpp-interpolation-ffi/native/interpolator_c.h`, `source-code/cpp-interpolation-ffi/src/lib.rs` |
| Converting C++ exceptions and validation failures to status codes | `source-code/cpp-interpolation-ffi/native/interpolator_c.cpp` |
| System native-library feature selection | `source-code/fftw-ffi/Cargo.toml`, `source-code/fftw-safe/Cargo.toml` |
| High-level safe crate built on a `-sys` crate | `source-code/hdf5-snapshot/Cargo.toml`, `source-code/hdf5-snapshot/src/lib.rs`, `source-code/fftw-safe/Cargo.toml`, `source-code/fftw-safe/src/main.rs` |
| Comparing local and ecosystem safe wrappers | `source-code/fftw-ffi/`, `source-code/fftw-safe/` |
| Safe aligned native buffers with `AlignedVec` | `source-code/fftw-safe/src/main.rs` |
| Owning real-to-complex and complex-to-real plan types | `source-code/fftw-safe/src/main.rs` |
| Checked conversion from `usize` to C `int` | `source-code/fftw-ffi/src/real_fft.rs` |
| Raw pointers isolated behind a safe API | `source-code/fftw-ffi/src/real_fft.rs`, `source-code/cpp-interpolation-ffi/src/lib.rs`, `source-code/cpp-interpolation-ffi/src/raw.rs` |
| Documented `unsafe` blocks | `source-code/fftw-ffi/src/real_fft.rs`, `source-code/cpp-interpolation-ffi/src/lib.rs` |
| Native aligned allocation with `NonNull` | `source-code/fftw-ffi/src/real_fft.rs` |
| Native resource cleanup with `Drop` | `source-code/fftw-ffi/src/real_fft.rs`, `source-code/cpp-interpolation-ffi/src/lib.rs` |
| Field order as a native lifetime invariant | `source-code/fftw-ffi/src/real_fft.rs` |
| Serializing native planner operations with `Mutex` | `source-code/fftw-ffi/src/real_fft.rs` |
| Safe slice-based forward and inverse transforms | `source-code/fftw-ffi/src/real_fft.rs`, `source-code/fftw-safe/src/main.rs` |
| FFI boundary length validation | `source-code/fftw-ffi/src/real_fft.rs` |
| Native-library application without local `unsafe` | `source-code/hdf5-snapshot/src/lib.rs`, `source-code/fftw-safe/src/main.rs` |

## Data I/O And Serialization

| Feature | Where to look |
|---|---|
| Buffered input with `BufReader` | `source-code/hashmap-hashset/src/count-nucleotides.rs`, `source-code/hashmap-hashset/src/read-errors.rs`, `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| Buffered output with `BufWriter` | `source-code/hashmap-hashset/src/generate-data.rs`, `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| Byte iterators over input streams | `source-code/hashmap-hashset/src/count-nucleotides.rs` |
| Line iterators over buffered input | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| CSV input | `source-code/iterators/src/main.rs`, `source-code/iterators/data.txt`, `source-code/polars-data-analysis/data/` |
| Lazy CSV input with Polars | `source-code/polars-data-analysis/src/lib.rs` |
| `serde::Deserialize` derive | `source-code/iterators/src/main.rs`, `source-code/smart-pointers/src/tree.rs`, `source-code/heat-diffusion/configurable/src/config.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Deserializing rows into a struct | `source-code/iterators/src/main.rs` |
| `serde::Serialize` derive | `source-code/smart-pointers/src/tree.rs`, `source-code/n-body-simulation/rust/src/main.rs` |
| JSON serialization and deserialization | `source-code/smart-pointers/src/tree.rs` |
| Serializing recursive data structures | `source-code/smart-pointers/src/tree.rs` |
| Writing CSV records | `source-code/n-body-simulation/rust/src/main.rs` |
| Writing indexed numerical CSV without a serialization crate | `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| Serializing typed records to CSV | `source-code/n-body-simulation/rust/src/main.rs` |
| TOML configuration file for run parameters | `source-code/heat-diffusion/configurable/configs/uniform-spot.toml`, `source-code/heat-diffusion/configurable/configs/gaussian-spot.toml`, `source-code/julia-set/julia-set-toml-config/julia-set.toml` |
| Deserializing a TOML file into typed configuration | `source-code/heat-diffusion/configurable/src/config.rs`, `source-code/julia-set/julia-set-toml-config/src/main.rs` |
| Nested TOML tables for related parameter groups | `source-code/heat-diffusion/configurable/configs/uniform-spot.toml`, `source-code/heat-diffusion/configurable/configs/gaussian-spot.toml` |
| Tagged enum for alternative configuration variants | `source-code/heat-diffusion/configurable/src/config.rs` |
| Rejecting unknown configuration fields | `source-code/heat-diffusion/configurable/src/config.rs` |
| RFC 3339 timestamp parsing with `chrono` | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| Computing elapsed calendar-style spans from parsed timestamps | `source-code/strings/src/main.rs`, `source-code/structural-matching/src/main.rs` |
| HDF5 groups and multidimensional datasets | `source-code/hdf5-snapshot/src/lib.rs` |
| Scalar and string HDF5 attributes | `source-code/hdf5-snapshot/src/lib.rs` |
| HDF5 chunking and gzip compression | `source-code/hdf5-snapshot/src/lib.rs` |
| HDF5 hyperslab selection | `source-code/hdf5-snapshot/src/lib.rs` |
| HDF5 array and metadata round-trip test | `source-code/hdf5-snapshot/src/lib.rs` |
| Writing Parquet output | `source-code/polars-data-analysis/src/main.rs` |

## Randomness And Reproducibility

| Feature | Where to look |
|---|---|
| Seedable random number generator | `source-code/random-numbers/src/main.rs` |
| Reproducible named RNG algorithm with `ChaCha12Rng` | `source-code/random-numbers/src/main.rs` |
| Uniform random values in `[0.0, 1.0)` | `source-code/random-numbers/src/main.rs` |
| Sampling from normal distributions with `rand_distr` | `source-code/random-numbers/src/main.rs` |
| Distribution choice parsed by `clap::ValueEnum` | `source-code/random-numbers/src/main.rs` |
| Runtime distribution enum with a common `sample` method | `source-code/random-numbers/src/main.rs` |
| Generic method over any RNG implementing `Rng` | `source-code/random-numbers/src/main.rs` |
| Seeded random data structure construction | `source-code/smart-pointers/src/main.rs` |
| Random initial conditions for a simulation | `source-code/n-body-simulation/rust/src/system.rs` |

## Scientific And Numerical Examples

| Feature | Where to look |
|---|---|
| Scalar numerical function sampled over an interval | `source-code/numerical-function/src/main.rs` |
| Unit-aware ODE-style parameters | `source-code/units/src/main.rs` |
| Numerical quadrature with multiple algorithms | `source-code/enum-match/`, `source-code/user-defined-trait/` |
| Numerical reference tests | `source-code/enum-match/src/simpson.rs`, `source-code/enum-match/src/gauss.rs`, `source-code/user-defined-trait/src/simpson.rs`, `source-code/user-defined-trait/src/gauss.rs` |
| Julia set as an end-to-end numerical example | `source-code/julia-set/` |
| Custom 2D matrix in a numerical application | `source-code/julia-set/julia-set-baseline/` |
| Dynamic multidimensional arrays with `mdarray` | `source-code/julia-set/julia-set-mdarray/` |
| `mdarray` expression evaluation | `source-code/julia-set/julia-set-mdarray-expr-eval/` |
| Configuration-driven Julia-set runs | `source-code/julia-set/julia-set-toml-config/` |
| Data parallelism with Rayon | `source-code/julia-set/julia-set-rayon/` |
| N-body simulation with velocity Verlet integration | `source-code/n-body-simulation/rust/` |
| Softened gravitational interactions | `source-code/n-body-simulation/rust/src/system.rs` |
| Two-dimensional heat diffusion with a five-point stencil | `source-code/heat-diffusion/` |
| Explicit finite-difference stability validation | `source-code/heat-diffusion/naive/src/heat_diffusion.rs`, `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/config.rs` |
| Symmetry and hand-calculated stencil tests | `source-code/heat-diffusion/naive/src/heat_diffusion.rs`, `source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Configuration-driven heat-diffusion runs | `source-code/heat-diffusion/configurable/` |
| Alternative uniform and Gaussian initial conditions | `source-code/heat-diffusion/configurable/src/config.rs`, `source-code/heat-diffusion/configurable/src/heat_diffusion.rs` |
| Grouped patient-data analysis with Polars | `source-code/polars-data-analysis/` |
| Gaussian temperature snapshot stored in HDF5 | `source-code/hdf5-snapshot/` |
| FFTW real-to-complex transform through FFI | `source-code/fftw-ffi/`, `source-code/fftw-safe/` |
| Spectral-peak and normalized round-trip tests | `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| One-sided mean-square power spectrum | `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| Parseval-style spectral power test | `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| Optional signal and spectrum CSV output | `source-code/fftw-ffi/src/main.rs`, `source-code/fftw-safe/src/main.rs` |
| Piecewise-linear interpolation through a C++ FFI wrapper | `source-code/cpp-interpolation-ffi/` |

## Python Visualization Helpers

| Feature | Where to look |
|---|---|
| Reading numeric output from standard input | `source-code/random-numbers/show-distribution.py`, `source-code/julia-set/view-fractal.py` |
| Pipe Rust program output into Python visualization | `source-code/random-numbers/README.md`, `source-code/julia-set/README.md` |
| Histogram visualization with Plotly | `source-code/random-numbers/show-distribution.py` |
| 2D heatmap visualization with Plotly and `numpy.loadtxt` | `source-code/julia-set/view-fractal.py` |
| CSV diagnostics visualization with Plotly | `source-code/n-body-simulation/visualize-evolution.py` |
| Signal and power-spectrum visualization with Matplotlib | `source-code/fftw-ffi/visualize-signal-and-spectrum.py` |
| Validating expected CSV columns before plotting | `source-code/fftw-ffi/visualize-signal-and-spectrum.py` |
| Static figure output for headless systems | `source-code/fftw-ffi/visualize-signal-and-spectrum.py` |
| Interactive 3D animation with Plotly | `source-code/n-body-simulation/animate-states.py` |

## Gaps To Add Later

These features are not yet covered, or are only lightly touched:

- deeper pattern matching: `while let` and struct destructuring;
- explicit lifetime parameters;
- richer error context beyond compact custom enums;
- integration tests;
- additional standard trait implementations such as `From`, `Default`,
  `Debug`, and `PartialEq`;
- broader serialization workflows beyond compact JSON examples;
- async Rust.
