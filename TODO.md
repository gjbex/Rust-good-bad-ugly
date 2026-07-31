# Outstanding Curriculum And Consistency Work

Baseline verification on 2026-07-29 confirmed that the complete training-site
build passed, inline `source-code/...` references pointed to existing paths,
and the generated learning modules and slides were consistent with their
sources. The Polars and HDF5 examples were merged after that verification, on
2026-07-31, so the complete cross-surface build should be rerun before this
baseline date is advanced.

The P1 teaching-surface fixes have also been verified: the `enum-match`
snippets match the working source, and the documented polynomial command
accepts its negative coefficient.

The heat-diffusion example is now integrated as Module 14. It includes
matching naive and `ndarray` implementations, scientific unit tests, a
cross-implementation black-box test, and teaching material on slices, views,
`Zip`, double buffering, and stencil computation.

The array material also includes broadcasting and a separate OpenBLAS-backed
SVD example that reconstructs a rectangular matrix with `dot` and checks
maximum absolute and Frobenius errors.

Native-library interoperability is now integrated as Module 15. Paired FFTW
examples compare a local safe wrapper over `fftw-sys` with the high-level
`fftw` crate. They run the same transform, validate frequency peaks and
inverse-transform error, and expose matching optional CLI paths that write a
Parseval-checked one-sided power spectrum as CSV. This makes the allocation,
ownership, and `unsafe` machinery removed by a safe crate directly visible.
The local wrapper also demonstrates library-quality error handling through a
public `FftError` enum, boundary validation, `Result`-based APIs, error
propagation with `?`, and tests for failure cases.

Two optional extended examples now broaden the scientific-data coverage
without adding new learning modules. `source-code/polars-data-analysis` uses a
lazy Polars query to join, filter, aggregate, and optionally write patient data
as Parquet. `source-code/hdf5-snapshot` stores an `ndarray` temperature field,
coordinates, units, run metadata, and valid ranges in a chunked and compressed
HDF5 file, then verifies the complete round trip and a hyperslab read.

The following work is still outstanding.

## P3: Curriculum Coverage

- [ ] Decide whether `source-code/smart-pointers` belongs in the curriculum.
  - The example is listed in `source-code/README.md` and mapped extensively in
    `FEATURE_MAP.md`, but it is not referenced by a learning module or slide.
  - If it is participant-facing, place it after the prerequisite ownership,
    generic-type, and trait material, then update the appropriate learning
    module, numbered slide file, and module overview.
  - If it is intentionally supplemental, label that status explicitly in the
    source index so the absence from the curriculum is deliberate.
  - Run its documented commands and tests after deciding:

    ```bash
    cd source-code/smart-pointers
    cargo run
    cargo run -- --num-elements 10 --seed 123
    cargo test
    ```

## High-Priority Curriculum Gaps

- [ ] Add a short Rust-specific bridge to numerical reliability.
  - Build on Module 2's introduction to `f32` and `f64` by covering
    floating-point failure modes, including `NaN`, ordering, and comparisons.
  - Contrast integer overflow in debug and release builds.
  - Introduce `checked_*`, `saturating_*`, and `wrapping_*` integer operations.
  - Show approximate numerical assertions and the effect of Rayon reduction
    order on reproducibility.
  - Make clear that Rust provides memory and type safety, not automatic
    numerical validity.
  - Leave conditioning, stability, accumulated rounding error, convergence,
    and general validation methodology to a separate language-agnostic
    numerical-reliability training.

- [ ] Add a practical performance-diagnosis workflow.
  - Build on the existing release-mode and Rayon benchmarks.
  - Examine allocations, avoidable cloning, data layout, cache behavior,
    bounds checks, and iterator-versus-loop assumptions.
  - Introduce system profiling with a tool such as `perf` or a flamegraph.
  - Use Criterion only where a focused microbenchmark is appropriate.

## Secondary Curriculum Gaps

- [ ] Add a home-grown C++ FFI companion to Module 15.
  - Create a small `source-code/cpp-interpolation-ffi` example around a
    one-dimensional C++ interpolator for which no `-sys` crate exists.
  - Put a stable `extern "C"` façade around the C++ class, represent the class
    as an opaque handle, and pass numerical arrays as pointer-length pairs.
  - Compile and link the native source from `build.rs` with the `cc` crate.
  - Write the small raw Rust declarations by hand in an `unsafe extern "C"`
    block, then hide them behind an owning safe Rust type that releases the
    native handle through `Drop`.
  - Convert C++ exceptions and validation failures to status codes or another
    C-compatible error contract; never allow exceptions to cross the FFI
    boundary.
  - Contrast the complete binding and build workflow with `fftw-sys`, where a
    `-sys` crate already supplies the raw declarations and system-library link
    selection.
  - Keep `bindgen` as an optional follow-up exercise because its `libclang`
    dependency can make the core live demonstration less portable.
  - Integrate the comparison into Module 15, its slides, the source index, and
    `FEATURE_MAP.md`, and test ownership cleanup and numerical behavior.

- [ ] Improve application-level error reporting in scientific file examples.
  - Add operation and path context to file I/O failures in an existing example
    such as `source-code/polars-data-analysis` or
    `source-code/hdf5-snapshot`.
  - Replace user-triggerable assertions in the FFTW applications with
    structured errors that produce concise diagnostics and nonzero exit
    statuses without panicking.
  - Keep the existing `FftError` as the example of a typed library error; do not
    create another custom error enum solely for feature coverage.

- [ ] Complete the research-data provenance and validation workflow.
  - Build on the existing HDF5 and Parquet examples rather than adding another
    format solely for coverage.
  - Add explicit schema versions and record the complete run configuration,
    applicable random seeds, and software or toolchain versions.
  - Validate these metadata fields when reading data.
  - Report unmatched or rejected Polars input records instead of silently
    excluding them.
  - Keep scientific results separate from validation reports and diagnostics.

- [ ] Extend project organization to a library-quality development workflow.
  - Add integration tests, documentation tests, `cargo doc`, `cargo fmt`,
    Clippy, and continuous integration.
  - Introduce Cargo workspaces and feature flags when optional or native
    dependencies provide a concrete motivation.

- [ ] Add parallel scientific patterns beyond independent element processing.
  - Cover parallel reductions, deterministic aggregation, thread-local random
    streams, and synchronization costs.
  - Present distributed-memory parallelism and MPI as an ecosystem direction,
    not necessarily as a core implementation module.

## Lower-Priority Topics

- [ ] Extend the `ArrayView2<'_, f64>` discussion in Module 14 to named
  lifetime parameters if an API requiring more than lifetime elision provides
  a natural example.
- [ ] Treat deeper pattern matching as an incremental addition to existing
  examples rather than as a separate module.
- [ ] Keep general async Rust out of the core curriculum unless the course
  expands into data acquisition, services, or concurrent I/O.
