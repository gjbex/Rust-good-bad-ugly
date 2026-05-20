# Data Parallelism With Rayon

This module introduces data parallelism in Rust using Rayon. The example is a
parallel Julia set implementation, chosen because each grid point can be
computed independently once the input parameters are known.

The goal is to show the basic Rayon workflow: identify independent work,
replace a sequential iterator with a parallel iterator, collect the results,
and control the number of worker threads when benchmarking or experimenting.

## Learning Objectives

After completing this module, participants should be able to:

- Explain what data parallelism means.
- Identify independent per-element work in a numerical algorithm.
- Add Rayon as a dependency.
- Import Rayon parallel iterator traits.
- Use `into_par_iter` to process a range in parallel.
- Use `map` and `collect` with a parallel iterator.
- Avoid shared mutable state in a parallel computation.
- Control Rayon worker threads with `RAYON_NUM_THREADS`.
- Compare serial and parallel implementations of the same algorithm.
- Recognize when parallel overhead can dominate small workloads.

## Prerequisites

Participants should already be comfortable with:

- Iterators and iterator adapters.
- Ownership and borrowing.
- Closures.
- Vectors and collection with `collect`.
- The baseline Julia set implementation.
- Running release builds with Cargo.

The example used in this module is:

- `source-code/julia-set/julia-set-rayon`

The serial reference implementation is:

- `source-code/julia-set/julia-set-baseline`

## Why The Julia Set Parallelizes Well

In the Julia set computation, each output grid point is computed independently.
For one grid point, the program starts from one complex value `z`, repeatedly
applies:

```text
z <- z * z + c
```

and records the number of iterations before escape.

The computation for one grid point does not need the result from any other grid
point. That independence is the main reason this is a good data-parallel
example.

## Adding Rayon

The Rayon implementation adds the crate in `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
num-complex = "0.4"
rayon = "1.10"
```

The program imports Rayon's prelude:

```rust
use rayon::prelude::*;
```

The prelude brings the parallel iterator traits into scope, including the trait
that provides `into_par_iter`.

## Serial Matrix Computation

The baseline computes the result matrix with nested loops:

```rust
fn iterate_z_matrix(z: &Matrix<Complex64>, c: Complex64, max_iterations: usize) -> Matrix<usize> {
    let mut result = Matrix::new(z.rows(), z.cols(), 0);
    for i in 0..z.rows() {
        for j in 0..z.cols() {
            let z_value = *z.get(i, j).expect("loop indices should be in bounds");
            let iterations = iterate_z_value(z_value, c, max_iterations);
            result.set(i, j, iterations)
                .expect("loop indices should be in bounds");
        }
    }
    result
}
```

This is clear and useful as a reference implementation, but it performs one
grid point after another on one thread.

## Parallel Matrix Computation

The Rayon version computes a flat vector of result values in parallel:

```rust
fn iterate_z_matrix(z: &Matrix<Complex64>, c: Complex64, max_iterations: usize) -> Matrix<usize> {
    let rows = z.rows();
    let cols = z.cols();
    let data: Vec<usize> = (0..rows * cols)
        .into_par_iter()
        .map(|index| {
            let row = index / cols;
            let col = index % cols;
            let z_value = *z.get(row, col).expect("flat index should be in bounds");
            iterate_z_value(z_value, c, max_iterations)
        })
        .collect();

    Matrix::from_vec(rows, cols, data).expect("parallel result should match matrix shape")
}
```

The range:

```rust
0..rows * cols
```

represents all flat matrix indices. Rayon splits that range across worker
threads.

Each worker maps an index to a row and column:

```rust
let row = index / cols;
let col = index % cols;
```

Then it computes one independent output value.

## Avoiding Shared Mutable State

A tempting parallel design would be to create one mutable result matrix and let
multiple threads write into it. That would introduce shared mutable state.

The Rayon version avoids that by having each parallel task produce one value:

```rust
.map(|index| {
    // compute one usize
})
```

The values are collected into a new vector:

```rust
let data: Vec<usize> = ...collect();
```

After the parallel computation is complete, the vector is wrapped in a matrix:

```rust
Matrix::from_vec(rows, cols, data)
```

This pattern is often the cleanest first approach to data parallelism: compute
independent output values and collect them, rather than coordinating concurrent
writes.

## Creating A Matrix From Computed Data

The Rayon variant adds a constructor to the simple matrix type:

```rust
pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Result<Self, String> {
    if data.len() == rows * cols {
        Ok(Self { rows, cols, data })
    } else {
        Err(format!(
            "matrix data has {} elements, but shape ({rows}, {cols}) requires {}",
            data.len(),
            rows * cols
        ))
    }
}
```

This constructor checks that the flat vector has the expected number of
elements. The parallel computation should produce exactly one value per grid
point, but the check keeps the matrix invariant explicit.

## Running The Parallel Version

Run the Rayon implementation with:

```bash
cd source-code/julia-set/julia-set-rayon
cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

It uses the same command-line arguments and output format as the baseline, so
the outputs can be compared directly.

## Controlling The Number Of Threads

Rayon uses a thread pool by default. The number of worker threads can be
controlled with `RAYON_NUM_THREADS`:

```bash
RAYON_NUM_THREADS=4 cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

This is useful when comparing runs on shared machines or when studying scaling.

For example:

```bash
RAYON_NUM_THREADS=1 cargo run --release -- --width 1200 --height 1200 > /dev/null
RAYON_NUM_THREADS=2 cargo run --release -- --width 1200 --height 1200 > /dev/null
RAYON_NUM_THREADS=4 cargo run --release -- --width 1200 --height 1200 > /dev/null
```

Thread counts should be interpreted together with the available CPU cores and
the size of the workload.

## Benchmarking

The Julia set directory includes a benchmark script that compares all Julia set
implementations:

```bash
cd source-code/julia-set
./benchmark.sh
```

The script builds all Julia set implementations in release mode, smoke-tests
them, and compares them with `hyperfine`.

For a quick experiment, reduce the number of runs:

```bash
WARMUP=1 RUNS=3 ./benchmark.sh
```

For a more meaningful Rayon comparison, use a larger image size:

```bash
WIDTH=1600 HEIGHT=1600 MAX_ITERATIONS=1000 WARMUP=1 RUNS=5 ./benchmark.sh
```

Very small workloads may make the parallel version look slower because thread
pool and scheduling overhead dominate the actual computation.

The Rayon implementation also has its own thread-scaling benchmark script:

```bash
cd source-code/julia-set/julia-set-rayon
THREAD_COUNTS="1 2 4 8" ./benchmark.sh
```

This script builds only the Rayon implementation and runs it repeatedly with
different `RAYON_NUM_THREADS` values.

## When Rayon Is A Good Fit

Rayon is a good fit when:

- work can be split into many independent tasks;
- each task is large enough to offset scheduling overhead;
- the computation avoids shared mutable state;
- the output can be collected or reduced safely;
- the algorithm is CPU-bound rather than I/O-bound.

The Julia set satisfies these conditions well for sufficiently large grids.

Rayon is less useful when:

- the workload is tiny;
- each task depends strongly on neighboring results;
- the main bottleneck is file I/O;
- synchronization would dominate the computation.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run the serial baseline:

   ```bash
   cd source-code/julia-set/julia-set-baseline
   cargo run --release -- --width 400 --height 300 > /tmp/julia-serial.txt
   ```

2. Run the Rayon version with the same parameters:

   ```bash
   cd ../julia-set-rayon
   cargo run --release -- --width 400 --height 300 > /tmp/julia-rayon.txt
   ```

3. Compare the outputs:

   ```bash
   diff /tmp/julia-serial.txt /tmp/julia-rayon.txt
   ```

4. Run the Rayon version with different thread counts:

   ```bash
   RAYON_NUM_THREADS=1 cargo run --release -- --width 1200 --height 1200 > /dev/null
   RAYON_NUM_THREADS=4 cargo run --release -- --width 1200 --height 1200 > /dev/null
   ```

5. Run the benchmark script from `source-code/julia-set`.

6. Run the Rayon-specific benchmark script with at least three thread counts.

7. Inspect `iterate_z_matrix` in the baseline and Rayon versions. Identify the
   exact point where serial iteration changes to parallel iteration.

8. Change the Rayon implementation to use a smaller image and observe whether
   parallelism still helps.

9. Discuss why collecting a vector of independent results is simpler than
   mutating a shared matrix from multiple threads.

## Discussion Points

This module is a good place to emphasize:

- Rayon provides data parallelism with a small change to iterator style.
- Parallel iterators preserve many familiar iterator concepts.
- Correct parallel code depends on independent work and clear ownership.
- Avoiding shared mutable state is often the simplest design.
- Parallel overhead matters, especially for small workloads.
- Benchmarks should use release builds and problem sizes large enough to be
  meaningful.

## Connection To Later Material

Rayon is a first step into Rust parallelism. It is higher-level than manual
threads and much easier to apply safely when the computation is data-parallel.

Later parallel topics could include:

- parallel reductions;
- parallel iteration over arrays from numerical crates;
- thread-local state and reproducible random streams;
- synchronization primitives;
- scoped threads;
- distributed-memory parallelism outside Rust's standard ecosystem.
