# FFTW through its high-level Rust crate

This application performs the same numerical workflow as `../fftw-ffi`, but
uses the high-level `fftw` crate instead of calling `fftw-sys` directly. It:

* creates the same 64-sample signal with frequency bins 3 and 7;
* computes a real-to-complex Fourier transform;
* identifies the dominant frequency bins;
* performs and normalizes the inverse transform;
* checks the round-trip error and one-sided power normalization;
* optionally writes CSV files compatible with the existing visualization
  helper.

The `fftw` crate supplies aligned vectors, owning plan types, error handling,
and safe transform methods. This project therefore contains no local `unsafe`
blocks, raw pointers, native `Drop` implementations, or planner lock.

The example uses a system FFTW installation. On Debian or Ubuntu, install the
development package with:

```bash
sudo apt install libfftw3-dev
```

## How to use?

Run the application and its tests with:

```bash
cargo run
cargo test
```

Write the same CSV files as the raw FFI version:

```bash
cargo run -- \
  --signal-output signal.csv \
  --spectrum-output power-spectrum.csv
```

Visualize them with the helper stored beside the raw FFI example:

```bash
../fftw-ffi/visualize-signal-and-spectrum.py \
  signal.csv power-spectrum.csv
```

For a headless run, write an image:

```bash
../fftw-ffi/visualize-signal-and-spectrum.py \
  signal.csv power-spectrum.csv \
  --output signal-and-spectrum.png
```

The output paths must be different. Both CSV files use full-precision
scientific notation. The power spectrum is normalized so that its sum equals
the signal's mean square apart from floating-point rounding.
