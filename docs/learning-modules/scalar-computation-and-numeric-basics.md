# Scalar Computation And Numeric Basics

This module introduces Rust's scalar types and the numeric operations that are
most relevant for small scientific and technical programs. It builds on the
project workflow from `docs/learning-modules/getting-started-with-rust-projects.md`: each
example is a small Cargo project that can be built, run, inspected, and
modified.

The emphasis is on explicitness. Rust does not silently convert between many
numeric types, and that is an important part of how the language avoids
ambiguous or accidental computations.

## Learning Objectives

After completing this module, participants should be able to:

- Recognize common integer, floating-point, Boolean, character, and pointer-size
  types.
- Explain the difference between signed and unsigned integer types.
- Use integer and floating-point arithmetic operators.
- Explain the difference between integer division and floating-point division.
- Use Euclidean division and remainder for signed integers.
- Call mathematical methods on floating-point values.
- Use floating-point constants from the standard library.
- Convert integer values to floating-point values explicitly.
- Understand why Rust avoids implicit double promotion.
- Use `num-complex` for complex arithmetic.

## Prerequisites

Participants should already be comfortable with:

- Running a Cargo project with `cargo run`.
- Checking a project with `cargo check`.
- Opening and lightly editing `src/main.rs`.
- Reading simple compiler diagnostics.

The examples used in this module are:

- `source-code/basic-types`
- `source-code/math`
- `source-code/numerical-function`
- `source-code/no-double-promotion`
- `source-code/complex-numbers`

## Scalar Types

Rust has several families of scalar types:

- Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
- Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.
- Floating-point values: `f32`, `f64`.
- Booleans: `bool`.
- Unicode scalar values: `char`.

The `basic-types` example prints the minimum and maximum values for many of
these types:

```bash
cd source-code/basic-types
cargo run
```

For fixed-width integer types, the number in the type name is the number of
bits. For example, `i32` is a signed 32-bit integer, and `u64` is an unsigned
64-bit integer.

The `isize` and `usize` types have the same width as a pointer on the target
platform. They are commonly used for indexing and sizes, especially `usize`.

## Floating-Point Types

Rust has two built-in floating-point types:

- `f32`: single precision.
- `f64`: double precision.

The `basic-types` example prints useful associated constants such as:

- `f32::MIN`
- `f32::MAX`
- `f32::MIN_POSITIVE`
- `f32::EPSILON`
- `f64::MIN`
- `f64::MAX`
- `f64::MIN_POSITIVE`
- `f64::EPSILON`

It also shows constants from the standard library, such as:

- `std::f32::consts::PI`
- `std::f32::consts::FRAC_1_SQRT_2`
- `std::f64::consts::E`
- `std::f64::consts::TAU`

These constants are namespaced by type, so the `f32` and `f64` versions are
distinct.

## Type Inference And Explicit Types

Rust often infers types from context:

```rust
let x = 17;
let y = 5.2;
```

For teaching examples and numerical code, it is often clearer to write the
type explicitly when the type matters:

```rust
let a: i32 = 17;
let b: i32 = 5;
let x: f64 = 17.3;
let y: f64 = 5.2;
```

Explicit types are especially useful when comparing integer and floating-point
behavior.

## Arithmetic Operators

The `math` example illustrates arithmetic for both integers and floating-point
values:

```bash
cd source-code/math
cargo run
```

The main arithmetic operators are:

- `+` for addition.
- `-` for subtraction.
- `*` for multiplication.
- `/` for division.
- `%` for remainder.

For integers, division discards the fractional part:

```rust
let a: i32 = 17;
let b: i32 = 5;

println!("{}", a / b);
println!("{}", a % b);
```

For floating-point values, division produces a floating-point result:

```rust
let x: f64 = 17.3;
let y: f64 = 5.2;

println!("{}", x / y);
println!("{}", x % y);
```

This is why the example uses different values for integer and floating-point
arithmetic: the outputs show that these are related operations, but not the
same computation.

## Division And Remainder For Negative Integers

Signed integer division can be subtle when negative values are involved. Rust's
ordinary `/` and `%` operators use truncating division. The `math` example also
shows Euclidean division:

```rust
let a: i32 = -17;
let b: i32 = 5;

println!("{}", a / b);
println!("{}", a % b);
println!("{}", a.div_euclid(b));
println!("{}", a.rem_euclid(b));
```

For algorithms where the remainder should be non-negative, such as indexing
periodic domains, `div_euclid` and `rem_euclid` are often the clearer choice.

## Mathematical Functions

Floating-point mathematical functions are implemented as methods on `f32` and
`f64` values.

Examples from `source-code/math` include:

```rust
let angle = std::f64::consts::FRAC_PI_6;
let value = 2.0_f64;

println!("{}", angle.sin());
println!("{}", angle.cos());
println!("{}", angle.tan());
println!("{}", value.sqrt());
println!("{}", value.powi(8));
println!("{}", value.powf(0.5));
println!("{}", value.exp());
println!("{}", value.ln());
println!("{}", value.log10());
```

Rounding and absolute-value methods include:

```rust
let x = -3.75_f64;

println!("{}", x.abs());
println!("{}", x.floor());
println!("{}", x.ceil());
println!("{}", x.round());
println!("{}", x.trunc());
```

The method-call syntax is important: these are functions associated with the
floating-point type and called on a value.

## Numeric Functions And Explicit Conversion

The `numerical-function` example defines a small polynomial function:

```bash
cd source-code/numerical-function
cargo run -- --help
```

The core function has typed parameters and a typed return value:

```rust
fn polynomial(x: f64, a: f64, b: f64, c: f64) -> f64 {
    a * x.powi(2) + b * x + c
}
```

The example also loops over integer values and converts them to `f64` when
constructing floating-point coordinates:

```rust
let delta_x = (x_max - x_min) / (nr_points as f64 - 1.0);

for i in 0..nr_points {
    let x = x_min + i as f64 * delta_x;
    let result = polynomial(x, args.a, args.b, args.c);
    println!("{x} {result}");
}
```

The `as f64` conversions are explicit. Rust does not automatically convert an
integer loop index to a floating-point value.

## Avoiding Implicit Double Promotion

The `no-double-promotion` example shows that Rust uses context to infer the
type of floating-point literals:

```bash
cd source-code/no-double-promotion
cargo run --release
```

In the example, the function argument is `f32`:

```rust
fn compute_polynom(x: f32) -> f32 {
    let a = 3.0;
    let b = 2.0;
    let c = 1.0;
    a * x * x + b * x + c
}
```

Because the literals are used in an `f32` expression, Rust infers them as
`f32`. This avoids a common problem in C, C++, and Fortran where single
precision values may accidentally be promoted to double precision and then
converted back.

The important lesson is not that Rust guesses magically, but that every
expression still has a concrete type. If the context is not clear enough, the
compiler will ask for more information.

## Complex Numbers

Complex numbers are not built into Rust's standard library. Scientific Rust
programs commonly use the `num-complex` crate.

Run the example with:

```bash
cd source-code/complex-numbers
cargo run
```

The example imports `Complex64`, which is a complex number with `f64` real and
imaginary parts:

```rust
use num_complex::Complex64;

let z1 = Complex64 { re: 1.0, im: 2.0 };
let z2 = Complex64 { re: 3.0, im: 4.0 };

println!("{}", z1 + z2);
println!("{}", z1 * z2);
println!("{}", z1.re);
println!("{}", z1.im);
println!("{}", z1.norm());
```

This example reinforces two earlier points:

- Numeric behavior can be extended through crates.
- External types are brought into scope with `use`.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run `source-code/basic-types` and identify the ranges of `i32`, `u32`,
   `f32`, and `f64`.

2. Modify `source-code/basic-types/src/main.rs` to print one additional
   floating-point constant for both `f32` and `f64`.

3. Run `source-code/math` and compare integer division with floating-point
   division.

4. Change the integer values in `source-code/math/src/main.rs` and predict the
   result of `/`, `%`, `div_euclid`, and `rem_euclid` before running the code.

5. Add one more mathematical function call to `source-code/math`, such as
   `value.cbrt()` or `value.log2()`.

6. Run `source-code/numerical-function` with different polynomial
   coefficients:

   ```bash
   cargo run -- --a 2.0 --b -1.0 --c 0.5
   ```

7. Remove one `as f64` conversion from `source-code/numerical-function` and run
   `cargo check`. Read the compiler diagnostic, then restore the conversion.

8. Run `source-code/no-double-promotion` and inspect the printed type names.

9. Run `source-code/complex-numbers` and add a calculation of `z1 - z2`.

## Discussion Points

This module is a good place to emphasize:

- Rust's numeric types are explicit and concrete.
- Integer and floating-point arithmetic have different semantics.
- Conversions between numeric types should be visible in the code.
- Floating-point constants and mathematical functions are type-specific.
- Scientific code often needs external crates for domain-specific types such
  as complex numbers.
- The compiler is a useful guide when a numeric expression has an ambiguous or
  inconsistent type.

## Connection To Later Modules

The concepts in this module appear throughout the rest of the training:

- Control-flow examples use integer ranges and explicit conversions.
- Iterator examples process numeric collections.
- The Julia set examples use complex arithmetic and floating-point constants.
- The N-body simulation uses vectors of floating-point values, mathematical
  functions, random initial conditions, and numerical diagnostics.

Once participants are comfortable with scalar values and numeric expressions,
they are ready to move on to control flow, functions, and pattern matching.
