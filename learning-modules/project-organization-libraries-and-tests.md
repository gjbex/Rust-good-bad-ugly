# Project Organization, Libraries, And Tests

This module introduces Cargo package layouts that go beyond a single
`src/main.rs`. Earlier examples used one binary at a time. Larger examples
often need shared code, several related executables, and tests that verify
small pieces of behavior.

The focus here is structure: where code lives, how binaries share library code,
how Cargo selects which binary to run, and how unit tests are placed close to
the code they check.

## Learning Objectives

After completing this module, participants should be able to:

- Explain the difference between a binary target and a library target.
- Use `src/lib.rs` for code shared by multiple executables.
- Define multiple executables in one Cargo package with `[[bin]]`.
- Run a specific binary with `cargo run --bin`.
- Import shared package code from a binary target.
- Add unit tests in a `#[cfg(test)]` module.
- Mark test functions with `#[test]`.
- Use `assert!` and `assert_eq!`.
- Write numerical checks with tolerances.
- Decide which code belongs in `main.rs` and which code belongs in reusable
  modules or a library target.

## Prerequisites

Participants should already be comfortable with:

- Cargo projects and `Cargo.toml`.
- Modules declared with `mod`.
- Functions, structs, enums, and traits.
- Collections and text processing.
- Basic `Option` and `Result`.

The examples used in this module are:

- `source-code/hashmap-hashset`
- `source-code/enum-match`
- `source-code/traits`

## From One Binary To Several Targets

A minimal Rust binary project has one main executable:

```text
src/
└── main.rs
```

That is enough for many small examples. The `hashmap-hashset` example is a
larger package with several related executables:

```text
src/
├── lib.rs
├── generate-data.rs
├── read-errors.rs
└── count-nucleotides.rs
```

The three executable programs are:

- `generate-data`
- `read-errors`
- `count-nucleotides`

They all work with the same DNA-like sequence vocabulary, so the shared
definitions live in `src/lib.rs`.

## Binary Targets

Each executable in a Cargo package is a binary target. The `hashmap-hashset`
package declares its binaries explicitly in `Cargo.toml`:

```toml
[[bin]]
name = "generate-data"
path = "src/generate-data.rs"

[[bin]]
name = "read-errors"
path = "src/read-errors.rs"

[[bin]]
name = "count-nucleotides"
path = "src/count-nucleotides.rs"
```

Each binary file has its own `main` function. Run a specific binary with:

```bash
cargo run --bin generate-data -- --count 800 --file data.txt
cargo run --bin read-errors -- --file data.txt --output errors.txt --error-rate 0.1
cargo run --bin count-nucleotides -- --file errors.txt
```

The first `--` separates Cargo arguments from arguments passed to the selected
program.

## Library Targets

When a package contains `src/lib.rs`, Cargo builds a library target as well.
This library can be used by the package's binaries.

The shared library code in `hashmap-hashset` is:

```rust
pub const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
pub const ERROR_TOKENS: [char; 12] = ['B', 'D', 'E', 'F', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O'];

pub fn is_valid_nucleotide(value: char) -> bool {
    VALID_NUCLEOTIDES.contains(&value)
}

pub fn is_error_token(value: char) -> bool {
    ERROR_TOKENS.contains(&value)
}
```

The constants and functions are marked `pub` because the binary targets need to
use them.

## Using Library Code From Binaries

Binary targets can import the package's library by using the package name with
hyphens converted to underscores.

The package is named:

```toml
name = "hashmap-hashset"
```

The corresponding Rust crate name is:

```rust
hashmap_hashset
```

For example, `count-nucleotides.rs` imports shared definitions with:

```rust
use hashmap_hashset::{VALID_NUCLEOTIDES, is_valid_nucleotide};
```

This keeps the valid nucleotide list in one place. The generator, error
injector, and counter do not each need their own separate copy of the same
definition.

## What Belongs In `main`?

A useful rule is that `main` should coordinate the program:

- parse command-line arguments;
- open input and output resources;
- call domain logic;
- report results.

Reusable logic should move into functions, modules, or `src/lib.rs`.

For the nucleotide example, the valid nucleotide list and helper functions are
shared domain logic, so they belong in the library. The details of command-line
arguments and file names remain in the individual binary programs.

This separation becomes more important when several executables need to agree
on the same rules.

## Unit Tests

Rust unit tests are usually placed close to the code they test. A common
pattern is a test module at the bottom of a source file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_valid_nucleotides() {
        for nucleotide in VALID_NUCLEOTIDES {
            assert!(is_valid_nucleotide(nucleotide));
        }
    }
}
```

The `#[cfg(test)]` attribute means the module is compiled only when running
tests.

The `#[test]` attribute marks a function as a test case.

Run tests with:

```bash
cargo test
```

## Using `super` In Tests

Tests inside a nested `tests` module often need access to items from the parent
module. This is done with `super`:

```rust
use super::*;
```

or more selectively:

```rust
use super::quad;
```

In `source-code/enum-match/src/simpson.rs`, the test imports the quadrature
function with:

```rust
use super::quad;
```

This means "use `quad` from the module that contains this test module."

## Assertions

Tests usually check behavior with assertion macros.

Use `assert!` for Boolean conditions:

```rust
assert!(is_valid_nucleotide(nucleotide));
```

Use `assert_eq!` when two values should be equal:

```rust
assert_eq!(matrix[(0, 0)], 1);
assert_eq!(matrix[(1, 1)], 4);
```

`assert_eq!` is often preferable for equality checks because failure messages
show both the left and right values.

## Testing Error Paths

Tests should cover error paths as well as successful paths. The matrix trait
example checks that ragged nested vectors are rejected:

```rust
#[test]
fn rejects_ragged_rows() {
    let result = Matrix::try_from(vec![vec![1, 2], vec![3]]);

    assert!(result.is_err());
}
```

This test does not need to inspect the exact error message. It only verifies
that invalid input is rejected.

For user-facing tools, more detailed tests may check the specific error text,
but that can make tests more brittle.

## Numerical Tests With Tolerances

Numerical code often should not be tested with exact floating-point equality.
The `enum-match` example tests Simpson integration of `sin(x)` on `[0, pi]`.
The exact result is `2`, but the computed result is compared with a tolerance:

```rust
#[test]
fn integrates_sine_on_zero_to_pi() {
    let result = quad(|x| f64::sin(x), 0.0, std::f64::consts::PI, 1000);

    assert!((result - 2.0).abs() < 2.0e-12);
}
```

The tolerance should be chosen based on the method, expected accuracy, and
floating-point scale of the problem. A tolerance that is too loose may miss
real bugs. A tolerance that is too tight may fail for unimportant numerical
noise.

## Testing Trait Behavior

The `traits` example uses tests to check behavior provided by trait
implementations:

```rust
#[test]
fn displays_matrix_rows() {
    let matrix =
        Matrix::try_from(vec![vec![1, 2], vec![3, 4]]).expect("rows have the same length");

    assert_eq!(matrix.to_string(), "1 2\n3 4");
}
```

This test checks the `Display` implementation through the public behavior it
enables: converting the matrix to a string.

Other tests check indexing, mutable indexing, borrowed iteration, mutable
borrowed iteration, and owned iteration. That is a useful pattern: test the
syntax and behavior that users of the type will actually rely on.

## Running Specific Tests

For small examples, running all tests is usually fine:

```bash
cargo test
```

Cargo can also run tests whose names match a filter:

```bash
cargo test displays_matrix_rows
```

This is useful when working on one behavior at a time.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Open `source-code/hashmap-hashset/Cargo.toml` and identify the three
   `[[bin]]` sections.

2. Run each binary:

   ```bash
   cd source-code/hashmap-hashset
   cargo run --bin generate-data -- --count 200 --file data.txt
   cargo run --bin read-errors -- --file data.txt --output errors.txt --error-rate 0.2
   cargo run --bin count-nucleotides -- --file errors.txt
   ```

3. Open `src/lib.rs` and identify which items are public.

4. Add a new helper function to `src/lib.rs`, such as:

   ```rust
   pub fn is_known_token(value: char) -> bool {
       is_valid_nucleotide(value) || is_error_token(value)
   }
   ```

5. Add a unit test for the new helper.

6. Run:

   ```bash
   cargo test
   ```

7. Open `source-code/enum-match/src/simpson.rs` and inspect the numerical test.

8. Change the tolerance temporarily and observe when the test fails.

9. Open `source-code/traits/src/matrix.rs` and run one specific test by name:

   ```bash
   cd source-code/traits
   cargo test displays_matrix_rows
   ```

10. Add a test that checks one additional behavior, such as formatting a
    one-row matrix.

## Discussion Points

This module is a good place to emphasize:

- A Cargo package can contain a library target and multiple binary targets.
- `src/lib.rs` is a good home for shared reusable code.
- Binaries should coordinate program behavior rather than duplicate shared
  rules.
- Unit tests are usually easiest to maintain when they live close to the code
  they test.
- Tests should cover success paths and meaningful error paths.
- Floating-point tests usually need tolerances, not exact equality.
- Test names should describe behavior.

## Connection To Later Modules

Project organization and tests support the larger examples that follow:

- Randomness examples benefit from testable helper functions and reproducible
  seeds.
- Julia set variants reuse earlier matrix and configuration patterns.
- The N-body example combines several responsibilities that would naturally be
  split into modules in a larger project.

Once participants understand how to organize shared code and tests, they are
ready to study reproducible random data generation and then larger integrated
examples.
