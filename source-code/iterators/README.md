# Iterators

This example processes tabular data with iterator pipelines.  Compared to
`../generic-structs`, it shows how to:

* read records from a CSV file with the `csv` crate;
* deserialize records into a Rust `struct` with `serde`;
* collect iterator results into explicitly typed vectors;
* copy scalar values out of borrowed iterators with `copied`;
* filter values with `filter`;
* transform values with `map`;
* combine related iterators with `zip`;
* split pairs back into separate vectors with `unzip`;
* reduce values with `sum`;
* attach indices to values with `enumerate`.

The type annotation on calls to `collect` is intentional: the iterator item type
can be inferred, but the collection type has to be specified.


## What is it?

1. `src/main.rs`: main source file for the application.  It reads a CSV file,
   stores the `x` and `y` columns in vectors, and demonstrates several iterator
   adapters.
1. `data.txt`: sample CSV input containing `x` and `y` values.
1. `Cargo.toml`: configuration file for the Rust package manager.  It specifies
   the dependencies on `clap`, `csv`, and `serde`.
1. `Cargo.lock`: lock file for the Rust package manager, automatically
   generated when building the application.


## How to use?

Run the application on the sample data:

```bash
cargo run -- --file data.txt
```

Show the command-line help:

```bash
cargo run -- --help
```
