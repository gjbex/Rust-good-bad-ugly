# Rust false friends by programming background

This note highlights common stumbling blocks for researchers learning Rust
after using C, C++, Fortran, Python, R, or MATLAB. The goal is not to compare
languages in general, but to point out places where familiar habits can lead to
incorrect, unidiomatic, or unnecessarily complicated Rust.

The examples assume ordinary, stable Rust: prefer clear ownership, borrowed
slices, standard collection types, `Result` and `Option`, iterators, tests, and
Cargo-based project structure before reaching for advanced features.

## General Rust habits that help everyone

- Rust distinguishes between owned values, shared references, mutable
  references, and lifetimes. Many mistakes come from treating these as
  interchangeable.
- Bindings are immutable by default. Use `mut` when the binding or borrowed
  value really needs to change.
- Assignment usually moves non-`Copy` values. If a value should remain usable,
  borrow it, clone it deliberately, or redesign the data flow.
- Prefer borrowed function parameters when ownership is not needed: `&[T]`
  instead of `Vec<T>`, `&str` instead of `String`, and `&T` instead of `T`.
- Use `Result<T, E>` for recoverable errors and `Option<T>` for values that may
  be absent. Avoid using sentinel values when the type system can represent the
  case directly.
- Rust uses zero-based indexing and half-open ranges, such as `0..n`.
- Integer division truncates: `1 / 2` is `0`, while `1.0 / 2.0` is `0.5`.
- Rust does not perform many implicit numeric conversions. Convert explicitly
  when changing numeric type.
- Compiler errors are part of the workflow. Read the first relevant error in
  your own code, and treat the borrow checker as feedback about data ownership.
- Prefer `cargo check`, small examples, and tests for fast iteration.
- Reach for `unsafe` only when a safe abstraction cannot express the operation
  and the invariants are documented locally.

## From C

C programmers often appreciate Rust's explicitness and low-level control. The
false friend is assuming that Rust is C with stricter syntax.

### False friends

- **Pointers as the default model**: Rust has raw pointers, but ordinary code
  should use owned values, references, slices, `Box<T>`, `Rc<T>`, `Arc<T>`, or
  collection types depending on ownership.
- **Manual allocation and cleanup**: Rust releases resources automatically when
  values go out of scope. Prefer ownership and RAII-style cleanup over explicit
  free-like logic.
- **Null pointers**: Safe Rust references cannot be null. Use `Option<&T>`,
  `Option<Box<T>>`, or another explicit optional type when absence is valid.
- **Aliasing and mutation**: C permits many aliasing patterns that are unsafe or
  hard to reason about. Rust enforces either many shared references or one
  mutable reference at a time.
- **Raw arrays as normal containers**: Use `Vec<T>` for growable owned data,
  `[T; N]` for fixed-size arrays, and `&[T]` or `&mut [T]` for borrowed views.
- **Integer and pointer casts**: Rust casts with `as` are explicit but still
  require care. Do not translate C cast-heavy code mechanically.
- **Macros for constants and small utilities**: Prefer `const`, `static`,
  functions, generic functions, and traits before writing macros.
- **Unchecked error codes**: Rust code normally makes error handling visible in
  the type with `Result`, and the `?` operator propagates errors explicitly.
- **Assuming uninitialized memory is routine**: Safe Rust requires initialized
  values. Low-level uninitialized memory APIs exist, but they belong in small,
  carefully reviewed abstractions.

### Practical advice

Start with safe Rust and let the type system express ownership, optionality, and
errors. Only drop to raw pointers or `unsafe` after the safe shape of the data
structure is clear.

## From C++

C++ programmers are often comfortable with value semantics, RAII, generics, and
performance-oriented design. The main surprise is that Rust makes ownership and
aliasing rules part of the type system rather than relying on conventions.

### False friends

- **Move semantics as an optimization detail**: In C++, a moved-from object
  remains valid but unspecified. In Rust, moving a non-`Copy` value normally
  makes the original binding unusable.
- **References as just aliases**: Rust references are borrowed access with
  lifetime and aliasing rules. A `&mut T` is exclusive access, not merely a
  non-const reference.
- **`const` habits**: Rust's default binding immutability is not the same as
  C++ `const`. Mutability is controlled by bindings and references, and interior
  mutability uses types such as `Cell`, `RefCell`, `Mutex`, or `RwLock`.
- **RAII alone is enough**: Rust has deterministic destruction through `Drop`,
  but memory safety also depends on ownership, borrowing, and lifetime checks.
- **Class hierarchies and virtual methods**: Rust has structs, enums, traits,
  generics, and trait objects. Prefer composition, enums, and trait bounds
  before trying to reproduce inheritance.
- **Templates map directly to generics**: Rust generics use trait bounds and
  monomorphization, but specialization, overloading, and template metaprogramming
  do not translate directly.
- **Function overloading**: Rust does not support ordinary function overloading
  by argument type. Use different names, traits, generic functions, or builder
  patterns.
- **Exceptions**: Rust does not use exceptions for recoverable errors. Use
  `Result` and propagate with `?`; reserve panics for bugs or unrecoverable
  conditions.
- **Operator overloading as a broad design tool**: Rust supports operator traits
  such as `Add` and `Index`, but fewer operators and conversions are overloadable
  than in C++.
- **Headers and separate declarations**: Rust modules do not use header files.
  Visibility is expressed with `pub`, and Cargo manages crates and dependencies.
- **Smart pointers as direct equivalents**: `Box<T>`, `Rc<T>`, `Arc<T>`,
  `RefCell<T>`, `Mutex<T>`, and `RwLock<T>` encode different ownership and
  borrowing policies. Pick the one that matches the sharing and threading
  semantics, not the one with the most familiar name.
- **Assuming zero-cost means no constraints**: Rust can generate efficient code,
  but clone operations, bounds checks, allocation, dynamic dispatch, and
  synchronization still matter.

### Practical advice

Think less in terms of "what is the Rust version of this C++ feature?" and more
in terms of who owns each value, who may read or mutate it, and how errors and
variants are represented in the type system.

## From Fortran

Fortran programmers often bring strong numerical-programming habits. The main
surprises in Rust are explicit ownership, library-based array support, and the
absence of built-in whole-array syntax.

### False friends

- **Array indexing**: Fortran arrays are commonly one-based; Rust slices,
  vectors, arrays, and ranges are zero-based.
- **Memory layout**: Fortran arrays are column-major. Rust's standard `Vec<T>`
  is one-dimensional contiguous storage; multidimensional layout depends on the
  library or indexing scheme you choose.
- **Array slicing**: Rust slices such as `&data[start..end]` are views into
  contiguous one-dimensional data. Multidimensional slicing requires a library
  or explicit indexing design.
- **Whole-array operations**: Rust has no built-in array language. Use loops,
  iterators, or numerical crates when expressing vector and matrix operations.
- **Pass-by-reference assumptions**: Rust requires explicit borrowing with
  `&T` or `&mut T`, and the compiler enforces whether mutation is allowed.
- **Implicit shape information**: A borrowed slice knows its length, but raw
  numeric kernels still need explicit shape, stride, and layout conventions.
- **Module expectations**: Rust modules organize names and visibility, but they
  are not Fortran modules. Cargo packages code into crates.
- **Assuming mature numerical coverage everywhere**: Rust's scientific
  ecosystem is useful but uneven compared with long-established Fortran, C, C++,
  Python, or MATLAB ecosystems.

### Practical advice

Be explicit about shape, layout, and ownership from the start. For scientific
code, choose the array and linear algebra crates deliberately, and document
layout conventions near the data structure or kernel.

## From Python

Python programmers often expect dynamic behavior, automatic memory management,
and quick feedback. Rust asks for more up-front precision, but the compiler then
checks many mistakes before the program runs.

### False friends

- **Names versus values**: Python variables are names bound to objects. Rust
  bindings own, borrow, or copy values with specific types and lifetimes.
- **Assignment as rebinding**: In Python, assignment usually rebinds a name. In
  Rust, assignment to an existing mutable binding changes the stored value, and
  assigning a non-`Copy` value elsewhere may move it.
- **Automatic garbage collection**: Rust normally has deterministic ownership
  instead of a tracing garbage collector. Reference-counted types exist but are
  explicit.
- **Duck typing**: Rust generic code uses traits and compile-time checks. If a
  function needs a capability, express it as a trait bound.
- **Lists versus vectors**: Python lists are growable arrays of references to
  arbitrary objects. `Vec<T>` stores homogeneous values contiguously.
- **Strings**: Rust distinguishes owned `String`, borrowed `&str`, bytes, and
  Unicode scalar values. Indexing a string by integer is not supported because
  UTF-8 characters are variable width.
- **Exceptions**: Rust uses `Result` for recoverable errors. The `?` operator
  can make this concise, but the possibility of failure remains visible.
- **Implicit truthiness**: Rust conditionals require `bool`. Values such as
  integers, strings, vectors, and options are not automatically truthy or falsy.
- **Interactive workflow**: Rust is compiled. Use `cargo check`, focused tests,
  and small examples instead of expecting notebook-like edit-and-run behavior.

### Practical advice

Treat Rust code as a precise description of data representation and allowed
operations. When porting Python, first decide which values are owned data,
borrowed views, optional results, and recoverable errors.

## From R

R programmers often come from a vectorized, data-analysis-oriented environment.
Rust is more explicit about types, sizes, missing values, and control flow.

### False friends

- **Vectorization as the default**: Rust standard-library code usually uses
  loops, iterators, or crate-provided array operations rather than implicit
  whole-vector arithmetic.
- **One-based indexing**: R is one-based; Rust is zero-based.
- **Recycling rules**: Rust does not recycle shorter vectors. Size mismatches
  must be checked or represented explicitly.
- **Missing values**: Rust has no universal `NA`. Use `Option<T>`, masks,
  sentinel values with clear documentation, or library-specific missing-data
  support.
- **Copy-on-modify intuition**: Rust makes moves, clones, borrows, and mutable
  borrows explicit. A `clone()` should be a deliberate choice.
- **Data frames**: Rust has data-frame crates, but no data frame in the standard
  library. Decide whether the task is tabular analysis, a numerical kernel, or a
  reusable command-line tool.
- **Type coercion**: Rust avoids many implicit coercions. Convert numeric and
  string types explicitly.
- **Formula interfaces**: Statistical formula syntax does not have a direct
  language-level equivalent. Models are usually expressed through functions,
  structs, enums, traits, or crate APIs.

### Practical advice

Make data representation part of the design. For research pipelines, Rust is
often strongest as a reliable parser, simulator, numerical kernel, or
reproducible command-line tool that can interoperate with R for analysis and
visualization.

## From MATLAB

MATLAB programmers often expect matrix-first syntax and interactive numerical
workflows. Rust can support numerical work, but most matrix behavior comes from
crates rather than the core language.

### False friends

- **One-based indexing**: MATLAB is one-based; Rust is zero-based.
- **Matrix as the default data type**: Rust has scalars, arrays, slices,
  vectors, structs, enums, and crate-provided matrix types. There is no built-in
  general matrix type.
- **Column-major layout**: MATLAB arrays are column-major. Rust layout depends
  on the chosen storage representation or numerical crate.
- **Element-wise versus matrix operations**: MATLAB distinguishes `*` from
  `.*`. In Rust, operator meaning depends on the implemented traits for the
  chosen type or crate.
- **Broadcasting**: Rust has no standard broadcasting rules. Numerical crates
  may provide them, but the semantics are library-specific.
- **Growing arrays in loops**: Repeated growth can cause reallocations. Use
  known sizes, `Vec::with_capacity`, `push`, or crate-specific constructors.
- **Scripts and shared workspace state**: Rust programs should express inputs,
  outputs, and dependencies through functions, structs, configuration, and files.
- **Plotting and visualization**: Rust has plotting crates, but many scientific
  workflows still compute in Rust and visualize with Python, R, MATLAB, or
  external tools.

### Practical advice

Translate the mathematical intent, not the MATLAB syntax. Decide which crate or
data representation owns the arrays, and keep indexing and layout conventions
visible in the code.

## Cross-language traps in scientific Rust

### Numeric types and precision

- Be explicit about `f32`, `f64`, signed integer types, unsigned integer types,
  and literals.
- Rust does not promote `f32` to `f64` automatically in mixed expressions.
- `usize` is useful for indexing but is not a universal integer type for
  numerical formulas.
- Integer overflow checks depend on build mode: debug builds check overflow for
  ordinary integer arithmetic, while optimized builds wrap unless you use
  checked, wrapping, saturating, or overflowing operations explicitly.
- Floating-point equality is usually the wrong test for computed values; use
  tolerances appropriate to the problem.

### Ownership, borrowing, and performance

- Passing `Vec<T>` by value transfers ownership. Use `&[T]` or `&mut [T]` when
  a function only needs to read or mutate existing data.
- Returning owned values is normal and often efficient. Avoid premature
  lifetime-heavy APIs when ownership would be clearer.
- `clone()` can be useful, but it is a real operation. Use it deliberately and
  look for avoidable large clones in performance-sensitive paths.
- Bounds checks protect indexing operations. Iterators, slices, and careful
  structure often express the same computation without scattered indexing.
- Views and references must not outlive the data they refer to. The compiler
  catches many mistakes, but API design still matters.

### Errors and absence

- Use `Option<T>` when a value may be absent.
- Use `Result<T, E>` when an operation may fail.
- Avoid encoding missing values as magic numbers unless interoperability or file
  format constraints require it.
- `panic!` is usually for bugs, violated invariants, or unrecoverable situations,
  not ordinary input errors.

### Libraries and interoperability

- Rust scientific projects often depend on crates for arrays, linear algebra,
  random numbers, command-line parsing, serialization, and parallelism. Learn
  the crate's ownership and layout conventions early.
- Interoperability with C, C++, Fortran, Python, R, or MATLAB can be valuable,
  but data layout, ownership, error handling, and build systems must be handled
  carefully.
- For HPC code, understand whether the project uses threads, Rayon, MPI, CUDA,
  HIP, SYCL, Kokkos, C/Fortran libraries, or another layer before choosing the
  Rust abstraction.

## Summary table

| Background | Most likely false friend | Rust habit to develop |
| --- | --- | --- |
| C | Treating Rust as safer C syntax | Use ownership, references, slices, `Option`, and `Result` before raw pointers |
| C++ | Translating classes, references, exceptions, and templates directly | Design around ownership, borrowing, traits, enums, and explicit errors |
| Fortran | Assuming familiar array layout and whole-array semantics | Make shape, layout, indexing, and crate choices explicit |
| Python | Expecting dynamic names and automatic runtime behavior | Use static types, ownership, borrowing, traits, and `Result` as design tools |
| R | Expecting vectorized data-frame semantics | Represent sizes, missing values, and data structures explicitly |
| MATLAB | Expecting matrix-first built-in syntax | Choose numerical crates and translate mathematical intent |

## Teaching tip

When participants ask "what is the Rust equivalent of this feature?", first
identify whether they mean syntax, semantics, performance behavior, or workflow.
The best Rust answer is often not a direct translation, but a design that makes
ownership, borrowing, errors, variants, and data layout explicit.
