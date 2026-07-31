# Multidimensional Arrays And Stencil Computation

Scientific programs often represent fields, images, and simulation states as
multidimensional arrays. This module introduces the `ndarray` crate through a
two-dimensional heat-diffusion example and then uses tests to guide a
behavior-preserving refactoring. A companion singular value decomposition
example introduces an OpenBLAS-backed linear-algebra workflow.

The complete examples are in `source-code/heat-diffusion` and
`source-code/svd`.

## Learning Objectives

By the end of this module, you should be able to:

- create a two-dimensional `Array2` and inspect its dimensions;
- index individual elements and iterate with their indices;
- select borrowed array regions with `slice` and `slice_mut`;
- combine equally shaped views with `Zip`;
- update a stencil computation without allocating a new array each step;
- expose array data through a read-only `ArrayView2`;
- broadcast one-dimensional coordinate arrays into a two-dimensional field;
- deserialize and validate structured TOML run configuration;
- reconstruct a matrix from its singular value decomposition using `dot`;
- compare numerical arrays using maximum absolute and Frobenius errors;
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

The directory contains three separate Cargo projects:

- `source-code/heat-diffusion/naive` expresses the stencil with explicit
  nested loops and element indexing;
- `source-code/heat-diffusion/ndarray-features` preserves the command-line
  behavior while using more of the `ndarray` API.
- `source-code/heat-diffusion/configurable` builds on the ndarray version,
  moves scientific parameters into TOML files, and adds a Gaussian initial
  condition through broadcasting.

The first two variants deliberately have the same inputs, validation, initial
condition, update equation, convergence rule, and output. The third can
reproduce that uniform-disk behavior from a configuration file or select a
new Gaussian profile.

Start by running the tests:

```bash
cd source-code/heat-diffusion
cargo test --manifest-path naive/Cargo.toml
cargo test --manifest-path ndarray-features/Cargo.toml
cargo test --manifest-path configurable/Cargo.toml
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

The boundary temperatures remain fixed. The first two implementations start
the interior at zero except for a uniform circular hot spot in the center. The
configurable version also supports a Gaussian hot spot over a configurable
background.

For this scheme, the stability check is:

```text
alpha * dt <= 0.25
```

The code validates this condition before running. Rust's memory safety does not
make an unstable numerical method stable; scientific constraints still need
to be represented and checked.

## Creating A Two-Dimensional Array

All three implementations store the grid in an `Array2<f64>`:

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

## When Command-Line Parameters Stop Scaling

The original command line exposes every numerical parameter separately. This
is convenient for a small demonstration, but it becomes awkward when a run
has related groups of parameters or mutually exclusive model choices.

The configurable implementation reduces the operational command line to:

```rust
#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    config: PathBuf,

    #[arg(long)]
    show: bool,
}
```

Scientific parameters live in the configuration file. The `--show` flag stays
on the command line because it controls presentation rather than the
simulation definition.

This distinction matters more than the raw number of arguments:

- a checked-in file gives a run a stable, reviewable identity;
- nested tables keep related values together;
- alternative model choices can require different fields;
- typographical errors can be rejected during deserialization.

## Structured TOML Configuration

The configuration mirrors the domain structure:

```toml
[grid]
size = 21
background_temperature = 0.0
boundary_temperature = 20.0

[material]
thermal_diffusivity = 0.1

[solver]
time_step = 0.01
max_steps = 25
tolerance = 1.0e-12
```

The corresponding Rust types derive `serde::Deserialize`. Each configuration
struct uses `#[serde(deny_unknown_fields)]`, so a misspelled field is an error
instead of a silently ignored parameter.

The complete configuration code is in
`source-code/heat-diffusion/configurable/src/config.rs`.

## Tagged Enums For Alternative Inputs

The initial-condition table has a `type` field:

```toml
[initial_condition]
type = "uniform-disk"
temperature = 100.0
radius = 5
```

Serde maps it to a tagged enum:

```rust
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum InitialCondition {
    UniformDisk {
        temperature: f64,
        radius: usize,
    },
    Gaussian {
        peak_temperature: f64,
        sigma: f64,
    },
}
```

Each variant carries only the parameters it needs. A uniform disk cannot
accidentally receive `sigma`, and a Gaussian cannot omit it.

Deserialization establishes the structure. A separate validation pass checks
scientific constraints such as finite temperatures, positive Gaussian width,
spot fit, positive tolerance, and the stencil stability condition.

## Gaussian Initialization With Broadcasting

For a Gaussian centered at `(row_center, col_center)`, the initial field is:

```text
temperature = background
    + (peak - background) * exp(-(row_distance^2 + col_distance^2)
                                / (2 * sigma^2))
```

The implementation creates a column of squared row distances with shape
`(rows, 1)` and a row of squared column distances with shape `(1, cols)`:

```rust
let row_squared = Array1::from_iter(
    (0..rows).map(|row| (row as f64 - center_row as f64).powi(2)),
)
.insert_axis(Axis(1));

let col_squared = Array1::from_iter(
    (0..cols).map(|col| (col as f64 - center_col as f64).powi(2)),
)
.insert_axis(Axis(0));

let radius_squared = &row_squared + &col_squared;
```

The addition broadcasts the singleton axes and produces a `(rows, cols)`
array. This expresses the separable coordinate construction without nested
indexing loops. The Gaussian formula is then applied with `mapv`, after which
the fixed boundary values overwrite the outer rows and columns.

The implementation is in
`source-code/heat-diffusion/configurable/src/heat_diffusion.rs`.

## Unit Tests As Refactoring Guardrails

The implementations test numerical properties rather than only checking that
the program runs:

- the hot spot is circular and the boundaries are fixed;
- one small update matches a hand calculation;
- a symmetric initial state remains symmetric after a step;
- convergence and maximum-step behavior are correct;
- the Gaussian has the configured peak, symmetry, and radial decay;
- both checked-in TOML files deserialize and validate;
- unknown configuration fields are rejected;
- invalid physical and numerical parameters are rejected.

The hand-calculated test is deliberately small. With a 5-by-5 grid, a central
temperature of 100, `alpha = 0.1`, and `dt = 0.5`, the center becomes 80 after
one step and the maximum change is 20.

Run each project's tests independently:

```bash
cargo test --manifest-path source-code/heat-diffusion/naive/Cargo.toml
cargo test --manifest-path source-code/heat-diffusion/ndarray-features/Cargo.toml
cargo test --manifest-path source-code/heat-diffusion/configurable/Cargo.toml
```

## Cross-Implementation Testing

Unit tests protect each implementation, but the refactoring also has a
cross-project behavioral contract. The shell script runs both binaries with
matching arguments and compares their complete output:

```bash
cd source-code/heat-diffusion
./test_consistency.sh
```

It covers the initial state, one step, multiple steps, and convergence for the
first two implementations. It also compares the uniform TOML configuration
with equivalent command-line arguments for the ndarray implementation. When
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

cargo run --manifest-path source-code/heat-diffusion/configurable/Cargo.toml -- \
    --config source-code/heat-diffusion/configurable/configs/gaussian-spot.toml \
    --show
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

This example focuses on array shapes, indexing, slicing, views, broadcasting,
and stencils. The companion `source-code/svd` project uses `ndarray-linalg`
because the matrix factorization, rather than a stencil, is its central
operation.

## Reconstructing A Matrix From Its SVD

For an `m`-by-`n` matrix, a compact singular value decomposition is:

```text
A = U Sigma V^T
```

With `k = min(m, n)`, the shapes are:

```text
A:       (m, n)
U:       (m, k)
Sigma:   (k, k)
V^T:     (k, n)
```

The `source-code/svd` example uses `ndarray-linalg` with a system OpenBLAS
backend. On Debian or Ubuntu, install its development package before building:

```bash
sudo apt install libopenblas-dev
```

The backend choice is explicit in `source-code/svd/Cargo.toml`:

```toml
ndarray = "0.17.2"
ndarray-linalg = { version = "0.18.1", features = ["openblas-system"] }
```

Importing `SVD` brings the factorization method into scope as an extension
trait. The singular values form the diagonal of `Sigma`, and `dot` performs
matrix multiplication:

```rust
use ndarray::{Array2, s};
use ndarray_linalg::SVD;

let (u, singular_values, vt) = matrix.svd(true, true)?;
let u = u.expect("left singular vectors were requested");
let vt = vt.expect("right singular vectors were requested");

let k = singular_values.len();
let sigma = Array2::from_diag(&singular_values);
let reconstructed = u
    .slice(s![.., ..k])
    .dot(&sigma)
    .dot(&vt.slice(s![..k, ..]));
```

The slices make the compact dimensions explicit even when a backend returns
larger factor matrices.

## Comparing The Reconstruction

Floating-point factorizations should not be checked with exact equality. The
example computes the element-wise difference and summarizes it in two ways:

```rust
let difference = &reconstructed - &original;
let max_absolute_error = difference
    .iter()
    .fold(0.0_f64, |largest, &value| largest.max(value.abs()));
let frobenius_error = difference.mapv(|value| value * value).sum().sqrt();
```

The maximum absolute error reports the largest element-wise discrepancy. The
Frobenius error is the square root of the sum of squared discrepancies over the
whole matrix.

Run the example and its test:

```bash
cd source-code/svd
cargo run
cargo test
```

Both errors should be close to machine precision. Their exact final digits can
depend on the linear-algebra backend, so the test checks a tolerance rather
than a printed reference value.

## Extended Example: Persisting Arrays With HDF5

The preceding examples keep arrays in memory or print small fields to the
terminal. Scientific applications usually need a richer persistent format
that preserves array shapes and metadata and supports reading subsets of a
large dataset. The optional `source-code/hdf5-snapshot` example illustrates
that transition with HDF5.

The Cargo dependency renames the maintained `hdf5-metno` package to the short
crate name used in the source:

```toml
hdf5 = { package = "hdf5-metno", version = "0.14.0" }
ndarray = "0.17.2"
```

The example creates a Gaussian temperature field and writes this logical
structure:

```text
/snapshot
  @description
  @time_seconds
  @step
  x                 one-dimensional coordinates, units = "m"
  y                 one-dimensional coordinates, units = "m"
  temperature       two-dimensional field, units and valid range
```

HDF5 datasets retain their element type and shape. Attributes attach context
to a group or dataset without encoding it into a filename or a separate text
file. The temperature dataset is chunked and compressed:

```rust
let (rows, columns) = snapshot.temperature.dim();
let temperature_dataset = group
    .new_dataset_builder()
    .chunk((rows.min(16), columns.min(16)))
    .deflate(4)
    .with_data(snapshot.temperature.view())
    .create("temperature")?;
```

Chunking divides the logical array into independently stored blocks. It is
required for compression and affects the cost of reading array subsets, so a
chunk shape should reflect expected access patterns rather than being chosen
arbitrarily.

The reader demonstrates an HDF5 hyperslab by loading only the central 3-by-3
selection into an `Array2`:

```rust
let center = dataset.read_slice_2d(s![
    row_start..row_start + 3,
    column_start..column_start + 3
])?;
```

The minimum and maximum are computed while the field is still in memory and
stored as scalar attributes. The inspection path can therefore report the
range without rereading the complete field. Keeping such summary metadata
consistent with the dataset becomes part of the writer's responsibility.

Run the example and inspect the resulting file with language-independent HDF5
tools:

```bash
cd source-code/hdf5-snapshot
cargo run --release
h5ls -r temperature-snapshot.h5
h5dump -pH temperature-snapshot.h5
```

The high-level crate keeps native handles and raw pointers out of the
application code, but deployment still depends on a compatible HDF5 library.
This makes the example a useful bridge to Module 15: a safe Rust API does not
remove the need to understand system packages, library discovery, cluster
modules, containers, and CI environments.

## Hands-On Refactoring

1. Add a unit test to one implementation before changing it.
2. Replace one explicit boundary loop with a mutable slice and `fill`.
3. Compare the circular initialization in the first two implementations.
4. Draw the five shifted stencil regions for a 5-by-5 grid.
5. Change the convergence tolerance and predict the reported step count.
6. Run `./test_consistency.sh` after every behavior-preserving change.
7. Add a configuration field, then misspell it and inspect the parse error.
8. Compare the uniform-disk and Gaussian initial-condition tables.
9. Change `sigma` and predict how much of the grid the Gaussian occupies.
10. Draw the compact SVD shapes for the 4-by-3 matrix in `source-code/svd`.
11. Change one matrix element, then rerun the SVD reconstruction test.
12. Run `source-code/hdf5-snapshot` and identify its groups, datasets, and
    attributes with `h5ls` and `h5dump`.
13. Change the HDF5 chunk shape and explain which access pattern it favors.

## Summary

- `Array2` provides owned rectangular two-dimensional storage.
- Slices and views borrow array regions without copying them.
- Mutable views constrain where an operation may write.
- `Zip` applies one closure to aligned elements from several views.
- Broadcasting combines `(rows, 1)` and `(1, cols)` coordinate arrays.
- Double buffering preserves stencil semantics while reusing allocations.
- Read-only `ArrayView2` values expose borrowed array data.
- Tagged enums model configuration alternatives with different parameters.
- TOML files make scientific run definitions reviewable and reproducible.
- `ndarray-linalg` extends arrays with OpenBLAS-backed factorizations.
- A compact SVD reconstructs an array with two matrix multiplications.
- Numerical reconstruction checks require an explicit tolerance.
- HDF5 stores typed multidimensional datasets together with metadata.
- Chunking enables compression and efficient subset-oriented I/O.
- Unit tests and cross-project output comparison support numerical
  refactoring.
