# Collections, Iterators, And Text Data

This module introduces collection-oriented programming in Rust. It focuses on
vectors, iterator pipelines, hash maps, hash sets, and simple text-file
processing. These are the tools that turn earlier examples from small scalar
computations into programs that process data.

The examples in this module deliberately stay close to common scientific and
technical workflows: read data, store values, transform collections, compute
summaries, and count or classify tokens.

## Learning Objectives

After completing this module, participants should be able to:

- Store homogeneous values in `Vec<T>`.
- Build vectors incrementally with `push`.
- Iterate over collections with `iter`, `iter_mut`, and `into_iter`.
- Use iterator adapters such as `filter`, `map`, `zip`, `unzip`, and
  `enumerate`.
- Use `sum` for simple reductions.
- Explain when `copied` is useful.
- Explain the role of `collect` and why type annotations are sometimes needed.
- Use `HashMap` to count values.
- Use `HashSet` to collect unique values.
- Read text input with buffered readers.
- Write text output with buffered writers.
- Process input byte by byte when that is appropriate for the file format.
- Process input line by line when records are naturally text lines.
- Distinguish owned `String` data from borrowed `&str` views.
- Parse timestamp strings into date/time values with a crate.

## Prerequisites

Participants should already be comfortable with:

- Ownership and borrowing.
- Shared and mutable references.
- Functions and closures.
- `for` loops.
- Basic error handling with `Result` is useful, but not the main focus here.

The examples used in this module are:

- `source-code/iterators`
- `source-code/hashmap-hashset`
- `source-code/strings`

## Vectors

A `Vec<T>` stores a growable sequence of values of the same type. The
`iterators` example reads two numeric columns from a CSV file and stores them in
separate vectors:

```bash
cd source-code/iterators
cargo run -- --file data.txt
```

The relevant setup is:

```rust
let mut xs = Vec::new();
let mut ys = Vec::new();
```

Values are appended with `push`:

```rust
xs.push(value.x);
ys.push(value.y);
```

The vectors are mutable because reading the file grows them one record at a
time.

## Reading Structured Text With `csv` And `serde`

The `iterators` example uses the `csv` crate to read records and `serde` to
deserialize each record into a Rust struct:

```rust
#[derive(Deserialize, Debug)]
struct Values {
    x: f64,
    y: f64,
}
```

The file is opened through the CSV reader:

```rust
let mut reader = csv::Reader::from_path(args.file)?;
```

Each record is deserialized in a loop:

```rust
for result in reader.deserialize() {
    let value: Values = result?;
    xs.push(value.x);
    ys.push(value.y);
}
```

The important pattern is that external text data is converted into typed Rust
values near the input boundary. The rest of the program can then work with
`Vec<f64>` rather than raw strings.

## Borrowed Iteration And `copied`

Calling `iter` on a vector yields references to the elements:

```rust
xs.iter()
```

For a `Vec<f64>`, this produces items of type `&f64`. When the program wants
independent `f64` values in the iterator pipeline, it can use `copied`:

```rust
let filtered_xs: Vec<f64> = xs
    .iter()
    .copied()
    .filter(|x| *x >= 10.0)
    .collect();
```

The `copied` adapter is appropriate here because `f64` is a small scalar type
that implements `Copy`.

Without `copied`, the pipeline would operate on references. That is often fine,
but collecting owned scalar values is clearer for this example.

## Filtering Values

The `filter` adapter keeps only values that satisfy a predicate:

```rust
let filtered_xs: Vec<f64> = xs
    .iter()
    .copied()
    .filter(|x| *x >= 10.0)
    .collect();
```

The closure:

```rust
|x| *x >= 10.0
```

decides whether each value should be kept.

Iterator adapters are lazy. The pipeline does not produce the final vector
until `collect` is called.

## Mapping Values

The `map` adapter transforms each item:

```rust
let cubed_xs: Vec<f64> = xs
    .iter()
    .copied()
    .map(|x| x.powi(3))
    .collect();
```

This pipeline reads the `x` values, copies each scalar value, computes its
cube, and collects the results into a new vector.

This is often clearer than writing a manual loop when the computation is a
straightforward element-wise transformation.

## Collecting Results

The `collect` adapter consumes an iterator and builds a collection:

```rust
let cubed_xs: Vec<f64> = xs
    .iter()
    .copied()
    .map(|x| x.powi(3))
    .collect();
```

The type annotation is important:

```rust
Vec<f64>
```

Rust can usually infer the iterator item type, but it often needs help knowing
which collection type to build. The same iterator could sometimes be collected
into a `Vec`, a `HashSet`, or another collection.

## Combining Iterators With `zip`

The `zip` adapter combines two iterators into one iterator over 2-tuples:

```rust
let filtered_pairs: Vec<(f64, f64)> = xs
    .iter()
    .copied()
    .zip(ys.iter().copied())
    .filter(|(x, _)| *x >= 10.0)
    .collect();
```

Here, the `x` and `y` columns are combined again after being stored in separate
vectors. The filter then keeps only 2-tuples whose `x` value is at least
`10.0`.

This is useful when two sequences represent related data and should be
processed together.

## Splitting 2-Tuples With `unzip`

The inverse operation is `unzip`, which splits an iterator over 2-tuples into
two collections:

```rust
let (filtered_xs_unpacked, filtered_ys_unpacked): (Vec<f64>, Vec<f64>) =
    filtered_pairs
        .iter()
        .copied()
        .unzip();
```

The result type is written explicitly:

```rust
(Vec<f64>, Vec<f64>)
```

This tells Rust that the first components should be collected into one vector
and the second components into another vector.

## Reductions With `sum`, `fold`, And `scan`

Some iterator operations reduce many values to one value. The `iterators`
example computes the sum of the `y` values:

```rust
let sum_y: f64 = ys.iter().sum();
```

The type annotation tells Rust which numeric type the sum should produce.

For more general accumulation, use `fold`:

```rust
let sum_of_squares = xs
    .iter()
    .copied()
    .fold(0.0, |accumulator, x| accumulator + x * x);
```

`fold` carries an accumulator through the iterator and returns the final
accumulated value.

Use `scan` when the intermediate accumulated states are also part of the
result:

```rust
let cumulative_sum: Vec<f64> = xs
    .iter()
    .copied()
    .scan(0.0, |state, x| {
        *state += x;
        Some(*state)
    })
    .collect();
```

This produces the running sum after each input value. Conceptually, `fold`
returns only the final accumulated value, while `scan` yields the sequence of
accumulated states.

## Adding Indices With `enumerate`

The `enumerate` adapter attaches an index to each item:

```rust
let indexed_xs: Vec<(usize, f64)> = xs
    .iter()
    .copied()
    .enumerate()
    .collect();
```

This produces 2-tuples of the form:

```rust
(index, value)
```

It can also be used directly in a loop:

```rust
for (i, y) in ys.iter().enumerate() {
    println!("Index: {i}, y value: {y:.1}");
}
```

This is usually preferable to manually maintaining a separate counter.

## Hash Maps For Counting

A `HashMap<K, V>` stores values by key. The `hashmap-hashset` example uses a
hash map to count nucleotide characters:

```bash
cd source-code/hashmap-hashset
cargo run --bin count-nucleotides -- --file errors.txt
```

The count map is created with:

```rust
let mut counts = HashMap::new();
```

Each valid nucleotide updates its count:

```rust
*counts.entry(nucleotide).or_insert(0) += 1;
```

This pattern is common enough to read carefully:

- `entry(nucleotide)` selects the map entry for that key.
- `or_insert(0)` inserts `0` if the key was not present.
- `*... += 1` increments the value stored in the map.

After processing the file, the program ensures that every valid nucleotide has
an entry:

```rust
for nucleotide in VALID_NUCLEOTIDES {
    counts.entry(nucleotide).or_insert(0);
    println!("{nucleotide}: {}", counts[&nucleotide]);
}
```

This makes the output stable even if a nucleotide did not occur in the input.

## Hash Sets For Unique Values

A `HashSet<T>` stores unique values. The same example uses a hash set to record
which invalid tokens appeared in the input:

```rust
let mut error_tokens = HashSet::new();
```

When an invalid token is found, it is inserted:

```rust
error_tokens.insert(nucleotide);
```

If the same invalid token appears many times, the set still stores it once.
That makes `HashSet` a natural choice when the question is "which values were
seen?" rather than "how many times did each value occur?"

## Buffered Text Input

The nucleotide-counting example reads a text file through a buffered reader:

```rust
let file = std::fs::File::open(args.file)
    .expect("Failed to open the DNA sequence file");
let reader = BufReader::new(file);
```

Buffered input avoids asking the operating system for tiny pieces of data one
at a time. This matters for larger files.

The example then iterates over bytes:

```rust
for byte in reader.bytes() {
    let nucleotide = byte.expect("Failed to read the DNA sequence file") as char;
    // process nucleotide
}
```

Byte-wise processing is appropriate here because the input is simple
ASCII-like sequence data. For general Unicode text, line-based or string-based
processing is usually more appropriate.

## Line-Based String Processing

The `strings` example uses line-based input for timestamped records:

```bash
cd source-code/strings
cargo run -- --file data.txt
```

The input is a small instrument-log format:

```text
time: 2023-06-01T12:00:00Z
temperature: 42.3
pressure: 1013.25
----
```

Line-based processing keeps the input format visible:

```rust
let reader = BufReader::new(file);

for line in reader.lines() {
    let line = line.expect("Failed to read line");
    // process one line
}
```

Each call to `lines` yields an owned `String`. The program builds a record in
another owned `String`:

```rust
record_buffer.push_str(&line);
record_buffer.push('\n');
```

When a complete record has been collected, parsing borrows the string data:

```rust
fn parse_record(record_str: &str) -> Result<Record, String> {
    // parse fields from borrowed text
}
```

This signature says that parsing reads the text but does not take ownership of
the buffer.

## Splitting And Parsing Fields

The record parser extracts values from lines such as `temperature: 42.3`.
Numeric values can be parsed from trimmed strings:

```rust
let temperature_val = temp_str
    .trim()
    .parse::<f64>();
```

The timestamp line is deliberately a little trickier:

```text
time: 2023-06-01T12:00:00Z
```

The timestamp value itself contains colons, so the example uses `split_once`
for that field:

```rust
line.split_once(':').map(|(_, value)| value)
```

This is a useful parsing lesson: a method that is sufficient for one field may
be wrong for another field if the data format changes the assumptions.

## Date And Time Values

Rust's standard library has `std::time::Duration`, `Instant`, and
`SystemTime`, but it does not provide full calendar date/time parsing. The
`strings` example therefore uses the `chrono` crate:

```rust
use chrono::{DateTime, Utc};
```

The record stores a parsed UTC timestamp:

```rust
struct Record {
    time: DateTime<Utc>,
    temperature: f64,
    pressure: f64,
}
```

Once timestamps are parsed, the program can compute the covered time span:

```rust
let duration = last - first;
let days = duration.num_seconds() as f64 / 86400.0;
```

This keeps date/time handling explicit and avoids treating timestamps as raw
strings after the input boundary.

## Matching Parser And Aggregator State

The same example also shows `match` on parser results:

```rust
match parse_record(&record_buffer) {
    Ok(record) => aggregator.add_record(record),
    Err(err) => {
        eprintln!("Failed to parse record:\n{}", record_buffer);
        eprintln!("Error: {}", err);
    }
}
```

The aggregator uses a structural match on the pair of optional timestamps:

```rust
match (self.first_time, self.last_time) {
    (None, None) => {
        self.first_time = Some(record.time);
        self.last_time = Some(record.time);
    }
    (Some(first), Some(last)) => {
        if record.time < first {
            self.first_time = Some(record.time);
        }
        if record.time > last {
            self.last_time = Some(record.time);
        }
    }
    _ => unreachable!("first_time and last_time should be updated together"),
}
```

This connects the earlier `match` discussion to data-processing code: patterns
can describe the structure of ordinary values, not only which enum variant was
selected at the command line.

## Buffered Text Output

The data-generation and error-injection programs use buffered writers:

```rust
let file = std::fs::File::create(args.file).expect("Unable to create file");
let mut output = std::io::BufWriter::new(file);
```

Values are written with `write!` and `writeln!`:

```rust
write!(output, "{random_nucleotide}").expect("Unable to write file");
writeln!(output).expect("Unable to write file");
```

Buffered output is the counterpart to buffered input: it groups many small
writes into fewer larger writes.

## Matching While Processing Input

The nucleotide-counting example classifies each character with `match` and
match guards:

```rust
match nucleotide {
    nucleotide if is_valid_nucleotide(nucleotide) => {
        *counts.entry(nucleotide).or_insert(0) += 1;
    }
    nucleotide if nucleotide.is_whitespace() => {}
    _ => {
        error_tokens.insert(nucleotide);
    }
}
```

The cases are:

- valid nucleotide: increment its count;
- whitespace: ignore it;
- anything else: record it as an error token.

This combines pattern matching with collection updates.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run the iterator example:

   ```bash
   cd source-code/iterators
   cargo run -- --file data.txt
   ```

2. Change the `filter` threshold from `10.0` to another value and inspect the
   output.

3. Add a `map` pipeline that computes `x.sqrt()` for all non-negative `x`
   values.

4. Add a `fold` expression that computes the sum of squares of the `y` values.

5. Add a `scan` expression that computes the cumulative sum of the `y` values.

6. Use `zip` to compute a vector of `x + y` values.

7. Run the nucleotide-counting example:

   ```bash
   cd source-code/hashmap-hashset
   cargo run --bin count-nucleotides -- --file errors.txt
   ```

8. Add a second `HashMap` that counts invalid tokens instead of storing only
   the unique invalid tokens.

9. Change the output order by printing the contents of the map directly, then
   compare that with iterating over `VALID_NUCLEOTIDES`.

10. Run the data generator and error injector to produce a new input file:

    ```bash
    cargo run --bin generate-data -- --count 200 --file data.txt
    cargo run --bin read-errors -- --file data.txt --output errors.txt --error-rate 0.2
    cargo run --bin count-nucleotides -- --file errors.txt
    ```

11. Run the timestamped string parser:

    ```bash
    cd ../strings
    cargo run -- --file data.txt
    ```

12. Change one timestamp, temperature, or pressure value and inspect how the
    averages or covered time span change.

13. Remove one field from a record and inspect the parser error message.

## Discussion Points

This module is a good place to emphasize:

- Iterators describe a sequence of processing steps.
- Iterator adapters are lazy until consumed by `collect`, `sum`, a `for` loop,
  or another consuming operation.
- Type annotations on `collect` tell Rust what collection to build.
- `copied` is useful when moving from borrowed scalar values to owned scalar
  values.
- `HashMap` is useful for counts and lookup tables.
- `HashSet` is useful for uniqueness.
- Buffered I/O is a sensible default for file-based text processing.
- Choose byte-wise, line-wise, or record-wise processing based on the input
  format.
- Use owned `String` values when the program needs to keep or grow text.
- Use borrowed `&str` parameters when a function only needs to read text.
- Convert external timestamps into date/time values near the input boundary.

## Connection To Later Modules

Collection and iterator patterns appear throughout larger Rust programs:

- Project-organization examples reuse collection-processing code from multiple
  binaries.
- Error-handling examples make file and parse failures explicit.
- Randomness examples generate collections of synthetic data.
- Julia set examples fill matrix-like storage with computed values.
- The N-body example iterates over particles, forces, diagnostics, and output
  records.

Once participants are comfortable processing collections and text data, they
are ready to study error handling and then the project organization needed for
larger examples.
