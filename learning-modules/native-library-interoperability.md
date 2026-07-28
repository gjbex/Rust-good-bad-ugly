# Native Library Interoperability

Scientific Rust programs rarely start in isolation. They often need mature C
or Fortran libraries that already implement validated algorithms and are
installed on HPC systems. This module wraps FFTW's C API in a small safe Rust
interface.

The complete example is in `source-code/fftw-ffi`.

## Learning Objectives

By the end of this module, you should be able to:

- recognize a raw FFI layer and the obligations it transfers to the caller;
- convert Rust lengths to C integer types with validation;
- pass contiguous buffers to native functions through raw pointers;
- represent native allocation and opaque handles with owning Rust types;
- release native resources through `Drop`;
- concentrate `unsafe` operations behind a safe slice-based API;
- translate native failures and boundary violations into Rust errors;
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

## The Scientific Workflow

The example creates a 64-sample signal with two known frequency components:

```text
x[j] = sin(2 pi 3 j / n) + 0.5 cos(2 pi 7 j / n)
```

It then:

1. computes a real-to-complex Fourier transform;
2. finds the two largest non-constant frequency bins;
3. performs the inverse transform;
4. normalizes the reconstructed signal;
5. computes maximum absolute and root-mean-square errors.

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

## Test Boundary And Scientific Behavior

The unit tests cover both wrapper contracts and numerical results:

- an empty transform is rejected;
- wrong input and spectrum lengths are rejected before the FFI call;
- a 64-point real transform returns 33 complex values;
- the dominant bins are 3 and 7;
- forward and inverse transforms reproduce the original signal within
  `1.0e-12`.

Run the complete example:

```bash
cd source-code/fftw-ffi
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

1. Change one frequency in `create_signal` and predict the dominant bins.
2. Remove the inverse normalization and explain the resulting scale factor.
3. Pass a slice with the wrong length and inspect the `FftError`.
4. Trace which destructors run if inverse-plan construction fails.
5. Add a `len` method to `RealFft` without exposing either native buffer.
6. Change the signal length and verify the `n / 2 + 1` spectrum shape.

## Summary

- Raw FFI functions transfer pointer, lifetime, and concurrency obligations to
  the caller.
- A safe wrapper validates dimensions and conversions before the native call.
- `NonNull`, owned lengths, and `Drop` model native allocations and handles.
- Field order can be part of a resource-lifetime invariant.
- Small `unsafe` blocks should state the invariants that make each call valid.
- A native wrapper still needs scientific property and round-trip tests.
- Rust can safely compose with established HPC libraries without rewriting
  their numerical kernels.
