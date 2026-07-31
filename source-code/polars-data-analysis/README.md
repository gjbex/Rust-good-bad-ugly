# Patient Data Analysis With Polars

This extended example mirrors the patient-data workflow used in the
`Python-for-data-science` training, but implements it directly in Rust with
[Polars](https://pola.rs/). It is intended as an optional follow-up to the
collections and CSV material rather than as a separate learning module.

The example demonstrates how to:

- scan two CSV files lazily;
- select the columns that form the input contract;
- join patient metadata to experimental measurements;
- filter missing temperatures;
- derive columns with expressions;
- group and aggregate by condition and gender;
- inspect the optimized query plan; and
- optionally write the summary as Parquet.

From this directory, run:

```bash
cargo run --release
```

Show the optimized query plan:

```bash
cargo run --release -- --show-plan
```

Write the result to a columnar file for later analysis:

```bash
cargo run --release -- --output summary.parquet
```

Polars has a substantial dependency graph. Build this example before a live
demonstration so that the initial optimized compilation does not interrupt the
session.

Use `--metadata` and `--measurements` to supply other files with the same
columns. Run `cargo test` to exercise the query with small in-memory
DataFrames.

The inner join deliberately illustrates a data-quality decision: measurements
for patient 4 have no matching metadata and are excluded. Likewise, metadata
for patients 10 and 11 has no measurements and does not appear in the summary.
In a real analysis, these unmatched records should also be counted and
reported rather than silently accepted.
