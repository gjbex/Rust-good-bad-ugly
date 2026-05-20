# Ownership, Borrowing, And Mutation

This module introduces Rust's ownership and borrowing model. These rules are
central to Rust: they determine when a value can be used, when it can be
modified, and whether a function receives ownership of data or only temporary
access to it.

The module builds on the earlier material about scalar values, functions, and
control flow. The examples are intentionally small because ownership is easiest
to learn when each program isolates one rule at a time.

## Learning Objectives

After completing this module, participants should be able to:

- Explain that bindings are immutable by default.
- Use `mut` when a local binding has to change.
- Distinguish between copying scalar values and moving owned data.
- Explain what happens when a `Vec<T>` is passed by value.
- Use `clone` when an explicit copy of owned data is required.
- Borrow data with shared references such as `&T` and `&[T]`.
- Borrow data with mutable references such as `&mut T` and `&mut [T]`.
- Explain why shared and mutable borrows cannot overlap arbitrarily.
- Prefer slices over `&Vec<T>` for read-only sequence arguments.
- Choose function signatures based on whether a function owns, reads, or
  modifies its input.

## Prerequisites

Participants should already be comfortable with:

- Defining and calling functions.
- Reading function signatures.
- Using `for` loops.
- Working with scalar values such as `f64`.
- Reading simple compiler diagnostics.

The examples used in this module are:

- `source-code/mutable-variables`
- `source-code/mutable-borrowing`
- `source-code/borrowing-vectors`
- `source-code/copy-vs-move`

## Immutable By Default

Rust bindings are immutable by default:

```rust
let x = 1.0;
```

Once `x` has been assigned, it cannot be changed unless the binding is declared
with `mut`:

```rust
let mut x = 1.0;
x += 0.1;
```

The `mutable-variables` example uses a mutable `x` value while stepping through
the points where a polynomial is evaluated:

```bash
cd source-code/mutable-variables
cargo run -- --help
```

The relevant part is:

```rust
let mut x = x_min;
let delta_x = (x_max - x_min) / (nr_points as f64 - 1.0);

for _ in 0..nr_points {
    let result = polynomial(x, args.a, args.b, args.c);
    println!("{x} {result}");
    x += delta_x;
}
```

The `mut` keyword says that the binding `x` may be assigned a new value. This
does not make every value in the program mutable; mutation is still local and
explicit.

## Mutable References

Sometimes a function needs to modify a value owned by its caller. Rust makes
that visible with a mutable reference:

```rust
fn rhs(x: f64, dxdt: &mut f64, _t: f64) {
    *dxdt = -x;
}
```

This appears in `source-code/mutable-borrowing`:

```bash
cd source-code/mutable-borrowing
cargo run
```

The caller passes a mutable reference with `&mut`:

```rust
rhs(x, &mut dxdt, t);
```

Inside the function, `dxdt` is a reference to a value owned elsewhere. The
assignment uses `*dxdt` to write through the reference:

```rust
*dxdt = -x;
```

This style is similar to a non-const reference parameter in C++. It is useful
for explaining mutable borrowing, but it is not always the most idiomatic Rust
API. For a simple computation, returning the computed value is often clearer:

```rust
fn rhs(x: f64, _t: f64) -> f64 {
    -x
}
```

The important point is that mutation through a reference is explicit both in
the function signature and at the call site.

## Copying Scalars

Simple scalar values such as `f64` implement the `Copy` trait. Assigning them
to another binding copies the value:

```rust
let x = 5.0;
let y = x;
println!("x: {x}, y: {y}");
```

After the assignment, both `x` and `y` can still be used. This is the behavior
most programmers expect for small numeric values.

The `copy-vs-move` example starts with this contrast:

```bash
cd source-code/copy-vs-move
cargo run
```

## Moving Owned Data

Owned data structures such as `Vec<f64>` do not implement `Copy`. Assigning a
vector to another binding moves ownership:

```rust
let xs = vec![1.0, 2.0, 3.0];
let ys = xs;
```

After this assignment, `ys` owns the vector and `xs` can no longer be used. The
data was not silently copied.

The same rule applies when passing a vector by value to a function:

```rust
fn mean_move(values: Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

let xs = vec![1.0, 2.0, 3.0];
let mean = mean_move(xs);
```

After the call, `xs` has been moved into `mean_move`. That is usually the wrong
signature for a read-only computation, because computing a mean should not need
to take ownership of the vector.

## Cloning Owned Data

If a real copy of owned data is needed, Rust makes that explicit with `clone`:

```rust
let xs = vec![1.0, 2.0, 3.0];
let ys = xs.clone();
```

After cloning, both vectors can be used because the vector contents have been
copied into a separate allocation.

This explicitness matters for performance. A vector clone may copy a large
amount of data, so Rust does not hide that operation inside an ordinary
assignment.

## Shared Borrowing

Most read-only functions should borrow data rather than take ownership of it.
The `copy-vs-move` example includes:

```rust
fn mean_borrow(values: &Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}
```

The caller passes a shared reference:

```rust
let xs = vec![1.0, 2.0, 3.0];
let mean = mean_borrow(&xs);
println!("Mean of xs: {mean}");
```

Because `mean_borrow` only borrows `xs`, ownership remains with the caller and
`xs` can still be used after the function call.

## Prefer Slices For Sequences

For read-only sequence data, a slice is usually more flexible than `&Vec<T>`:

```rust
fn mean_borrow_slice(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}
```

A slice, `&[f64]`, means "a borrowed view of contiguous `f64` values." It can
refer to an entire vector, part of a vector, or other contiguous storage.

The caller can pass a vector as a slice:

```rust
let xs = vec![1.0, 2.0, 3.0];
let mean = mean_borrow_slice(&xs);
```

This is a common Rust API design rule: if a function only needs to read a
sequence, prefer `&[T]` over `&Vec<T>`.

## Mutable Slices

When a function needs to modify a sequence in place, it can take a mutable
slice:

```rust
fn normalize(data: &mut [f64]) {
    let mean_value = mean(data);
    for value in data.iter_mut() {
        *value /= mean_value;
    }
}
```

This appears in `source-code/borrowing-vectors`:

```bash
cd source-code/borrowing-vectors
cargo run
```

The caller must have a mutable vector binding and must pass a mutable borrow:

```rust
let mut data = vec![3.1, 2.4, 5.6, 1.2, 4.8];
normalize(&mut data);
```

The `iter_mut` method produces mutable references to the elements. The loop
writes through those references:

```rust
*value /= mean_value;
```

Again, mutation is visible in the function signature, the call site, and the
body of the loop.

## Borrowing Rules

Rust allows many shared references or one mutable reference, but not both at
the same time when the references could overlap.

This is accepted:

```rust
{
    let first_value = &data[0];
    let this_mean = mean(&data);
    println!("First value: {first_value}, no problem using data");
}
```

Both borrows are read-only. The shared reference `first_value` is compatible
with the shared borrow used by `mean`.

This pattern is rejected:

```rust
{
    let first_value = &data[0];
    normalize(&mut data);
    println!("First value before normalization: {first_value}");
}
```

The reference `first_value` points into `data` and is used after the call to
`normalize`. Rust therefore rejects the mutable borrow in between, because
`normalize(&mut data)` could change the data while `first_value` is still
expected to refer to the old value.

The rule prevents a common class of bugs: reading through a reference while the
referenced data is being modified elsewhere.

## Borrowed References In Collections

Borrowing can also be extended by storing references in another collection:

```rust
let xs = vec![1.0, 2.0, 3.0];
let x_filtered: Vec<&f64> = xs.iter().filter(|&&x| x > 1.5).collect();
```

The values in `x_filtered` are references into `xs`. As long as those
references are used, `xs` remains borrowed.

If independent scalar values are needed instead, copy the values out:

```rust
let x_filtered: Vec<f64> = xs.iter().filter(|&&x| x > 1.5).copied().collect();
```

The `copied` adapter turns `&f64` items into independent `f64` values. After
that, the filtered vector no longer borrows from `xs`.

## Returning Owned Values

Ownership can also be transferred from a function to its caller:

```rust
fn return_vector() -> Vec<f64> {
    vec![1.0, 2.0, 3.0]
}

let xs = return_vector();
```

The function creates a vector and returns ownership to the caller. This is a
normal and efficient Rust pattern. Returning owned data does not mean the whole
vector is copied element by element.

## Choosing Function Signatures

A useful way to design function signatures is to ask what the function needs:

- If the function only needs a small scalar value, pass it by value.
- If the function only needs to read a sequence, pass `&[T]`.
- If the function needs to modify a sequence in place, pass `&mut [T]`.
- If the function must take responsibility for owned data, pass `Vec<T>` or
  another owning type by value.
- If the function creates new owned data, return it.

For example, a mean function should usually read data without taking ownership:

```rust
fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / (data.len() as f64)
}
```

A normalization function can modify data in place:

```rust
fn normalize(data: &mut [f64]) {
    let mean_value = mean(data);
    for value in data.iter_mut() {
        *value /= mean_value;
    }
}
```

These signatures communicate intent and let the compiler enforce it.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run `source-code/mutable-variables` and identify which binding has to be
   mutable.

2. Remove `mut` from that binding and run `cargo check`. Read the compiler
   diagnostic, then restore `mut`.

3. Run `source-code/mutable-borrowing` and follow where `dxdt` is modified.

4. Rewrite `rhs` locally so that it returns `f64` instead of writing through
   `&mut f64`, then update the call site.

5. Run `source-code/copy-vs-move` and compare the scalar copy with the vector
   move.

6. Uncomment one of the lines that is expected to fail after a move, run
   `cargo check`, and read the diagnostic. Restore the comment afterward.

7. Change `mean_borrow` to use a slice signature, then update the call site.

8. Run `source-code/borrowing-vectors` and inspect the normalization output.

9. Uncomment the borrow-checker example in `borrowing-vectors`, run
   `cargo check`, and identify which borrow is still active. Restore the
   original code afterward.

10. Add a function that shifts all values in a mutable slice by a constant:

    ```rust
    fn shift(data: &mut [f64], offset: f64) {
        for value in data.iter_mut() {
            *value += offset;
        }
    }
    ```

## Discussion Points

This module is a good place to emphasize:

- `mut` belongs to a binding; it is not a global property of a value.
- Moving ownership is different from copying data.
- `clone` should be visible because it can be expensive.
- Borrowing lets functions inspect data without taking ownership.
- Mutable borrowing makes in-place modification explicit.
- Slices are often better API boundaries than vectors.
- Borrow-checker errors usually point to overlapping access patterns that need
  to be made clearer.

## Connection To Later Modules

Ownership and borrowing appear in almost every later example:

- Struct examples store owned data in fields and expose behavior through
  methods.
- Trait examples use references and trait objects to share behavior without
  unnecessary ownership transfer.
- Iterator examples rely on borrowed iteration with `iter` and mutable
  iteration with `iter_mut`.
- File I/O examples use borrowed paths, owned strings, and fallible results.
- Julia set and N-body examples use ownership to manage arrays, simulation
  state, and output writers.

Once participants are comfortable reading function signatures in terms of
ownership, borrowing, and mutation, they are ready to define their own data
types with structs and methods.
