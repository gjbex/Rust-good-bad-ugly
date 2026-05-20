# Data Modeling With Structs And Methods

This module introduces user-defined data types in Rust. Up to this point, the
examples have mostly used built-in scalar types, vectors, functions, and small
enums. Structs make it possible to group related data into a domain type and
attach behavior to that type with methods.

The main example is a small matrix type. It is intentionally simple and meant
for teaching. For serious numerical work, use established crates such as
`ndarray`, `faer`, or `nalgebra`.

## Learning Objectives

After completing this module, participants should be able to:

- Define a `struct` with named fields.
- Explain why fields are often kept private.
- Create values through an associated function such as `new`.
- Implement methods in an `impl` block.
- Distinguish `self`, `&self`, and `&mut self`.
- Use accessor methods to expose selected internal state.
- Store two-dimensional data in a flat vector.
- Split a type definition into a separate source module.
- Define a generic struct such as `Matrix<T>`.
- Add trait bounds only where an operation needs them.

## Prerequisites

Participants should already be comfortable with:

- Functions and return values.
- Modules declared with `mod`.
- Ownership and borrowing.
- Shared and mutable references.
- Vectors and indexing.
- Basic use of `Option` and `Result` as return values is helpful, but not
  required.

The examples used in this module are:

- `source-code/structs-and-methods`
- `source-code/generic-structs`

## Why Structs?

A struct groups related values into one type. For a matrix, the relevant data
are:

- the number of rows;
- the number of columns;
- the element storage.

Instead of passing these three values around separately, the example defines a
single `Matrix` type:

```rust
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}
```

Run the example with:

```bash
cd source-code/structs-and-methods
cargo run -- --help
```

The fields are private because they are not marked `pub`. Code outside the
`matrix` module cannot directly modify `rows`, `cols`, or `data`. Instead, it
uses the methods provided by the type.

This is useful because the fields have an invariant: `data` should contain
`rows * cols` elements. If all external code could modify the fields directly,
it would be easy to create an inconsistent matrix.

## Implementing Methods

Methods are defined in an `impl` block:

```rust
impl Matrix {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
```

The `impl Matrix` block says that the functions inside are associated with the
`Matrix` type.

The first parameter determines how the method receives the value:

- `&self`: shared access; the method can read the value.
- `&mut self`: mutable access; the method can modify the value.
- `self`: owned access; the method consumes the value.

The `rows` and `cols` methods only read from the matrix, so they take `&self`.

## Associated Functions

An associated function belongs to a type but does not take `self`. Constructors
are commonly written this way:

```rust
impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }
}
```

`Self` means the type currently being implemented, here `Matrix`.

The function is called with type-qualified syntax:

```rust
let mut matrix = Matrix::new(args.rows, args.cols);
```

This creates a matrix with the requested dimensions and initializes the flat
storage with zeros.

## Accessor And Mutating Methods

The matrix example provides methods to get and set an element:

```rust
pub fn get(&self, row: usize, col: usize) -> f64 {
    self.data[row * self.cols + col]
}

pub fn set(&mut self, row: usize, col: usize, value: f64) {
    self.data[row * self.cols + col] = value;
}
```

The `get` method takes `&self` because it only reads from the matrix. The `set`
method takes `&mut self` because it modifies the matrix.

The indexing formula maps two-dimensional coordinates to a flat vector:

```rust
row * self.cols + col
```

This layout stores matrix entries row by row.

The caller sees the borrowing requirements at the call site:

```rust
let mut matrix = Matrix::new(args.rows, args.cols);

matrix.set(i, j, (i * matrix.cols() + j) as f64);
let value = matrix.get(i, j);
```

The matrix binding must be mutable because `set` requires `&mut self`.

## Keeping `main.rs` Focused

The matrix type is defined in its own file:

```text
src/
├── main.rs
└── matrix.rs
```

The module is declared in `main.rs`:

```rust
mod matrix;
use matrix::Matrix;
```

This keeps `main.rs` focused on command-line parsing and using the matrix,
while `matrix.rs` contains the definition of the matrix abstraction.

This is a common organization pattern:

- `main.rs` handles program setup.
- Separate modules define domain types and operations.

## Encapsulation

The matrix fields are private, but selected operations are public:

```rust
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}
```

The `pub struct Matrix` declaration makes the type itself visible outside the
module. The fields remain private because they do not have `pub`.

The public methods define the supported interface:

```rust
pub fn new(rows: usize, cols: usize) -> Self
pub fn rows(&self) -> usize
pub fn cols(&self) -> usize
pub fn get(&self, row: usize, col: usize) -> f64
pub fn set(&mut self, row: usize, col: usize, value: f64)
```

This lets the module control how matrices are created and modified. That is the
core idea of encapsulation in this example.

## Generic Structs

The first matrix stores only `f64` values:

```rust
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}
```

The `generic-structs` example generalizes this to any element type `T`:

```bash
cd source-code/generic-structs
cargo run -- --help
```

The generic definition is:

```rust
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}
```

`T` is a type parameter. A `Matrix<f64>` stores floating-point values, while a
`Matrix<i32>` stores integer values. The same abstraction can be reused for
different element types.

The example creates both:

```rust
let mut matrix = Matrix::new(args.rows, args.cols, 0.0_f64);
let mut integer_matrix = Matrix::new(2, 2, 0_i32);
```

## Generic Methods

Methods that do not require special behavior from `T` can be implemented for
all element types:

```rust
impl<T> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
```

The `get` method returns a borrowed element:

```rust
pub fn get(&self, row: usize, col: usize) -> Option<&T> {
    self.index(row, col).map(|index| &self.data[index])
}
```

Returning `Option<&T>` has two useful effects:

- `None` can represent an out-of-bounds index.
- The element does not have to be copied out of the matrix.

The `set` method receives a value of type `T` and moves it into the matrix:

```rust
pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), String> {
    let index = self
        .index(row, col)
        .ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))?;
    self.data[index] = value;
    Ok(())
}
```

This uses `Result` because setting an element can fail if the index is outside
the matrix. Detailed error-handling patterns are covered later; here the key
idea is that the method signature makes failure explicit.

## Trait Bounds Where Needed

The generic constructor needs to fill a vector with repeated copies of the
initial value:

```rust
data: vec![value; rows * cols],
```

That requires `T` to be cloneable. Instead of requiring `Clone` for the whole
type, the example puts the bound only on the constructor implementation:

```rust
impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![value; rows * cols],
        }
    }
}
```

This is an important Rust design habit: add trait bounds only where the
operation needs them. Reading dimensions does not require `T: Clone`, but
initializing all elements from one value does.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run `source-code/structs-and-methods` and create matrices with different
   dimensions:

   ```bash
   cargo run -- --rows 2 --cols 5
   ```

2. Open `source-code/structs-and-methods/src/matrix.rs` and identify which
   fields are private and which methods are public.

3. Add a public `len` method that returns the total number of stored elements:

   ```rust
   pub fn len(&self) -> usize {
       self.data.len()
   }
   ```

4. Call `matrix.len()` from `main.rs` and print the value.

5. Try to access `matrix.data` directly from `main.rs` and run `cargo check`.
   Read the compiler diagnostic, then restore the original code.

6. Add a private helper method to compute the flat index:

   ```rust
   fn index(&self, row: usize, col: usize) -> usize {
       row * self.cols + col
   }
   ```

   Use it from both `get` and `set`.

7. Run `source-code/generic-structs` and identify where the code creates a
   `Matrix<f64>` and a `Matrix<i32>`.

8. Add a `Matrix<bool>` or `Matrix<char>` to `source-code/generic-structs` and
   set one or two values.

9. Temporarily remove the `Clone` bound from `impl<T: Clone> Matrix<T>` and run
   `cargo check`. Read why the constructor needs that bound, then restore it.

## Discussion Points

This module is a good place to emphasize:

- Structs group related data into a named type.
- Methods attach behavior to that type.
- Private fields protect invariants.
- Associated functions such as `new` are commonly used as constructors.
- `&self` is for reading, `&mut self` is for modifying, and `self` is for
  consuming.
- Generic structs let one abstraction work with different element types.
- Trait bounds should be introduced where they are needed, not everywhere by
  default.

## Connection To Later Modules

Structs and methods are the basis for larger Rust designs:

- Trait modules build on method syntax and trait bounds.
- Iterator examples often expose borrowed access to internal data.
- Error-handling examples improve checked matrix access.
- Julia set examples use matrix-like storage for numerical output.
- The N-body example uses structs and methods to represent particles and
  systems.

Once participants are comfortable defining data types and methods, they are
ready to study traits as a way to express shared behavior across types.
