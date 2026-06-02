# Structural Matching

This example parses the same timestamped instrument-log format as
`../strings`, but makes more use of structural pattern matching while parsing
fields and summarizing the result. Compared to `../strings`, it shows how to:

* match on the tuple returned by `split_once(':')`;
* destructure `key: value` lines directly in `match` arms;
* match on nested results such as `Some(("time", value))`;
* match on parse results from `DateTime<Utc>` and `f64`;
* match on tuple-shaped optional state in the aggregator;
* keep parser success and failure paths explicit with `Ok` and `Err`;
* reuse the same line-based text workflow with a different parser shape.

The example is meant as a follow-up to `../strings`. It keeps the same input
format so the teaching focus can move from string handling to the structure of
the patterns.


## What is it?

1. `src/main.rs`: main source file for the application. It reads timestamped
   records, parses each record with structural matching, and computes average
   temperature, average pressure, and the time span covered by the data.
1. `data.txt`: sample input containing three timestamped records.
1. `Cargo.toml`: configuration file for the Rust package manager. It specifies
   dependencies on `clap` and `chrono`.
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
