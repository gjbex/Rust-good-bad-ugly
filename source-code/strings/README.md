# Strings

This example parses a simple instrument log with timestamped records and
aggregates the numeric fields. Compared to `../hashmap-hashset`, it shows how
to:

* read text input line by line with `BufRead::lines`;
* accumulate a multi-line record in an owned `String`;
* parse borrowed string data through a function that accepts `&str`;
* split and trim field values from `key: value` text lines;
* parse numeric values from strings with `parse::<f64>`;
* parse UTC timestamps with `chrono::DateTime<Utc>`;
* convert missing parsed fields from `Option` to `Result` with `ok_or_else`;
* use `match` on parser results and on tuple-shaped state;
* aggregate records without loading the whole input file into memory.

The example intentionally keeps the file format small. It is meant to make
string ownership, borrowed string slices, line-based text processing, and
date/time parsing visible in one compact scientific-data workflow.


## What is it?

1. `src/main.rs`: main source file for the application. It reads timestamped
   records, parses each record, and computes average temperature, average
   pressure, and the time span covered by the data.
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
