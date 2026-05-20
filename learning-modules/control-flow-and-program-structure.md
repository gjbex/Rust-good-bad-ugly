# Control Flow And Program Structure

This module introduces the basic building blocks used to organize Rust
programs: branches, loops, functions, enums, pattern matching, source modules,
and reusable computations. It builds on the scalar values and numeric expressions from
`learning-modules/scalar-computation-and-numeric-basics.md`.

The goal is to move from straight-line programs to programs that make choices,
repeat work, name reusable computations, and select behavior from a small set
of well-defined alternatives.

## Learning Objectives

After completing this module, participants should be able to:

- Use `if` and `else` to choose between branches.
- Use `while` for condition-controlled loops.
- Use `for` loops over integer ranges.
- Use inclusive ranges with `..=`.
- Define functions with typed parameters and return values.
- Use mutable function parameters when an algorithm updates local state.
- Create, access, and destructure small tuples.
- Define an enum to represent a fixed set of choices.
- Use `match` to select behavior based on an enum variant.
- Split code across multiple source files with `mod`.

## Prerequisites

Participants should already be comfortable with:

- Running Cargo examples.
- Reading and editing `src/main.rs`.
- Basic scalar types such as `i32`, `usize`, and `f64`.
- Integer ranges and simple arithmetic expressions.

The examples used in this module are:

- `source-code/control-flow`
- `source-code/numerical-function`
- `source-code/enum-match`

## Branches With `if` And `else`

The `control-flow` example computes greatest common divisors using a
subtraction-based Euclidean algorithm:

```bash
cd source-code/control-flow
cargo run
```

The key function is:

```rust
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
```

The `if` expression chooses which value to update:

- If `a > b`, subtract `b` from `a`.
- Otherwise, subtract `a` from `b`.

In Rust, the condition must be a `bool`. Integers are not accepted as
conditions. This is different from C and C++, where zero and nonzero integer
values are often used as false and true.

## Loops With `while`

A `while` loop repeats as long as its condition is true:

```rust
while a != b {
    // update a or b
}
```

The `gcd` function continues until the two values are equal. At that point, the
common value is the greatest common divisor and the function returns it.

The parameters are written as `mut` because the algorithm updates local copies
of `a` and `b`:

```rust
fn gcd(mut a: i32, mut b: i32) -> i32
```

The `mut` applies to the local bindings inside the function. It does not mean
that the caller's variables are changed.

## Loops With `for` And Ranges

Rust's `for` loop iterates over values produced by an iterator. A common early
example is a half-open integer range:

```rust
for i in 0..n {
    println!("{i}");
}
```

The range `0..n` includes `0` and stops before `n`. This is the most common
form when looping over indices or repeating something `n` times.

Rust also has inclusive ranges. The `control-flow` example uses `..=` so that
both endpoints are included:

```rust
for a in 1..=a_max {
    for b in 1..=b_max {
        println!("gcd({a}, {b}) = {}", gcd(a, b));
    }
}
```

The range `1..=a_max` includes both `1` and `a_max`. By contrast, `1..a_max`
would stop before `a_max`.

Nested loops are useful for small tables or grid-like computations. Here they
compute `gcd(a, b)` for every pair of values in a small square domain.

## Functions

The `numerical-function` example shows a function with several floating-point
parameters and a floating-point return value:

```bash
cd source-code/numerical-function
cargo run -- --help
```

The polynomial function is:

```rust
fn polynomial(x: f64, a: f64, b: f64, c: f64) -> f64 {
    a * x.powi(2) + b * x + c
}
```

The parameter types are written explicitly. The return type appears after
`->`.

The final expression in the function body is returned because it has no
semicolon:

```rust
a * x.powi(2) + b * x + c
```

Adding a semicolon would turn the expression into a statement, and the function
would no longer return the computed value.

## Blocks And Expressions

Rust uses blocks delimited by braces:

```rust
{
    let x = 2.0;
    x * x
}
```

Many Rust constructs are expressions, including blocks and `if` expressions.
That means they can produce values. For example:

```rust
let weight = if i % 2 == 0 { 2.0 } else { 4.0 };
```

This pattern appears in the Simpson quadrature implementation used by the
`enum-match` example. Both branches must produce compatible types, because the
whole `if` expression has one resulting type.

## Tuples

A tuple groups a fixed number of values together without defining a named
struct type. Tuple types are written with parentheses:

```rust
let point: (f64, f64) = (1.0, 2.0);
```

This is a 2-tuple containing two `f64` values. Rust does not have a separate
standard `pair` type like C++ has `std::pair`; a pair of values is normally
represented as a 2-tuple.

Tuple fields can be accessed by position:

```rust
let x = point.0;
let y = point.1;
```

Tuples can also be destructured into separate bindings:

```rust
let (x, y) = point;
```

Destructuring is useful when a function, iterator, or pattern produces a tuple
and the code wants to name the individual components.

For example, a matrix index can be represented as a 2-tuple:

```rust
let index: (usize, usize) = (2, 3);
let (row, col) = index;
```

Tuples are best for small, local groupings where the meaning of each position
is obvious from context. If the grouped values represent a concept that appears
throughout a program, a `struct` with named fields is usually clearer.

## Enums As A Set Of Choices

The `enum-match` example computes a numerical integral using one of two
quadrature methods:

```bash
cd source-code/enum-match
cargo run
cargo run -- --method gauss
```

The available methods are represented by an enum:

```rust
#[derive(Clone, ValueEnum)]
enum QuadratureMethod {
    Simpson,
    Gauss,
}
```

An enum is a type whose value is one of a fixed set of variants. Here, a
`QuadratureMethod` is either `QuadratureMethod::Simpson` or
`QuadratureMethod::Gauss`.

This is more precise than using a string throughout the program. Once command
line parsing has succeeded, the rest of the program works with a real Rust
type.

## Selecting Behavior With `match`

The selected quadrature method is handled with `match`:

```rust
let result = match args.method {
    QuadratureMethod::Simpson => simpson::quad(f, a, b, 1000),
    QuadratureMethod::Gauss => gauss::quad(f, a, b),
};
```

Each arm handles one enum variant. This has two useful properties:

- The code says explicitly what happens for each method.
- The compiler can check whether all variants are covered.

The value produced by the selected arm becomes the value assigned to `result`.
As with `if`, the arms must produce compatible types.

## Passing Functions And Closures

The `enum-match` example defines the function to integrate as a closure:

```rust
let f = |x: f64| x.sin();
```

The quadrature functions accept any callable value with the type `Fn(f64) ->
f64`:

```rust
pub fn quad<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    // implementation
}
```

This allows the quadrature code to work with different mathematical functions
without rewriting the integration algorithm.

At this stage, the important idea is that functions can receive behavior as an
argument. The generic syntax and trait bound can be treated as a preview of the
later modules on generics and traits.

## Splitting Code Into Modules

The `enum-match` example is split across several files:

```text
src/
├── main.rs
├── simpson.rs
└── gauss.rs
```

The modules are declared at the top of `main.rs`:

```rust
mod simpson;
mod gauss;
```

This tells Rust to compile `src/simpson.rs` and `src/gauss.rs` as modules of
the current crate. Public functions from those modules can then be called with
qualified names:

```rust
simpson::quad(f, a, b, 1000)
gauss::quad(f, a, b)
```

Splitting source files this way keeps `main.rs` focused on program setup and
dispatch, while the numerical algorithms live in separate files.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run `source-code/control-flow` and inspect the table of greatest common
   divisors.

2. Change `a_max` and `b_max` in `source-code/control-flow/src/main.rs` and run
   the program again.

3. Modify the `gcd` function to print the intermediate values of `a` and `b`
   for one small input pair.

4. Replace the inclusive range `1..=a_max` with `1..a_max` and observe how the
   output changes.

5. Run `source-code/numerical-function` with different coefficients:

   ```bash
   cargo run -- --a 1.0 --b 2.0 --c -3.0
   ```

6. Add a second function to `source-code/numerical-function`, such as a cubic
   polynomial, and call it from `main`.

7. Run `source-code/enum-match` with both quadrature methods:

   ```bash
   cargo run
   cargo run -- --method gauss
   ```

8. Change the function being integrated in `source-code/enum-match/src/main.rs`
   from `sin(x)` to `cos(x)` and run both quadrature methods again.

9. Add a new enum variant name in `QuadratureMethod` without adding a matching
   `match` arm, then run `cargo check` and read the compiler diagnostic. Restore
   the original code afterward.

## Discussion Points

This module is a good place to emphasize:

- `if`, `while`, and `for` require explicit conditions and iteration sources.
- Function signatures document the types flowing into and out of a computation.
- The absence or presence of a semicolon can change whether an expression is
  returned.
- Enums make small sets of choices explicit in the type system.
- `match` is useful when different variants require different behavior.
- Modules help keep source files focused as examples grow.

## Connection To Later Modules

The ideas in this module are used throughout the rest of the training:

- Ownership examples use functions to show moves and borrows.
- Struct examples attach behavior to data through methods.
- Trait examples generalize the enum-based quadrature design.
- Iterator examples replace some explicit loops with iterator pipelines.
- Julia set and N-body examples use branches, loops, functions, modules, and
  larger program structure in more realistic programs.

Once participants are comfortable with control flow and program structure, they
are ready to focus on ownership, borrowing, and mutation.
