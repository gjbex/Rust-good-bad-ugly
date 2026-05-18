# Reusable Abstractions With Traits

This module introduces traits as Rust's main mechanism for shared behavior.
Structs let you define new data types; traits let you define what a type can
do. A type implements a trait when it provides the behavior required by that
trait.

The examples in this module show two complementary uses of traits:

- implementing standard library traits so a custom type works with familiar
  Rust syntax;
- defining a project-specific trait to express a shared numerical interface.

## Learning Objectives

After completing this module, participants should be able to:

- Explain what a trait represents.
- Implement standard traits for a custom type.
- Use `Index` and `IndexMut` to support indexing syntax.
- Use `Display` to control formatted output with `{}`.
- Use `TryFrom` for fallible conversion.
- Recognize associated types in trait implementations.
- Define a user-defined trait.
- Implement the same trait for multiple concrete types.
- Use trait bounds to require behavior from a generic type.
- Explain the role of `dyn Trait` in a trait object.
- Distinguish static dispatch from dynamic dispatch at a conceptual level.

## Prerequisites

Participants should already be comfortable with:

- Defining structs and methods.
- Generic structs such as `Matrix<T>`.
- Shared and mutable references.
- `Option` and `Result` at a basic level.
- Enums and `match`.
- Closures as function arguments at a basic level.

The examples used in this module are:

- `source-code/generic-structs`
- `source-code/traits`
- `source-code/user-defined-trait`

## What Is A Trait?

A trait is a named set of behavior. It describes methods, associated types, or
other requirements that an implementing type must provide.

For example, a type that implements `Display` defines how it should be printed
with `{}`:

```rust
println!("{value}");
```

A type that implements `Index` can be indexed with square brackets:

```rust
matrix[(row, col)]
```

Traits are not inheritance. They do not say that one type is a subclass of
another type. They say that a type supports a particular behavior.

## Standard Traits For A Matrix Type

The `traits` example extends the generic matrix from the previous module:

```bash
cd source-code/traits
cargo run
```

The matrix type is still a generic struct:

```rust
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}
```

The difference is that it now implements several standard traits:

- `Index`
- `IndexMut`
- `Display`
- `TryFrom`
- `IntoIterator`

These implementations let the type participate in familiar Rust syntax instead
of requiring every operation to be called as a named method.

## Indexing With `Index`

The `Index` trait defines immutable indexing. For the matrix, the index is a
pair of `usize` values:

```rust
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let flat_index = self
            .flat_index(row, col)
            .unwrap_or_else(|| panic!("matrix index ({row}, {col}) is out of bounds"));
        &self.data[flat_index]
    }
}
```

The associated type:

```rust
type Output = T;
```

tells Rust what kind of value is produced by indexing. The method returns a
shared reference to that value:

```rust
&Self::Output
```

After implementing `Index`, users can write:

```rust
let value = matrix[(row, col)];
```

The indexing implementation panics for out-of-bounds indices, matching the
behavior of Rust slices. A fallible API can still be provided separately with a
method returning `Option` or `Result`.

## Mutable Indexing With `IndexMut`

The `IndexMut` trait enables assignment through indexing:

```rust
impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let flat_index = self
            .flat_index(row, col)
            .unwrap_or_else(|| panic!("matrix index ({row}, {col}) is out of bounds"));
        &mut self.data[flat_index]
    }
}
```

The method takes `&mut self` and returns a mutable reference to the selected
element. This allows code such as:

```rust
matrix[(row, col)] = (row * matrix.cols() + col) as f64;
```

This should feel connected to the previous ownership module: mutable indexing
works because the matrix is mutably borrowed for the assignment.

## Formatting With `Display`

The `Display` trait defines how a value is formatted with `{}`:

```rust
impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if col > 0 {
                    write!(formatter, " ")?;
                }
                write!(formatter, "{}", self[(row, col)])?;
            }
            if row + 1 < self.rows {
                writeln!(formatter)?;
            }
        }
        Ok(())
    }
}
```

The implementation has a trait bound:

```rust
T: Display
```

This is needed because printing a `Matrix<T>` requires printing each element.
Rust cannot format arbitrary `T` values unless `T` itself implements
`Display`.

After implementing `Display`, the matrix can be printed directly:

```rust
println!("{matrix}");
```

## Fallible Conversion With `TryFrom`

The `TryFrom` trait expresses a conversion that can fail. The matrix example
uses it to build a matrix from nested vectors:

```rust
impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = String;

    fn try_from(rows: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let row_count = rows.len();
        let col_count = rows.first().map_or(0, Vec::len);

        if rows.iter().any(|row| row.len() != col_count) {
            return Err("all matrix rows must have the same length".to_string());
        }

        let data = rows.into_iter().flatten().collect();
        Ok(Self {
            rows: row_count,
            cols: col_count,
            data,
        })
    }
}
```

The associated type:

```rust
type Error = String;
```

tells Rust what kind of error the conversion can return.

The caller can then write:

```rust
let integer_matrix =
    Matrix::try_from(vec![vec![1, 0], vec![0, 2]])
        .expect("all rows have the same length");
```

This is a good fit because nested vectors may be ragged: one row might have a
different length from another.

## Iteration With `IntoIterator`

The `traits` example also implements `IntoIterator` for three cases:

- owned matrix iteration;
- shared borrowed matrix iteration;
- mutable borrowed matrix iteration.

Owned iteration consumes the matrix and yields owned elements:

```rust
impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
```

Borrowed iteration yields shared references:

```rust
impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
```

Mutable borrowed iteration yields mutable references:

```rust
impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
```

This supports ordinary loop syntax:

```rust
for value in &mut matrix {
    *value *= 0.5;
}
```

The lifetime parameter `'a` connects the lifetime of the iterator items to the
lifetime of the borrowed matrix. The detailed lifetime rules can be studied
later; here the practical point is that borrowed iteration returns references
that cannot outlive the matrix borrow.

## User-Defined Traits

Standard traits are useful when your type should support behavior Rust already
knows about. You can also define your own trait for behavior specific to your
domain.

The `user-defined-trait` example computes numerical integrals using different
quadrature rules:

```bash
cd source-code/user-defined-trait
cargo run
cargo run -- --method gauss
```

The shared interface is:

```rust
pub trait QuadratureRule {
    fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64;

    fn name(&self) -> &'static str;
}
```

Any quadrature rule that implements this trait must provide:

- an `integrate` method;
- a `name` method.

This separates what a quadrature rule does from how each rule implements the
algorithm.

## Implementing A User-Defined Trait

The Simpson rule stores its number of subdivisions:

```rust
pub struct Simpson {
    subdivisions: usize,
}
```

It implements the shared trait:

```rust
impl QuadratureRule for Simpson {
    fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
        let h = (b - a) / self.subdivisions as f64;
        let mut sum = f(a) + f(b);

        for i in 1..self.subdivisions {
            let x = a + i as f64 * h;
            let weight = if i % 2 == 0 { 2.0 } else { 4.0 };
            sum += weight * f(x);
        }

        h * sum / 3.0
    }

    fn name(&self) -> &'static str {
        "composite Simpson"
    }
}
```

The Gauss-Legendre rule also implements the same trait, but with a different
algorithm and no stored fields:

```rust
pub struct GaussLegendre10;
```

Both types can now be treated as quadrature rules, even though their
implementations are different.

## Trait Objects And `dyn Trait`

The concrete quadrature rule is selected from command-line arguments at run
time:

```rust
fn select_rule(args: &Args) -> Box<dyn QuadratureRule> {
    match args.method {
        QuadratureMethod::Simpson => Box::new(Simpson::new(args.subdivisions)),
        QuadratureMethod::Gauss => Box::new(GaussLegendre10),
    }
}
```

The return type:

```rust
Box<dyn QuadratureRule>
```

means "a heap-allocated value of some concrete type that implements
`QuadratureRule`." The concrete type might be `Simpson` or `GaussLegendre10`,
but the caller only relies on the trait methods.

The `dyn` keyword marks dynamic dispatch. The exact method implementation is
chosen at run time through the trait object.

That lets `main` use the selected rule uniformly:

```rust
let rule = select_rule(&args);
let result = rule.integrate(&f, a, b);

println!("using {} quadrature", rule.name());
```

## Static And Dynamic Dispatch

Rust has two common ways to use traits:

- static dispatch through generics and trait bounds;
- dynamic dispatch through trait objects such as `dyn Trait`.

Static dispatch is visible in a generic bound such as:

```rust
impl<T: Display> Display for Matrix<T>
```

The compiler knows the concrete type at compile time and can generate code for
that type.

Dynamic dispatch is visible in:

```rust
Box<dyn QuadratureRule>
```

The concrete type is selected at run time, and method calls go through the
trait object.

A practical rule of thumb:

- Use generics and trait bounds when the concrete type can remain known at
  compile time.
- Use trait objects when different concrete types must be selected and handled
  uniformly at run time.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run `source-code/traits`:

   ```bash
   cd source-code/traits
   cargo run
   ```

2. Identify where `IndexMut` enables assignment with `matrix[(row, col)] = ...`.

3. Change the `Display` implementation to separate matrix entries with commas
   instead of spaces.

4. Create a ragged nested vector in `main.rs` and pass it to
   `Matrix::try_from`. Observe the error path.

5. Add a loop over `&matrix` and print every value without consuming the matrix.

6. Add a loop over `&mut matrix` and scale every value by a different factor.

7. Run `source-code/user-defined-trait` with both quadrature methods:

   ```bash
   cd source-code/user-defined-trait
   cargo run
   cargo run -- --method gauss
   cargo run -- --method simpson --subdivisions 2000
   ```

8. Change the string returned by one `name` implementation and observe the
   program output.

9. Add a new quadrature-rule struct with a simple placeholder implementation,
   implement `QuadratureRule` for it, and add it to the command-line enum and
   `select_rule` match.

## Discussion Points

This module is a good place to emphasize:

- Traits describe behavior, not inheritance relationships.
- Standard traits connect custom types to familiar Rust syntax.
- Associated types are part of some trait contracts.
- Trait bounds should describe the behavior an implementation actually needs.
- User-defined traits are useful when several concrete types share the same
  domain-level role.
- Trait objects are useful when the concrete type is selected at run time.
- `dyn Trait` means dynamic dispatch through a trait object.

## Connection To Later Modules

Traits appear throughout more idiomatic Rust code:

- Iterator examples rely heavily on trait-based iterator behavior.
- Error-handling examples use standard traits for conversion and formatting.
- Data I/O examples use traits from crates such as `serde`.
- Julia set examples use generic and trait-based library APIs.
- The N-body example uses traits indirectly through iterators, formatting,
  parsing, and serialization.

Once participants are comfortable with traits, they are ready to study
collections and iterator pipelines in more depth.
