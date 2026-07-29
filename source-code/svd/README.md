# Singular value decomposition

This application illustrates a small linear-algebra workflow with `ndarray`
and `ndarray-linalg`. It:

* creates a rectangular matrix;
* computes its singular value decomposition (SVD);
* reconstructs the matrix with two matrix multiplications;
* compares the reconstruction with the original matrix using maximum absolute
  and Frobenius errors.

The example uses a system OpenBLAS installation. On Debian or Ubuntu, install
the development package with:

```bash
sudo apt install libopenblas-dev
```


## What is it?

1. `src/main.rs` contains the matrix construction, SVD, reconstruction, error
   calculations, and unit tests.
1. `Cargo.toml` selects the `openblas-system` backend for `ndarray-linalg`.
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
