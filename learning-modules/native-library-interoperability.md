# Native Library Interoperability

Scientific Rust programs rarely start in isolation. They may need an in-house
C++ library with no Rust bindings, a mature C or Fortran library exposed by a
raw `-sys` crate, or an existing high-level Rust wrapper. This module follows
those three entry points in increasing order of available abstraction:

1. `source-code/cpp-interpolation-ffi` builds a complete binding for a
   home-grown C++ library;
2. `source-code/fftw-ffi` starts from the raw declarations and linking supplied
   by `fftw-sys` and constructs a safe wrapper;
3. `source-code/fftw-safe` performs the same FFTW workflow with the existing
   high-level `fftw` crate.

## Learning Objectives

By the end of this module, you should be able to:

- recognize a raw FFI layer and the obligations it transfers to the caller;
- expose a C++ class through a stable C ABI and opaque handle;
- compile included native sources from `build.rs`;
- translate native failures and boundary violations into Rust errors;
- represent native allocation and opaque handles with owning Rust types;
- release native resources through `Drop`;
- pass contiguous buffers to native functions through raw pointers;
- convert Rust lengths to C integer types with validation;
- concentrate `unsafe` operations behind a safe slice-based API;
- decide when to use an existing safe crate instead of maintaining a local FFI
  wrapper;
- expose optional output files through typed `clap` arguments;
- test the scientific behavior of an FFI wrapper with numerical tolerances.

## Prerequisites

This module builds on:

- ownership and borrowing from Module 4;
- structs and methods from Module 5;
- error handling from Module 8;
- library organization and tests from Module 9;
- numerical array validation from Module 14.

It belongs at the end of the course because FFI code must state requirements
that the Rust compiler cannot infer from a C API.

## Start With A Home-Grown C++ Library

A small in-house library is the clearest place to see the complete binding
stack because no crate supplies any layer in advance. The
`source-code/cpp-interpolation-ffi` example wraps a C++ linear interpolator in
four layers:

```text
safe Rust API       src/lib.rs
raw Rust bindings   src/raw.rs
stable C facade     native/interpolator_c.h and native/interpolator_c.cpp
C++ library         native/interpolator.hpp and native/interpolator.cpp
```

Rust does not call the C++ class directly. C++ symbol names and class layouts
are compiler-specific, and C++ exceptions must not unwind into Rust. The
adapter therefore presents an `extern "C"` interface with an opaque handle:

```c
typedef struct interpolation_handle interpolation_handle;

int interpolation_create(
    const double* coordinates,
    size_t coordinate_count,
    const double* values,
    size_t value_count,
    interpolation_handle** output
);

int interpolation_evaluate(
    const interpolation_handle* handle,
    double coordinate,
    double* output
);

void interpolation_destroy(interpolation_handle* handle);
```

The pointer-length pairs describe borrowed input arrays without committing the
C ABI to a Rust or C++ container layout. Construction copies the arrays into
the C++ object, so the object does not borrow the Rust slices after the call.
Every potentially throwing entry point catches exceptions and returns a status
code. Validation failures, out-of-range queries, allocation failure, and
unexpected exceptions therefore cross the ABI as ordinary integer values.

The project compiles its included C++17 sources from `build.rs`:

```rust
cc::Build::new()
    .cpp(true)
    .std("c++17")
    .include("native")
    .file("native/interpolator.cpp")
    .file("native/interpolator_c.cpp")
    .compile("interpolation");
```

The `cc` crate selects the platform compiler and tells Cargo how to link the
resulting static library and C++ runtime. This example requires a C++ compiler
but no separately installed native library.

The raw Rust module writes the small declaration set by hand:

```rust
#[repr(C)]
pub struct InterpolationHandle {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn interpolation_evaluate(
        handle: *const InterpolationHandle,
        coordinate: c_double,
        output: *mut c_double,
    ) -> c_int;
}
```

These declarations are promises about the C header, not checks against it.
Handwriting is reasonable for this small stable interface. For a larger or
frequently changing header, generating declarations with `bindgen` can reduce
copying errors, at the cost of adding a `libclang` build dependency.

The public Rust type owns a non-null opaque handle and releases it exactly once:

```rust
pub struct LinearInterpolator {
    handle: NonNull<raw::InterpolationHandle>,
    minimum: f64,
    maximum: f64,
}

impl Drop for LinearInterpolator {
    fn drop(&mut self) {
        unsafe { raw::interpolation_destroy(self.handle.as_ptr()) };
    }
}
```

Its safe constructor and `evaluate` method accept slices and return
`Result<_, InterpolationError>`. Callers cannot pass null pointers, forget the
array lengths, leak the C++ object, or allow a C++ exception to cross the FFI
boundary.

Run the example and its boundary tests:

```bash
cd source-code/cpp-interpolation-ffi
cargo run -- --query 1.5
cargo run -- --query 5.0
cargo test
```

The first query produces `2.5`. The second lies outside the tabulated domain
and demonstrates status-to-error conversion with a nonzero process exit.

## Scale The Pattern Up To FFTW

The first example owned the C++ implementation, C facade, raw declarations,
and safe wrapper. A mature scientific library often supplies the native API,
while a `-sys` crate supplies declarations close to that API and the logic
needed to link it. The application still has to recover the native library's
pointer, lifetime, ownership, and concurrency requirements in a safe Rust API.

FFTW makes those responsibilities concrete because it uses aligned native
buffers and opaque plans that retain buffer addresses. The example also adds a
scientific contract: a transform has to be normalized and interpreted
correctly, not merely called without crashing.

## The FFTW Scientific Workflow

The example creates a 64-sample signal with two known frequency components:

```text
x[j] = sin(2 pi 3 j / n) + 0.5 cos(2 pi 7 j / n)
```

It then:

1. computes a real-to-complex Fourier transform;
2. finds the two largest non-constant frequency bins;
3. performs the inverse transform;
4. normalizes the reconstructed signal;
5. computes maximum absolute and root-mean-square errors;
6. optionally writes the signal and power spectrum as CSV;
7. visualizes both series with a separate Python helper.

The expected dominant bins are 3 and 7. This gives the wrapper a scientific
property test rather than merely checking that the native call returned.

## The Raw And Safe Layers

The `fftw-sys` crate exposes FFTW declarations close to the C API. Its
functions are unsafe because Rust cannot verify the pointer sizes, alignment,
aliasing, initialization, handle lifetime, or concurrency rules.

The example separates those declarations from participant-facing code:

```text
src/
|-- lib.rs
|-- main.rs
`-- real_fft.rs
```

- `real_fft.rs` contains all calls to the raw FFTW API;
- `lib.rs` exposes only `RealFft` and `FftError`;
- `main.rs` uses ordinary slices, vectors, and `Result`.

The goal is not to make the native operations safe. The goal is to establish
and enforce their requirements once, then expose an API whose callers cannot
violate them.

## Use The System FFTW Installation

On Debian or Ubuntu, install the development package:

```bash
sudo apt install libfftw3-dev
```

The manifest selects the system library rather than downloading and compiling
another FFTW copy:

```toml
[dependencies]
clap = { version = "4.6.4", features = ["derive"] }
fftw-sys = {
    version = "0.8.0",
    default-features = false,
    features = ["system"]
}
num-complex = "0.4.6"
```

This keeps the dependency choice visible. On an HPC system, the build may
instead use a site-provided FFTW module and matching compiler environment.

## Validate Values At The Boundary

FFTW's one-dimensional planning function accepts its transform length as a C
`int`. The Rust API accepts a `usize`, rejects zero, and performs a checked
conversion:

```rust
if length == 0 {
    return Err(FftError::EmptyInput);
}

let ffi_length =
    i32::try_from(length).map_err(|_| FftError::LengthTooLarge(length))?;
```

The wrapper also validates input slice lengths before passing their pointers
to FFTW:

```rust
pub fn forward(
    &mut self,
    signal: &[f64],
) -> Result<Vec<Complex64>, FftError> {
    self.check_length(self.length, signal.len())?;
    self.real_buffer.as_mut_slice().copy_from_slice(signal);
    self.forward_plan.execute();
    Ok(self.complex_buffer.as_slice().to_vec())
}
```

These checks turn potential out-of-bounds native access into ordinary Rust
errors.

## Own Native Allocations

FFTW provides aligned allocation functions. `FftwBuffer<T>` records the
non-null pointer and element count:

```rust
struct FftwBuffer<T> {
    pointer: NonNull<T>,
    length: usize,
}
```

Construction checks for a null result. The allocation is initialized before
the wrapper creates Rust slices over it:

```rust
let pointer = unsafe { fftw_alloc_real(length) };
let pointer =
    NonNull::new(pointer).ok_or(FftError::AllocationFailed("real-valued"))?;

unsafe { pointer.as_ptr().write_bytes(0, length) };
```

The safety comments state facts that must remain true when the code changes.
They are not a substitute for those facts.

## Release Resources With Drop

Native allocations must be returned with FFTW's matching deallocator:

```rust
impl<T> Drop for FftwBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            fftw_free(self.pointer.as_ptr().cast::<c_void>());
        }
    }
}
```

Plans are opaque native handles with their own `Drop` implementation:

```rust
impl Drop for FftwPlan {
    fn drop(&mut self) {
        let _planner_guard = lock_planner();
        unsafe {
            fftw_destroy_plan(self.0.as_ptr());
        }
    }
}
```

This is RAII applied across an FFI boundary. Normal returns, early `?`
propagation, and panics all run the destructors for fully constructed values.

## Drop Order Is Part Of The Design

An FFTW plan retains the addresses of the buffers used during plan creation.
The wrapper therefore declares plans before buffers:

```rust
pub struct RealFft {
    length: usize,
    spectrum_length: usize,
    forward_plan: FftwPlan,
    inverse_plan: FftwPlan,
    real_buffer: FftwBuffer<f64>,
    complex_buffer: FftwBuffer<Complex64>,
}
```

Rust drops struct fields in declaration order. The plans are destroyed before
the allocations whose addresses they reference. Moving `RealFft` itself is
safe because moving the owner does not move its native heap allocations.

## Concurrency Requirements Still Matter

FFTW permits independent plans to execute concurrently, but its planner
operations require serialization. The wrapper protects plan creation and
destruction with one process-wide mutex:

```rust
static FFTW_PLANNER_LOCK: Mutex<()> = Mutex::new(());

fn lock_planner() -> MutexGuard<'static, ()> {
    FFTW_PLANNER_LOCK
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}
```

The execution path does not take this lock. The wrapper encodes the native
library's narrower concurrency rule instead of serializing the numerical
work.

## Real FFT Output Shape

For `n` real input samples, FFTW returns only the non-redundant half of the
spectrum:

```text
real input:      n values
complex output:  floor(n / 2) + 1 values
```

The wrapper computes and exposes that size:

```rust
let spectrum_length = length / 2 + 1;
```

For 64 input samples, the example reports 33 complex values. The two dominant
non-constant bins are found by sorting squared complex magnitudes.

## Define A One-Sided Power Spectrum

For a real signal, negative-frequency values repeat the power of their
positive-frequency counterparts. The example therefore writes a one-sided
mean-square power spectrum:

```text
P[k] = c[k] |X[k]|^2 / n^2

c[k] = 1  for the DC bin and, when n is even, the Nyquist bin
c[k] = 2  for every other stored bin
```

The factor of two accounts for the omitted negative-frequency value. With
this normalization, the sum of the stored powers equals the signal's mean
square:

```text
sum(P[k]) = sum(x[j]^2) / n
```

The implementation keeps this calculation separate from the FFI wrapper:

```rust
let one_sided_factor =
    if is_dc || is_nyquist { 1.0 } else { 2.0 };

one_sided_factor * value.norm_sqr() / (signal_length as f64).powi(2)
```

For the example signal, bin 3 has power `0.5` and bin 7 has power `0.125`.
Their sum, `0.625`, is the signal's mean square.

## Normalize The Inverse Transform

FFTW's inverse transform is unnormalized. A forward transform followed by its
inverse returns `n` times the input. The wrapper performs the normalization
before returning an owned vector:

```rust
let normalization = self.length as f64;
let reconstructed = self
    .real_buffer
    .as_slice()
    .iter()
    .map(|value| value / normalization)
    .collect();
```

The complex input to an inverse real transform may be overwritten, so
`inverse` copies the caller's slice into its owned FFTW buffer first. The
caller's spectrum remains unchanged.

## Keep Unsafe Local

The raw operations are limited to:

- allocating and freeing FFTW memory;
- creating, executing, and destroying plans;
- initializing native allocations;
- constructing slices from owned pointer-and-length pairs.

Every public operation on `RealFft` is safe:

```rust
let mut transform = RealFft::new(signal.len())?;
let spectrum = transform.forward(&signal)?;
let reconstructed = transform.inverse(&spectrum)?;
```

No public method accepts a raw pointer, and no returned value borrows memory
that FFTW may later overwrite.

## Prefer An Existing Safe Wrapper When It Fits

Writing the raw wrapper exposes the invariants that make FFI difficult, but an
application should not automatically maintain those invariants itself. The
`fftw` crate builds on `fftw-sys` and already provides aligned buffers, owning
plan types, safe transform methods, and Rust error values.

The companion project selects the same system FFTW installation:

```toml
fftw = {
    version = "0.8.0",
    default-features = false,
    features = ["system"]
}
```

Its forward transform uses safe crate APIs throughout:

```rust
let mut input = AlignedVec::new(length);
input.copy_from_slice(signal);

let mut spectrum = AlignedVec::new(length / 2 + 1);
let mut plan = R2CPlan64::aligned(&[length], Flag::ESTIMATE)?;
plan.r2c(&mut input, &mut spectrum)?;
```

The distinction is about ownership of the boundary:

| Concern | Local `fftw-sys` wrapper | High-level `fftw` crate |
|---|---|---|
| Aligned allocation and cleanup | Implemented with `NonNull` and `Drop` | Provided by `AlignedVec` |
| Plan ownership and destruction | Implemented locally | Provided by owning plan types |
| Raw pointers and safety invariants | Documented in local `unsafe` blocks | Encapsulated by the crate |
| Planner synchronization | Implemented locally | Handled by the crate |
| Signal definition and FFT normalization | Application responsibility | Application responsibility |
| Power-spectrum meaning and tests | Application responsibility | Application responsibility |

The two programs intentionally expose the same CLI and CSV schemas. For the
training signal, their generated signal and power-spectrum files are
byte-for-byte identical. That equivalence makes the comparison about API
design rather than different numerical problems.

For normal application code, start by evaluating a maintained high-level crate
such as `fftw`. Build directly on a `-sys` crate when the safe crate does not
expose a required feature, when its abstraction is unsuitable, or when the
wrapper itself is the subject being taught.

## Other Scientific Binding Layers

The raw-binding and wrapper split appears throughout the scientific Rust
ecosystem:

| Native library | Raw binding crate | Wrapper or application-level crate | Main abstraction added |
|---|---|---|---|
| FFTW | `fftw-sys` | `fftw` | Aligned buffers and owning transform plans |
| MPI | `mpi-sys` | `mpi` | Communicators, requests, and datatype traits |
| HDF5 | `hdf5-metno-sys` | `hdf5-metno` | Files, groups, datasets, and `ndarray` I/O |
| netCDF | `netcdf-sys` | `netcdf` | Files, dimensions, variables, and attributes |
| SUNDIALS | `sundials-sys` | `sundials` | Contexts and selected solver interfaces |
| BLAS | `blas-sys` | `blas` | Typed BLAS functions; calls remain `unsafe` |
| LAPACK | `lapack-sys` | `lapack`, `ndarray-linalg` | Thin functions or higher-level array methods |

This table is a starting point, not a guarantee that every native operation is
covered or safe. In particular, the `blas` and `lapack` crates remain close to
their Fortran APIs and expose `unsafe` numerical calls. The
`source-code/svd` example instead uses `ndarray-linalg`, which provides
array-oriented decomposition methods while delegating to LAPACK and OpenBLAS
below that interface.

The optional `source-code/hdf5-snapshot` example uses the maintained
`hdf5-metno` wrapper and renames it to `hdf5` in `Cargo.toml`, keeping concise
`hdf5::...` paths in application code. It demonstrates another reason to start
with a high-level wrapper: the application can work with files, datasets,
attributes, and `ndarray` values without owning raw HDF5 handles.

Before choosing a wrapper, check its supported native-library versions,
feature coverage, thread-safety model, error handling, maintenance status, and
system-linking options. Dropping to the corresponding `-sys` crate may still
be necessary for functionality the wrapper does not expose.

## Optional CSV Output

Both implementations always run the numerical computation, while two optional
`clap` arguments control visualization output:

```rust
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, value_name = "FILE")]
    signal_output: Option<PathBuf>,

    #[arg(long, value_name = "FILE")]
    spectrum_output: Option<PathBuf>,
}
```

Generate both files with:

```bash
cargo run -- \
  --signal-output signal.csv \
  --spectrum-output power-spectrum.csv
```

The signal file contains:

```text
sample,value
0,5.00000000000000000e-1
...
```

The spectrum file contains:

```text
frequency_bin,power
0,9.43705672749745568e-33
...
3,5.00000000000000222e-1
```

Both files use full-precision scientific notation. They can be read directly
by Python, Julia, R, a spreadsheet, or a command-line plotting tool. File
creation and write failures propagate through `Result` rather than being
silently ignored. The program rejects using the same path for both files so
that the second write cannot overwrite the first result.

## Visualize The Signal And Spectrum

The repository includes
`source-code/fftw-ffi/visualize-signal-and-spectrum.py`. It validates the two
CSV schemas and plots the signal and one-sided power spectrum side by side.
Install Matplotlib:

```bash
python3 -m pip install matplotlib
```

After generating the CSV files, display the interactive figure with:

```bash
./visualize-signal-and-spectrum.py signal.csv power-spectrum.csv
```

On a headless system, write an image instead:

```bash
./visualize-signal-and-spectrum.py \
  signal.csv power-spectrum.csv \
  --output signal-and-spectrum.png
```

Matplotlib selects the output format from the filename extension, so the same
option can also produce PDF or SVG output.

The horizontal axes are the sample index and the discrete frequency-bin index.
No physical time step or sampling frequency is specified by this example, so
the script does not imply time or frequency units that the input data does not
contain.

## Test Boundary And Scientific Behavior

The raw-wrapper tests cover both boundary contracts and numerical results:

- an empty transform is rejected;
- wrong input and spectrum lengths are rejected before the FFI call;
- a 64-point real transform returns 33 complex values;
- the dominant bins are 3 and 7;
- bin powers are `0.5` and `0.125`;
- total spectral power equals the signal mean square;
- forward and inverse transforms reproduce the original signal within
  `1.0e-12`;
- the CSV writer produces the expected headers and numeric representation;
- one path cannot be used for both output files.

The safe-crate version repeats the scientific, round-trip, CSV, and CLI
contract tests. Its dependency owns the raw allocation and plan boundary, so
the application does not repeat the local wrapper's boundary tests.

Run the examples in teaching order:

```bash
cd source-code/cpp-interpolation-ffi
cargo run -- --query 1.5
cargo test

cd ../fftw-ffi
cargo run
cargo test

cd ../fftw-safe
cargo run
cargo test
```

Representative output is:

```text
Signal length: 64
Non-redundant spectrum length: 33
Dominant frequency bins: [3, 7]
Maximum round-trip error: 4.441e-16
RMS round-trip error: 1.640e-16
```

The final digits may vary with the FFTW build. The frequency bins and tolerance
checks are the stable contract.

## Hands-On Exercises

1. In `cpp-interpolation-ffi`, trace one query through the safe wrapper, raw
   declaration, C facade, and C++ method.
2. Add a safe method that evaluates several query coordinates while keeping
   all pointer and exception handling below the public Rust API.
3. Pass a slice with the wrong length to `RealFft` and inspect the `FftError`.
4. Trace which destructors run if inverse-plan construction fails.
5. Add a `len` method to `RealFft` without exposing either native buffer.
6. Change one frequency in `create_signal` and predict the dominant bins.
7. Remove the inverse normalization and explain the resulting scale factor.
8. Change the signal length and verify the `n / 2 + 1` spectrum shape.
9. Generate both CSV files and visualize them with the Python helper.
10. Compare `fftw-ffi` with `fftw-safe`: list which safety obligations disappear
   from the application and which scientific responsibilities remain.
11. Change the signal in both FFTW versions and compare their generated CSV
    files.

## Summary

- A C facade gives a C++ library a stable ABI based on functions, opaque
  handles, pointer-length pairs, and status codes.
- `build.rs` and `cc` can compile included C++ sources as part of a Cargo build.
- Raw FFI functions transfer pointer, lifetime, and concurrency obligations to
  the caller.
- A safe wrapper validates dimensions and conversions before the native call.
- `NonNull`, owned lengths, and `Drop` model native allocations and handles.
- Field order can be part of a resource-lifetime invariant.
- Small `unsafe` blocks should state the invariants that make each call valid.
- A suitable high-level crate avoids duplicating native ownership and safety
  machinery in application code.
- A one-sided spectrum needs explicit normalization and endpoint treatment.
- Optional output paths keep visualization separate from the core computation.
- A small plotting helper can consume the numerical CSV contract independently.
- Safe native bindings still need application-level scientific property and
  round-trip tests.
- Rust can safely compose with established HPC libraries without rewriting
  their numerical kernels.
