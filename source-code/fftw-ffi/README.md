# FFTW foreign-function interface

This application wraps FFTW's raw C interface in a small safe Rust API. It:

* creates a signal containing two known frequencies;
* computes a real-to-complex Fourier transform;
* identifies the dominant frequency bins;
* performs the inverse transform and normalizes the result;
* checks the round-trip error with numerical tolerances.

The wrapper owns FFTW-aligned buffers and opaque plans. Its `Drop`
implementations release those native resources, while the public `RealFft`
methods accept ordinary Rust slices.

The example uses a system FFTW installation. On Debian or Ubuntu, install the
development package with:

```bash
sudo apt install libfftw3-dev
```


## What is it?

1. `src/real_fft.rs` contains the safe wrapper and the concentrated
   `unsafe` calls to FFTW.
1. `src/lib.rs` exposes the wrapper as a Rust library API.
1. `src/main.rs` creates and transforms the signal, reports its dominant
   frequencies, and measures the reconstruction error.
1. `Cargo.toml` selects the `system` backend for `fftw-sys`.
1. `Cargo.lock` records the exact dependency versions used by the example.


## How to use?

Run the application with:

```bash
cargo run
```

Run its unit tests with:

```bash
cargo test
```
