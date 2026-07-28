# FFTW foreign-function interface

This application wraps FFTW's raw C interface in a small safe Rust API. It:

* creates a signal containing two known frequencies;
* computes a real-to-complex Fourier transform;
* identifies the dominant frequency bins;
* performs the inverse transform and normalizes the result;
* checks the round-trip error with numerical tolerances;
* optionally writes the signal and its one-sided power spectrum as CSV;
* visualizes both series with a small Python helper.

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
   frequencies, measures the reconstruction error, and handles optional CSV
   output.
1. `visualize-signal-and-spectrum.py` plots the two CSV files side by side.
1. `Cargo.toml` selects the `system` backend for `fftw-sys` and enables
   `clap`'s derive macros.
1. `Cargo.lock` records the exact dependency versions used by the example.


## How to use?

Run the application with:

```bash
cargo run
```

Write CSV files for visualization with:

```bash
cargo run -- \
  --signal-output signal.csv \
  --spectrum-output power-spectrum.csv
```

The signal file contains `sample,value` columns. The spectrum file contains
`frequency_bin,power` columns. Its power values use a one-sided normalization,
so their sum equals the signal's mean square apart from floating-point
rounding. The two output paths must be different.

Install Matplotlib and display the two curves with:

```bash
python3 -m pip install matplotlib
./visualize-signal-and-spectrum.py signal.csv power-spectrum.csv
```

In a headless environment, write an image instead:

```bash
./visualize-signal-and-spectrum.py \
  signal.csv power-spectrum.csv \
  --output signal-and-spectrum.png
```

Run its unit tests with:

```bash
cargo test
```
