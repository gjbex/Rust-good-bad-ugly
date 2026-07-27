# Heat-Diffusion TODO

Status verified on 2026-07-27. Both implementations compile; the naive
implementation passes eight unit tests and the `ndarray-features`
implementation passes nine. They produce identical output for matching
inputs.

## P1: Keep The Implementations Scientifically Consistent

- [x] Use the same initial-condition geometry in the `naive` and
  `ndarray-features` variants.
  - Both variants create a circular hot spot.
  - Both variants independently test the expected initial grid and fixed
    boundaries.

- [x] Validate physical and numerical parameters.
  - Reject grids that are too small for the requested initial region.
  - Require positive time steps and tolerances and non-negative thermal
    diffusivity.
  - Check the explicit two-dimensional five-point stencil stability condition.
    With unit grid spacing, require approximately `alpha * dt <= 0.25`.
  - Return useful errors instead of allowing indexing failures or unstable
    simulations.

## P2: Improve The `ndarray` Implementation

- [x] Replace the per-step grid clone with double buffering.
  - Store current and next grids in `System`.
  - Initialize their boundaries consistently.
  - Write the new interior into the next grid and use `std::mem::swap` after
    each step.
  - Avoid allocating a new array during every simulation step.

- [x] Calculate the maximum change during the stencil update.
  - Accumulate `max_change` inside the `Zip::for_each` closure.
  - Remove the additional full-grid traversal that compares the two arrays.

- [x] Replace negative slice endpoints with dimension-based bounds.
  - Expressions such as `s![1..-1, 1..-1]` are valid `ndarray` syntax but
    trigger Clippy's `reversed_empty_ranges` lint.
  - Obtain `(rows, cols)` from the grid and use bounds such as
    `s![1..rows - 1, 1..cols - 1]`.
  - Confirm that `cargo clippy --all-targets -- -D warnings` passes.

- [x] Expose the computed grid as a read-only view in `ndarray-features`.
  - Add a `get_grid` accessor returning `ndarray::ArrayView2<'_, f64>`.
  - Use the view in tests or output code so array views and their lifetimes have
    a concrete teaching purpose.
  - Leave the naive implementation's `&Array2<f64>` accessor unchanged so the
    two return types can be discussed during the refactoring exercise.

## P3: Tests And CLI Polish

- [x] Add focused unit tests for both implementations.
  - Tests cover fixed boundaries, initial-region geometry, a hand-calculated
    step, one-step symmetry, invalid parameters, convergence, and the maximum
    step count.

- [ ] Compare the `naive` and `ndarray-features` results for identical inputs
  after the packages have been restructured for cross-package testing.

- [ ] Modernize the Clap derive attributes.
  - Prefer `#[arg(...)]` to `#[clap(...)]`.
  - Prefer typed defaults such as `default_value_t = 100`.
  - Keep all short option names unique.

## P4: Integrate The Example Into The Training Repository

- [ ] Add `source-code/heat-diffusion/README.md`.
  - Explain the purpose of the two implementations and their teaching order.
  - Document matching commands for running and testing both variants.
  - State which behavior should remain equivalent during refactoring.

- [ ] Add the example to `source-code/README.md` and `FEATURE_MAP.md`.
  - Map `Array2`, slicing, mutable views, `Zip`, stencil computation, unit
    tests, and behavior-preserving refactoring.

- [ ] Add the example to the appropriate learning module and matching slide
  source when the multidimensional-array curriculum item is implemented.

## Validation

Verified on 2026-07-27 in both Cargo projects:

- [x] `cargo fmt --check`
- [x] `cargo check`
- [x] `cargo test` with eight passing naive tests and nine passing
  `ndarray-features` tests
- [x] `cargo clippy --all-targets -- -D warnings`
- [x] Matching 21-by-21 initial states with radius 5 and identical
  temperatures
- [x] Matching 21-by-21 states after 25 simulation steps

Rerun the following commands in both Cargo projects after each remaining
change:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

Run matching simulations in the `naive` and `ndarray-features` directories and
compare their output after any change to their numerical behavior.
