# HashMap and HashSet

This example uses `HashMap` and `HashSet` to process simple DNA-like sequence
data.  Compared to `../random-numbers`, it shows how to:

* define shared constants and helper functions in `src/lib.rs`;
* reuse library code from multiple binaries in the same package;
* define several command-line applications with `[[bin]]` sections in
  `Cargo.toml`;
* count values with a `HashMap`;
* collect unique values with a `HashSet`;
* select a random element from an array with `choose`;
* stream input with `BufReader`;
* stream output with `BufWriter`;
* iterate over input bytes with the `bytes` iterator;
* use match guards to combine pattern matching with predicate functions.

The example keeps the valid nucleotide set in one place so generation, error
injection, and counting all use the same definition.


## What is it?

1. `src/lib.rs`: shared definitions for valid nucleotides and possible read
   error tokens.
1. `src/generate-data.rs`: generates random DNA sequence data from the shared
   nucleotide list.
1. `src/read-errors.rs`: introduces random read errors into valid nucleotide
   positions.
1. `src/count-nucleotides.rs`: counts valid nucleotides with a `HashMap` and
   collects invalid tokens with a `HashSet`.
1. `data.txt`: sample DNA-like sequence data.
1. `errors.txt`: sample sequence data after introducing read errors.
1. `Cargo.toml`: configuration file for the Rust package manager.  It defines
   three binaries and dependencies on `clap`, `rand`, `rand_chacha`, and
   `rand_distr`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Generate a sequence:

```bash
cargo run --bin generate-data -- --count 800 --file data.txt
```

Introduce read errors:

```bash
cargo run --bin read-errors -- --file data.txt --output errors.txt --error-rate 0.1
```

Count valid nucleotides and report invalid tokens:

```bash
cargo run --bin count-nucleotides -- --file errors.txt
```
