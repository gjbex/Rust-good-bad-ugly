# Integrated Numerical Example: Julia Set

This module uses the Julia set implementations as an integrated numerical
example. Earlier modules introduced individual Rust concepts in isolation:
numeric types, functions, control flow, ownership, structs, traits, iterators,
error handling, project organization, and reproducibility. The Julia set
examples combine many of those ideas in small but realistic programs.

The goal is not to teach fractals in depth. The goal is to study how a
numerical algorithm is represented as Rust code, and how different
implementations make different design choices.

## Learning Objectives

After completing this module, participants should be able to:

- Explain the main steps of the Julia set computation.
- Use complex numbers through `num-complex`.
- Map a two-dimensional pixel grid to points in the complex plane.
- Store numerical output in a matrix-like data structure.
- Compare a custom matrix type with an external array crate.
- Separate scalar iteration from matrix-level iteration.
- Use command-line parameters for numerical runs.
- Use a TOML configuration file for reproducible run parameters.
- Pipe numerical output into a visualization tool.
- Compare implementation variants of the same algorithm.

## Prerequisites

Participants should already be comfortable with:

- Complex numbers and floating-point arithmetic.
- Functions and loops.
- Structs and methods.
- Generic matrix-like storage.
- Command-line parsing with `clap`.
- Error handling with `Result` and `?`.
- Running release builds with Cargo.

The example group used in this module is:

- `source-code/julia-set`

## The Example Family

The `source-code/julia-set` directory contains several implementations:

- `julia-set-baseline`: custom matrix type and explicit nested loops.
- `julia-set-mdarray`: `mdarray` storage with explicit nested loops.
- `julia-set-mdarray-expr-eval`: `mdarray` expression evaluation.
- `julia-set-toml-config`: custom matrix type with run parameters read from
  TOML.
- `view-fractal.py`: Plotly visualization helper.

All variants compute the same kind of output: for each point in a grid, count
how many iterations are needed before the complex recurrence escapes.

## Mathematical Core

The Julia set iteration is based on the recurrence:

```text
z <- z * z + c
```

Here, `z` and `c` are complex numbers. The parameter `c` is fixed for one run,
while the initial value of `z` varies over a grid in the complex plane.

The scalar iteration is implemented as:

```rust
fn iterate_z_value(z: Complex64, c: Complex64, max_iterations: usize) -> usize {
    let mut z_n = z;
    for n in 0..max_iterations {
        if z_n.norm() > 2.0 {
            return n;
        }
        z_n = z_n * z_n + c;
    }
    max_iterations
}
```

This function is a good example of keeping the scalar numerical kernel small:

- it receives one complex starting value;
- it receives the fixed complex parameter `c`;
- it receives the maximum number of iterations;
- it returns the escape iteration count.

## Complex Numbers

Rust's standard library does not provide a complex number type. The examples
use `num-complex`:

```rust
use num_complex::Complex64;
```

The fixed parameter is created from real and imaginary parts:

```rust
let c = Complex64::new(args.c_real, args.c_imag);
```

Complex arithmetic then uses ordinary operators:

```rust
z_n = z_n * z_n + c;
```

The escape condition uses the complex norm:

```rust
if z_n.norm() > 2.0 {
    return n;
}
```

This illustrates a common Rust pattern for scientific computing: use a crate
for domain-specific numeric types that are not part of the standard library.

## Mapping A Grid To The Complex Plane

The grid is defined by a number of rows and columns. Each grid position is
mapped to a point in the square domain from `-2.0` to `2.0` in both directions:

```rust
fn initialize_z(rows: usize, cols: usize) -> Matrix<Complex64> {
    let mut z = Matrix::new(rows, cols, Complex64::new(0.0, 0.0));
    let domain_min = -2.0;
    let domain_max = 2.0;
    let delta_re = (domain_max - domain_min) / (cols as f64);
    let delta_im = (domain_max - domain_min) / (rows as f64);

    for i in 0..rows {
        for j in 0..cols {
            let z_value = Complex64::new(
                domain_min + j as f64 * delta_re,
                domain_min + i as f64 * delta_im,
            );
            z.set(i, j, z_value)
                .expect("loop indices should be in bounds");
        }
    }
    z
}
```

This combines several earlier topics:

- half-open ranges with `0..rows` and `0..cols`;
- explicit integer-to-floating-point conversion with `as f64`;
- nested loops over a rectangular grid;
- complex-number construction;
- mutable matrix updates.

## Matrix-Level Iteration

Once the initial complex grid has been created, the program computes an
iteration count for every element:

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

The input matrix is borrowed:

```rust
z: &Matrix<Complex64>
```

The result matrix is owned and returned:

```rust
Matrix<usize>
```

This is a useful ownership pattern for numerical code: read from borrowed input
data, construct new output data, and return ownership of the result.

## Baseline Implementation

The baseline implementation uses the custom matrix type introduced in earlier
modules:

```text
source-code/julia-set/julia-set-baseline
```

Run it with:

```bash
cd source-code/julia-set/julia-set-baseline
cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

The baseline is valuable because it is explicit:

- grid initialization is written with nested loops;
- matrix access goes through `get` and `set`;
- the scalar iteration function is separate from the matrix iteration function;
- command-line options configure the run.

This implementation is a good reference point for comparing later variants.

## External Array Storage With `mdarray`

The `julia-set-mdarray` implementation replaces the custom matrix with the
`mdarray` crate:

```text
source-code/julia-set/julia-set-mdarray
```

It defines type aliases:

```rust
type MatrixC = DArray<Complex64, 2>;
type MatrixCSlice = DSlice<Complex64, 2>;
type MatrixI = DArray<usize, 2>;
```

The array is initialized with:

```rust
let mut z = MatrixC::from_elem([rows, cols], Complex64::new(0.0, 0.0));
```

Indexing uses array syntax:

```rust
z[[i, j]] = z_value;
let z_value = z[[i, j]];
```

This version illustrates a common transition in scientific code: start with a
small custom data structure to learn the mechanics, then use a dedicated array
crate when the problem becomes more realistic.

## Expression Evaluation Variant

The `julia-set-mdarray-expr-eval` implementation keeps `mdarray` but uses
expression evaluation for some matrix operations:

```text
source-code/julia-set/julia-set-mdarray-expr-eval
```

Grid initialization is expressed as a function over indices:

```rust
expr::from_fn([rows, cols], |idx| {
    let i = idx[0];
    let j = idx[1];

    Complex64::new(
        domain_min + j as f64 * delta_re,
        domain_min + i as f64 * delta_im,
    )
})
.eval()
```

The matrix iteration also becomes more expression-oriented:

```rust
z.expr()
    .map(|&z_value| iterate_z_value(z_value, c, max_iterations))
    .eval()
```

This version is useful for comparison. It is shorter and more declarative, but
it also relies more heavily on library abstractions. Learners can compare it
with the explicit-loop implementation and discuss readability, control, and
performance.

## TOML Configuration Variant

The `julia-set-toml-config` implementation reads run parameters from a TOML
file instead of taking each parameter separately from the command line:

```text
source-code/julia-set/julia-set-toml-config
```

The command line selects the configuration file:

```bash
cd source-code/julia-set/julia-set-toml-config
cargo run --release -- julia-set.toml | ../view-fractal.py
```

The configuration structure is:

```rust
#[derive(Debug, Deserialize)]
struct Config {
    max_iterations: usize,
    width: usize,
    height: usize,
    c_real: f64,
    c_imag: f64,
}
```

The file is read and deserialized with:

```rust
fn read_config(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    let config_text = fs::read_to_string(path)?;
    let config = toml::from_str(&config_text)?;
    Ok(config)
}
```

This combines several earlier topics:

- `clap` for command-line parsing;
- `serde` for deserialization;
- `toml` for configuration files;
- fallible I/O with `Result`;
- `?` for error propagation.

Configuration files are useful when run parameters should be saved, edited,
shared, or reused.

## Text Output And Visualization

The Rust programs print the iteration-count matrix as text. The visualization
script reads that output and displays a heatmap:

```bash
cargo run --release -- --width 800 --height 600 | ../view-fractal.py
```

The Python helper can also read from a file:

```bash
./view-fractal.py julia-set-baseline/julia-set.txt
```

The script uses Plotly:

```python
go.Heatmap(
    z=data,
    colorscale="Viridis",
    colorbar={"title": "Iterations"},
)
```

This division of labor is pragmatic: Rust computes the numerical data, and
Python provides a lightweight visualization workflow.

## Comparing Implementations

Because the directory contains several variants of the same algorithm, it is a
good place to discuss design tradeoffs:

- Custom matrix type versus external array crate.
- Explicit loops versus expression-style array operations.
- Command-line parameters versus configuration files.
- Minimal dependencies versus domain-specific crates.
- Code that exposes every step versus code that delegates more to libraries.

No single variant is automatically best. The useful question is what each
variant makes easier to read, test, modify, or reuse.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run the baseline implementation and visualize the result:

   ```bash
   cd source-code/julia-set/julia-set-baseline
   cargo run --release -- --width 400 --height 300 | ../view-fractal.py
   ```

2. Change `c_real` and `c_imag` on the command line and compare the image:

   ```bash
   cargo run --release -- --width 400 --height 300 --c-real -0.8 --c-imag 0.156 | ../view-fractal.py
   ```

3. Inspect `initialize_z` and identify where integer indices are converted to
   floating-point coordinates.

4. Inspect `iterate_z_value` and explain why it returns early.

5. Run the `mdarray` implementation with the same parameters and compare the
   output visually.

6. Compare the explicit-loop `mdarray` implementation with the expression
   evaluation implementation.

7. Run the TOML configuration implementation:

   ```bash
   cd ../julia-set-toml-config
   cargo run --release -- julia-set.toml | ../view-fractal.py
   ```

8. Edit `julia-set.toml` to change the image size or complex parameter, then
   rerun the program.

9. Add one extra field to the TOML configuration, such as a different domain
   limit, and discuss which functions would need to change.

10. Compare the command used for a CLI-parameter run with the TOML-based run.
    Discuss which is easier to reproduce later.

## Discussion Points

This module is a good place to emphasize:

- Integrated examples reveal how small language features combine.
- Numerical kernels are easier to reason about when scalar and matrix-level
  logic are separated.
- A custom type is useful for teaching, but established crates are usually
  better for serious array work.
- Configuration files are a practical tool for reproducible numerical runs.
- Text output and shell pipelines are simple interfaces between tools.
- Comparing multiple implementations is often more instructive than studying
  one implementation in isolation.

## Connection To The N-Body Example

The Julia set example prepares participants for the N-body integrated example:

- both examples use command-line configuration;
- both map numerical concepts to Rust structs, functions, and loops;
- both produce output that is visualized with Python;
- both benefit from separating numerical kernels from I/O and setup code.

The Julia set is deterministic and compact, making it a good integrated example
before moving on to the larger simulation code.
