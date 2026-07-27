# Heat-Diffusion TODO

## P1: Keep The Implementations Scientifically Consistent

- [x] Use the same initial-condition geometry in the baseline and
  `ndarray-features` variants.
  - Both variants create a circular hot spot.
  - Both variants independently test the expected initial grid and fixed
    boundaries.

- [ ] Validate physical and numerical parameters.
  - Reject grids that are too small for the requested initial region.
  - Require positive time steps and tolerances and non-negative thermal
    diffusivity.
  - Check the explicit two-dimensional five-point stencil stability condition.
    With unit grid spacing, require approximately `alpha * dt <= 0.25`.
  - Return useful errors instead of allowing indexing failures or unstable
    simulations.

## P2: Improve The `ndarray` Implementation

- [ ] Replace the per-step grid clone with double buffering.
  - Store current and next grids in `System`.
  - Initialize their boundaries consistently.
  - Write the new interior into the next grid and use `std::mem::swap` after
    each step.
  - Avoid allocating a new array during every simulation step.

- [ ] Calculate the maximum change during the stencil update.
  - Accumulate `max_change` inside the `Zip::for_each` closure.
  - Remove the additional full-grid traversal that compares the two arrays.

- [x] Replace negative slice endpoints with dimension-based bounds.
  - Expressions such as `s![1..-1, 1..-1]` are valid `ndarray` syntax but
    trigger Clippy's `reversed_empty_ranges` lint.
  - Obtain `(rows, cols)` from the grid and use bounds such as
    `s![1..rows - 1, 1..cols - 1]`.
  - Confirm that `cargo clippy --all-targets -- -D warnings` passes.

- [ ] Expose the computed grid as a read-only view.
  - Add an accessor returning `ndarray::ArrayView2<'_, f64>`.
  - Use the view in tests or output code so array views and their lifetimes have
    a concrete teaching purpose.

## P3: Tests And CLI Polish

- [ ] Complete the focused tests for both implementations.
  - Existing tests cover fixed boundaries, initial-region geometry, a
    hand-calculated step, invalid radii, convergence, and the maximum step
    count.
  - Add a test that a symmetric initial condition remains symmetric after one
    step.
  - Compare the baseline and `ndarray-features` results for identical inputs
    after the packages have been restructured for cross-package testing.

- [ ] Modernize the Clap derive attributes.
  - Prefer `#[arg(...)]` to `#[clap(...)]`.
  - Prefer typed defaults such as `default_value_t = 100`.
  - Keep all short option names unique.

## Validation

Run the following commands in both Cargo projects:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
```

Run matching small simulations in the baseline and `ndarray-features`
directories and compare their output after the initial conditions have been
made equivalent.
