# Heat Diffusion

This example solves a two-dimensional heat-diffusion problem with an explicit
five-point stencil. It contains two Cargo projects that implement the same
command-line program:

- `naive/` uses explicit nested loops and indexed array access. Start here to
  see the algorithm directly.
- `ndarray-features/` preserves the behavior while introducing `ndarray`
  slices, mutable views, `Zip`, read-only views, and double buffering.

The teaching sequence is a behavior-preserving refactoring exercise. Read and
test the naive version first, then compare each part with the
`ndarray-features` version.

## Numerical Model

For each interior grid point, one time step computes

```text
new = center + alpha * dt * (up + down + left + right - 4 * center)
```

The examples use unit grid spacing. For this explicit two-dimensional scheme,
the command-line parameters must satisfy `alpha * dt <= 0.25`. The outer
boundary remains at the requested boundary temperature.

The initial condition is a circular hot spot centered in the grid. The spot
radius must keep the hot spot inside every boundary.

## Run Both Implementations

From this directory, run matching simulations with:

```bash
cargo run --manifest-path naive/Cargo.toml -- \
    --grid-size 21 --spot-radius 5 --steps 25 --show

cargo run --manifest-path ndarray-features/Cargo.toml -- \
    --grid-size 21 --spot-radius 5 --steps 25 --show
```

Use `--help` on either command to see all parameters. Both programs use the
same option names, defaults, validation rules, convergence criterion, status
line, and formatted grid output.

## Test The Implementations

Each Cargo project has unit tests for the initial condition, fixed boundaries,
a hand-calculated update, symmetry, convergence, and invalid parameters:

```bash
cargo test --manifest-path naive/Cargo.toml
cargo test --manifest-path ndarray-features/Cargo.toml
```

The shell test runs both binaries with identical inputs and compares their
complete output:

```bash
./test_consistency.sh
```

Run this test after each refactoring step. It checks the initial state, one
step, multiple steps, and a convergence case.

## Behavior That Must Remain Equivalent

The two implementations should agree on:

- the circular initial hot spot and fixed boundary values;
- parameter validation and stability checks;
- the five-point stencil update;
- the number of completed steps and convergence behavior;
- the grid printed by `--show`.

Their internal APIs intentionally differ. In particular, the naive
implementation returns `&Array2<f64>` from `get_grid`, while the refactored
implementation returns an `ArrayView2<'_, f64>`. The latter exposes a
read-only view without exposing the owned array type as the caller's value.

## Suggested Reading Order

1. Run the unit tests and `test_consistency.sh`.
2. Inspect the nested loops in `naive/src/heat_diffusion.rs`.
3. Compare the circular initialization and boundary assignments.
4. Follow the shifted array views through the `Zip` stencil update.
5. Identify how `grid` and `next_grid` are swapped without allocating each
   time step.
6. Change one implementation and use both levels of testing to check the
   result.
