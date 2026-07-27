# Multidimensional Arrays And Stencil Computation

Scientific programs often represent fields, images, and simulation states as
multidimensional arrays. This module introduces the `ndarray` crate through a
two-dimensional heat-diffusion example and then uses tests to guide a
behavior-preserving refactoring.

The complete example is in `source-code/heat-diffusion`.

## Learning Objectives

By the end of this module, you should be able to:

- create a two-dimensional `Array2` and inspect its dimensions;
- index individual elements and iterate with their indices;
- select borrowed array regions with `slice` and `slice_mut`;
- combine equally shaped views with `Zip`;
- update a stencil computation without allocating a new array each step;
- expose array data through a read-only `ArrayView2`;
- use unit and black-box tests to protect a numerical refactoring.

## Prerequisites

This module builds on:

- structs and methods from Module 5;
- borrowing and mutation from Module 4;
- error handling from Module 8;
- unit tests from Module 9.

It belongs near the end of the course because the array operations make Rust's
ownership and borrowing rules concrete in a realistic numerical program.

## The Example Family

The directory contains two separate Cargo projects:

- `source-code/heat-diffusion/naive` expresses the stencil with explicit
  nested loops and element indexing;
- `source-code/heat-diffusion/ndarray-features` preserves the command-line
  behavior while using more of the `ndarray` API.

Both variants deliberately have the same inputs, validation, initial
condition, update equation, convergence rule, and output. This lets tests
distinguish a change in implementation from a change in behavior.

Start by running the tests:

```bash
cd source-code/heat-diffusion
cargo test --manifest-path naive/Cargo.toml
cargo test --manifest-path ndarray-features/Cargo.toml
./test_consistency.sh
```

## The Numerical Model

The simulation stores temperature on a square grid. For each interior point,
one explicit time step applies a five-point stencil:

```text
new = center + alpha * dt * (up + down + left + right - 4 * center)
```

Here `alpha` is the thermal diffusivity and `dt` is the time step. The grid
spacing is one in both directions.

The boundary temperatures remain fixed. The interior starts at zero except
for a circular hot spot in the center.

For this scheme, the stability check is:

```text
alpha * dt <= 0.25
```

The code validates this condition before running. Rust's memory safety does not
make an unstable numerical method stable; scientific constraints still need
to be represented and checked.

## Creating A Two-Dimensional Array

Both implementations store the grid in an `Array2<f64>`:

```rust
let grid = Array2::<f64>::zeros((grid_size, grid_size));
```

`Array2<T>` is a two-dimensional specialization of `ndarray::Array`. The shape
is a tuple, and `zeros` allocates owned, contiguous storage initialized with
zero values.

The dimensions are available without storing them separately:

```rust
let (rows, cols) = self.grid.dim();
```

An individual element is indexed with an array-shaped index:

```rust
let temperature = self.grid[[row, col]];
self.grid[[row, col]] = 100.0;
```

Unlike a nested `Vec<Vec<f64>>`, an `Array2` has one rectangular shape and a
defined memory layout.

## Start With Explicit Loops

The naive version makes the stencil mechanics visible:

```rust
for row in 1..rows - 1 {
    for col in 1..cols - 1 {
        new_grid[[row, col]] = self.grid[[row, col]]
            + self.alpha
                * dt
                * (self.grid[[row + 1, col]]
                    + self.grid[[row - 1, col]]
                    + self.grid[[row, col + 1]]
                    + self.grid[[row, col - 1]]
                    - 4.0 * self.grid[[row, col]]);
    }
}
```

This is a useful baseline: the loops describe the mathematical update
directly, and a hand-calculated unit test establishes a small known result
before refactoring.

The implementation is in
`source-code/heat-diffusion/naive/src/heat_diffusion.rs`.

## Selecting Regions With Slices

The `s!` macro constructs an array slice description. A shared slice borrows
data for reading:

```rust
let center = self.grid.slice(s![1..rows - 1, 1..cols - 1]);
```

A mutable slice borrows a region for writing:

```rust
self.grid
    .slice_mut(s![0, ..])
    .fill(boundary_temperature);
```

The first expression selects the interior. The second selects the complete top
row and fills it. Similar calls set the other three boundaries.

These operations create views; they do not copy the selected values. The
borrow remains active for as long as the view is used.

## A Circular Initial Condition

The central square that can contain the hot spot is selected once:

```rust
let mut spot = self.grid.slice_mut(s![
    center_row - spot_radius..center_row + spot_radius + 1,
    center_col - spot_radius..center_col + spot_radius + 1
]);
```

The local indices from `indexed_iter_mut` are then used to decide whether an
element lies inside the circle:

```rust
for ((row, col), temperature) in spot.indexed_iter_mut() {
    let row_distance = row.abs_diff(spot_radius);
    let col_distance = col.abs_diff(spot_radius);

    if row_distance.pow(2) + col_distance.pow(2) <= spot_radius.pow(2) {
        *temperature = spot_temperature;
    }
}
```

This combines three ideas:

- the slice limits the iteration to the relevant square;
- `indexed_iter_mut` supplies local coordinates and mutable element
  references;
- the condition fills only points inside the circular region.

## Shifted Views Describe The Stencil

The refactored step creates five equally shaped, read-only views:

```rust
let center = self.grid.slice(s![1..rows - 1, 1..cols - 1]);
let up = self.grid.slice(s![0..rows - 2, 1..cols - 1]);
let down = self.grid.slice(s![2..rows, 1..cols - 1]);
let left = self.grid.slice(s![1..rows - 1, 0..cols - 2]);
let right = self.grid.slice(s![1..rows - 1, 2..cols]);
```

Every position in these views refers to one center point and its four
neighbors. Explicit dimension-based bounds also make the selected shapes easy
to compare.

## Combining Views With Zip

`ndarray::Zip` walks arrays or views with matching shapes in lockstep:

```rust
Zip::from(self.next_grid.slice_mut(s![1..rows - 1, 1..cols - 1]))
    .and(center)
    .and(up)
    .and(down)
    .and(left)
    .and(right)
    .for_each(|new, &c, &u, &d, &l, &r| {
        let new_value = c + diffusion_factor * (u + d + l + r - 4.0 * c);
        max_change = max_change.max((new_value - c).abs());
        *new = new_value;
    });
```

The closure receives one mutable output and five input values. The maximum
change is accumulated while each new value is computed, avoiding a second
full-grid traversal.

This formulation emphasizes aligned array regions rather than row and column
bookkeeping. The explicit-loop version remains valuable when the mapping
between views would be less clear than the indices.

The complete refactored implementation is in
`source-code/heat-diffusion/ndarray-features/src/heat_diffusion.rs`.

## Double Buffering

A stencil update must read the complete old state while writing the new state.
Updating one grid in place would let later points observe already updated
neighbors.

The refactored `System` therefore owns two arrays:

```rust
pub struct System {
    grid: Array2<f64>,
    next_grid: Array2<f64>,
    alpha: f64,
}
```

The new interior is written into `next_grid`. The fixed boundaries were
initialized in both buffers. At the end of the step, ownership of the two
arrays is exchanged:

```rust
std::mem::swap(&mut self.grid, &mut self.next_grid);
```

This reuses both allocations for every time step.

## Returning A Read-Only View

The `ndarray-features` getter returns a borrowed view:

```rust
pub fn get_grid(&self) -> ArrayView2<'_, f64> {
    self.grid.view()
}
```

`ArrayView2` contains shape and stride information while borrowing the
elements. The `'_` lifetime says that the returned view cannot outlive the
borrow of `self`.

Callers can index and iterate over the result, but cannot mutate it. The naive
implementation returns `&Array2<f64>` instead; existing callers can use both
forms similarly for read-only indexing and iteration.

## Unit Tests As Refactoring Guardrails

Both implementations test numerical properties rather than only checking that
the program runs:

- the hot spot is circular and the boundaries are fixed;
- one small update matches a hand calculation;
- a symmetric initial state remains symmetric after a step;
- convergence and maximum-step behavior are correct;
- invalid physical and numerical parameters are rejected.

The hand-calculated test is deliberately small. With a 5-by-5 grid, a central
temperature of 100, `alpha = 0.1`, and `dt = 0.5`, the center becomes 80 after
one step and the maximum change is 20.

Run each project's tests independently:

```bash
cargo test --manifest-path source-code/heat-diffusion/naive/Cargo.toml
cargo test --manifest-path source-code/heat-diffusion/ndarray-features/Cargo.toml
```

## Cross-Implementation Testing

Unit tests protect each implementation, but the refactoring also has a
cross-project behavioral contract. The shell script runs both binaries with
matching arguments and compares their complete output:

```bash
cd source-code/heat-diffusion
./test_consistency.sh
```

It covers the initial state, one step, multiple steps, and convergence. When
the outputs differ, `diff` shows the mismatch.

This black-box check is complementary to unit tests:

- unit tests localize errors and check scientific invariants;
- the shell test verifies that the two independently built command-line
  programs remain interchangeable for the chosen cases.

## Run A Visible Example

From the repository root:

```bash
cargo run --manifest-path source-code/heat-diffusion/naive/Cargo.toml -- \
    --grid-size 21 --spot-radius 5 --steps 25 --show

cargo run --manifest-path source-code/heat-diffusion/ndarray-features/Cargo.toml -- \
    --grid-size 21 --spot-radius 5 --steps 25 --show
```

Try changing `--steps`, `--alpha`, and `--dt`. An unstable combination such as
`--alpha 1.0 --dt 0.3` should be rejected before the simulation starts.

## Memory Layout And Access Order

An `Array2` uses row-major layout by default, so elements in the same row are
normally adjacent in memory. Traversing rows and then columns follows that
layout:

```rust
for row in grid.rows() {
    for &value in row {
        // use value
    }
}
```

Views can have different strides and are not necessarily contiguous. Code that
needs a contiguous slice must check rather than assume that every view exposes
one.

The heat-diffusion example uses views to express rectangular regions. It does
not benchmark the loop and `Zip` versions, so do not infer a performance result
from their syntax alone. Measure representative release builds when
performance matters.

## Choosing A Scientific Array Crate

The Rust ecosystem has several relevant choices:

- use `ndarray` for NumPy-like multidimensional arrays, slicing, views,
  element-wise operations, and scientific data pipelines;
- use `nalgebra` for vectors, matrices, transformations, and
  dimension-aware linear algebra;
- consider `faer` for dense linear algebra with a strong focus on performance;
- use BLAS/LAPACK-backed crates when established native libraries and their
  deployment requirements fit the project.

This example focuses on array shapes, indexing, slicing, views, and stencils.
Broadcasting and matrix factorizations are separate topics and should be
introduced with examples where those operations are central.

## Hands-On Refactoring

1. Add a unit test to one implementation before changing it.
2. Replace one explicit boundary loop with a mutable slice and `fill`.
3. Compare the circular initialization in both implementations.
4. Draw the five shifted stencil regions for a 5-by-5 grid.
5. Change the convergence tolerance and predict the reported step count.
6. Run `./test_consistency.sh` after every behavior-preserving change.

## Summary

- `Array2` provides owned rectangular two-dimensional storage.
- Slices and views borrow array regions without copying them.
- Mutable views constrain where an operation may write.
- `Zip` applies one closure to aligned elements from several views.
- Double buffering preserves stencil semantics while reusing allocations.
- Read-only `ArrayView2` values expose borrowed array data.
- Unit tests and cross-project output comparison support numerical
  refactoring.
