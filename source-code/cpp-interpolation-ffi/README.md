# Wrapping a C++ interpolation library

This example wraps a small home-grown C++ linear-interpolation library for
which no Rust `-sys` crate exists. It complements the FFTW examples by showing
the complete binding workflow rather than starting from generated raw bindings.

The layers are:

```text
safe Rust API       src/lib.rs
raw Rust bindings   src/raw.rs
stable C facade     native/interpolator_c.h and native/interpolator_c.cpp
C++ library         native/interpolator.hpp and native/interpolator.cpp
```

Rust does not call the C++ class directly. The `extern "C"` facade removes C++
name mangling, represents the class as an opaque handle, accepts arrays as
pointer-length pairs, and catches all exceptions before they can cross the FFI
boundary. `build.rs` compiles the included C++17 sources with the `cc` crate.
A C++ compiler is therefore required, but there is no external native library
to install.

The safe Rust `LinearInterpolator` owns the opaque handle and releases it with
`Drop`. Its constructor and `evaluate` method convert C status codes to the
typed `InterpolationError` enum. Callers use only slices and `Result`.

Run the example:

```bash
cargo run
cargo run -- --query 3.0
```

An out-of-range query demonstrates error propagation and a nonzero exit:

```bash
cargo run -- --query 5.0
```

Run the numerical and boundary tests with:

```bash
cargo test
```

For a live walkthrough, follow one operation downward from `src/main.rs` to
the C++ class, then trace ownership upward: the C++ allocation becomes an
opaque C handle, a non-null Rust pointer, and finally an owning Rust value.

