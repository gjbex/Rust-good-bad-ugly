# Error Handling

This module introduces Rust's explicit approach to missing values and
recoverable errors. Instead of using null values or exceptions for ordinary
control flow, Rust programs commonly use `Option<T>` and `Result<T, E>`.

The main example extends the matrix type from earlier modules with checked
indexing. The goal is to make out-of-bounds access visible in the type
signature instead of silently producing invalid behavior.

## Learning Objectives

After completing this module, participants should be able to:

- Use `Option<T>` to represent a value that may be absent.
- Use `Result<T, E>` to represent an operation that may fail.
- Explain the difference between `Some`, `None`, `Ok`, and `Err`.
- Convert an `Option` into a `Result` with `ok_or_else`.
- Transform the value inside an `Option` with `map`.
- Use the `?` operator to return early from a failing operation.
- Handle errors at the call site with `expect` when failure indicates a bug.
- Decide whether a function should return `Option` or `Result`.
- Recognize when panicking is less appropriate than returning an error.

## Prerequisites

Participants should already be comfortable with:

- Defining structs and methods.
- Shared and mutable references.
- Basic enum syntax.
- Closures.
- Matrix indexing concepts from the structs and traits modules.

The main example used in this module is:

- `source-code/error-handling`

## Missing Values With `Option`

`Option<T>` represents either a value of type `T` or no value:

```rust
Some(value)
None
```

The matrix example uses `Option<usize>` for checked index calculation:

```rust
fn index(&self, row: usize, col: usize) -> Option<usize> {
    if row < self.rows && col < self.cols {
        Some(row * self.cols + col)
    } else {
        None
    }
}
```

This function returns `Some(flat_index)` when the row and column are valid. It
returns `None` when the requested element is outside the matrix.

Run the example with:

```bash
cd source-code/error-handling
cargo run -- --help
```

## Transforming `Option` With `map`

The `get` method uses the checked index to return a matrix value:

```rust
pub fn get(&self, row: usize, col: usize) -> Option<f64> {
    self.index(row, col).map(|index| self.data[index])
}
```

The `map` call transforms the value inside `Some`:

- If `index(row, col)` returns `Some(index)`, `map` applies the closure and
  returns `Some(self.data[index])`.
- If `index(row, col)` returns `None`, `map` leaves it as `None`.

This avoids a manual `match` for a simple transformation.

The return type communicates the possibility of absence:

```rust
Option<f64>
```

A caller cannot ignore that possibility accidentally.

## Recoverable Failure With `Result`

`Result<T, E>` represents either success or failure:

```rust
Ok(value)
Err(error)
```

The matrix `set` method can fail if the requested index is out of bounds:

```rust
pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String> {
    let index = self
        .index(row, col)
        .ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))?;
    self.data[index] = value;
    Ok(())
}
```

The success type is `()` because a successful `set` operation does not need to
return a meaningful value. It only needs to signal that the mutation succeeded.

The error type is `String` because the example returns a human-readable error
message.

## Converting `Option` To `Result`

The `index` helper returns `Option<usize>`, but `set` wants to return a
`Result<(), String>`. The conversion happens here:

```rust
.ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))?
```

`ok_or_else` converts:

- `Some(index)` into `Ok(index)`;
- `None` into `Err(message)`.

The closure:

```rust
|| format!("matrix index ({row}, {col}) is out of bounds")
```

constructs the error message only when it is needed.

Use this pattern when absence is detected by one helper function, but the
public operation should report a recoverable error with context.

## The `?` Operator

The `?` operator propagates an error from the current function:

```rust
let index = self
    .index(row, col)
    .ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))?;
```

If the expression before `?` is `Ok(index)`, the `index` value is extracted and
execution continues.

If it is `Err(error)`, the current function returns early with that error.

This keeps the successful path readable while still handling failure
explicitly. Without `?`, the same code would need a `match`:

```rust
let index = match self
    .index(row, col)
    .ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))
{
    Ok(index) => index,
    Err(error) => return Err(error),
};
```

The `?` operator is one of the main reasons Rust error-handling code can remain
compact without hiding the fact that a function can fail.

## Handling Errors At The Call Site

The example fills the matrix using loop indices that are known to be in bounds:

```rust
for i in 0..matrix.rows() {
    for j in 0..matrix.cols() {
        matrix
            .set(i, j, (i * matrix.cols() + j) as f64)
            .expect("loop indices should be in bounds");
    }
}
```

The call to `expect` says: if this operation fails, stop the program and print
this message.

That is acceptable here because failure would indicate a programming mistake in
the loop bounds. The loops use `0..matrix.rows()` and `0..matrix.cols()`, so
the indices should be valid.

The same pattern appears when reading values back:

```rust
let value = matrix.get(i, j).expect("loop indices should be in bounds");
```

Use `expect` when failure would mean the programmer's assumptions are wrong.
For ordinary user input or file-system errors, returning or reporting the error
is usually better than panicking.

## `Option` Or `Result`?

Use `Option<T>` when absence is expected and no additional explanation is
needed:

```rust
fn index(&self, row: usize, col: usize) -> Option<usize>
```

The caller only needs to know whether the index exists.

Use `Result<T, E>` when failure should carry information:

```rust
fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String>
```

Here, an out-of-bounds write is a failed operation and the error message can
explain what went wrong.

As examples grow, `Result` becomes especially important for:

- opening files;
- reading data;
- parsing input;
- validating command-line parameters;
- writing output;
- reporting invalid configuration.

## Panics Versus Recoverable Errors

Rust also has panics. A panic stops normal execution, usually because the
program has reached a state it cannot sensibly recover from.

Panics are appropriate for programming errors and violated internal
assumptions. Recoverable errors are better represented with `Result`.

In this repository, both styles appear for teaching purposes:

- checked methods such as `get` and `set` return `Option` or `Result`;
- indexing syntax in the trait example panics for out-of-bounds access, similar
  to Rust slices;
- `expect` is used where the example knows loop indices should be valid.

The practical habit is to make expected failure explicit and reserve panics for
bugs or unrecoverable internal assumptions.

## Toward Fallible Programs

Many real command-line programs have a fallible `main` function:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // fallible work
    Ok(())
}
```

This allows `?` to be used in `main` when opening files, reading data, parsing
input, or writing output.

The iterator module already uses this style when reading CSV data:

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(args.file)?;
    for result in reader.deserialize() {
        let value: Values = result?;
        // use value
    }
    Ok(())
}
```

The exact error type can be refined later. At this stage, the important point
is that errors are part of the function signature and can be propagated with
`?`.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run the matrix error-handling example:

   ```bash
   cd source-code/error-handling
   cargo run -- --rows 3 --cols 4
   ```

2. Open `source-code/error-handling/src/matrix.rs` and identify the methods
   that return `Option` and `Result`.

3. Change the `get` method locally to use a manual `match` instead of `map`.

4. Add a call in `main.rs` that tries to read an out-of-bounds element and
   handles the `None` case with `match`.

5. Add a call in `main.rs` that tries to write an out-of-bounds element and
   prints the error message instead of using `expect`.

6. Temporarily remove the `?` from `set` and rewrite the error propagation with
   a manual `match`.

7. Change the error message in `ok_or_else` to include the valid matrix shape.

8. Compare the API of `get` with the API of `set`: discuss why one returns
   `Option<f64>` and the other returns `Result<(), String>`.

## Discussion Points

This module is a good place to emphasize:

- `Option` and `Result` make uncertainty visible in types.
- Absence and failure are related but not identical concepts.
- `map` is useful when transforming a successful `Option` value.
- `ok_or_else` is useful when converting absence into a meaningful error.
- `?` keeps the successful path readable while still propagating errors.
- `expect` is best reserved for cases where failure indicates a bug.
- Fallible `main` functions are common in programs that do I/O.

## Connection To Later Modules

Error handling becomes more important as examples grow:

- Project-organization examples use shared functions that should report
  failures consistently.
- Randomness and data-generation examples involve file creation and output.
- Julia set examples read command-line arguments or TOML configuration files.
- The N-body example writes CSV output and handles optional output files.

Once participants are comfortable with `Option`, `Result`, and `?`, they are
ready to study how larger Cargo packages organize shared code and tests.
