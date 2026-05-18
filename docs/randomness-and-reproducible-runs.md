# Randomness And Reproducible Runs

This module introduces random number generation in Rust with an emphasis on
reproducibility. Randomness is common in simulations, sampling, randomized
algorithms, and synthetic data generation. For scientific and technical work,
it is not enough to generate random values; a run should also be repeatable
when the same inputs and seed are used.

The example in this module generates samples from selected probability
distributions and optionally visualizes the result with a small Python helper.

## Learning Objectives

After completing this module, participants should be able to:

- Explain why reproducible random streams require explicit seeds.
- Choose a named random number generator.
- Construct a seedable RNG with `SeedableRng`.
- Generate values from a uniform distribution.
- Generate values from a normal distribution with `rand_distr`.
- Use a command-line enum to select a distribution.
- Separate CLI choices from runtime distribution objects.
- Write a method generic over RNG implementations.
- Generate data that can be piped into another tool.
- Visualize generated samples with a histogram.

## Prerequisites

Participants should already be comfortable with:

- Cargo projects and command-line arguments.
- Enums and `match`.
- Traits and trait bounds at a basic level.
- Error handling with `expect`.
- Running shell pipelines.

The example used in this module is:

- `source-code/random-numbers`

## Why Reproducibility Matters

Random numbers are often used to explore variability, initialize simulations,
or generate test data. In those settings, reproducibility matters because a run
should be repeatable when investigating results.

A seed is the input that initializes the random number generator. With the same
RNG algorithm, same seed, and same sequence of sampling calls, the program
produces the same stream of values.

Run the example twice with the same seed:

```bash
cd source-code/random-numbers
cargo run -- --count 5 --seed 42 --distribution uniform
cargo run -- --count 5 --seed 42 --distribution uniform
```

The output should be the same. Changing the seed changes the stream:

```bash
cargo run -- --count 5 --seed 43 --distribution uniform
```

This is the basic reproducibility contract used throughout the module.

## Choosing An RNG

The example uses `ChaCha12Rng`:

```rust
use rand_chacha::ChaCha12Rng;
```

The RNG is constructed from a command-line seed:

```rust
let mut rng = ChaCha12Rng::seed_from_u64(args.seed);
```

Using a named RNG algorithm is important for reproducibility. If a program uses
an unspecified default RNG, its exact behavior may change across crate versions
or platforms. A named RNG makes the intended stream more explicit.

The RNG is mutable because sampling advances its internal state.

## Command-Line Parameters

The example exposes three command-line parameters:

```rust
#[derive(Parser, Debug)]
#[command(author, version, about="Random number generator")]
struct Args {
    /// The number of random numbers to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// The seed for the random number generator
    #[arg(short, long, default_value_t = 1234)]
    seed: u64,

    /// The distribution to sample from
    #[arg(short, long, default_value = "uniform")]
    distribution: DistributionKind,
}
```

This makes random generation configurable from the shell:

```bash
cargo run -- --count 5 --seed 42 --distribution normal
```

For scientific examples, the seed should be treated as part of the run
configuration. It belongs in command-line arguments, configuration files, logs,
or output metadata.

## CLI Choices With `ValueEnum`

The available distribution choices are represented by an enum:

```rust
#[derive(Clone, ValueEnum, Debug)]
enum DistributionKind {
    Uniform,
    Normal,
}
```

Because it derives `ValueEnum`, `clap` can parse these values from the command
line and show them in `--help` output:

```bash
cargo run -- --help
```

This keeps accepted values explicit. The program does not need to manually
parse arbitrary strings such as `"uniform"` or `"normal"`.

## Runtime Distribution Objects

The command-line enum describes the user's choice, but the actual sampling code
needs concrete distribution objects from the `rand_distr` crate:

```rust
enum RealDistribution {
    Uniform(Uniform<f64>),
    Normal(Normal<f64>),
}
```

The conversion from command-line choice to runtime distribution object happens
in one place:

```rust
impl RealDistribution {
    fn from_kind(kind: DistributionKind) -> Self {
        match kind {
            DistributionKind::Uniform => {
                Self::Uniform(Uniform::new(0.0, 1.0).expect("valid uniform distribution"))
            }
            DistributionKind::Normal => {
                Self::Normal(Normal::new(0.0, 1.0).expect("valid normal distribution"))
            }
        }
    }
}
```

This is a useful design pattern:

- one type represents the CLI choice;
- another type represents the runtime object used by the computation.

The uniform distribution samples values in the half-open interval `[0.0, 1.0)`.
The normal distribution uses mean `0.0` and standard deviation `1.0`.

## Sampling From A Distribution

The distribution enum provides a common sampling method:

```rust
fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
    match self {
        Self::Uniform(distribution) => distribution.sample(rng),
        Self::Normal(distribution) => distribution.sample(rng),
    }
}
```

The method is generic over any RNG that implements `Rng`:

```rust
R: Rng + ?Sized
```

At this stage, the important idea is that the distribution does not need to
know the exact RNG type. It only needs an RNG that supports the behavior
required for sampling.

The `?Sized` bound allows the method to work with RNG values that may be used
behind references or trait objects. It is not central to the example, but it
matches common Rust library patterns.

## Generating A Stream Of Values

The `main` function ties the pieces together:

```rust
let args = Args::parse();
let mut rng = ChaCha12Rng::seed_from_u64(args.seed);
let distribution = RealDistribution::from_kind(args.distribution);

for _ in 0..args.count {
    let random_number = distribution.sample(&mut rng);
    println!("{}", random_number);
}
```

Each call to `sample` advances the RNG and prints one value. The program writes
plain text to standard output, one sample per line. This makes it easy to
inspect directly or pipe into another program.

## Visualizing A Distribution

The example includes a Python helper script:

```text
source-code/random-numbers/show-distribution.py
```

It reads numbers from standard input and displays a Plotly histogram. Use it
with a shell pipeline:

```bash
cargo run -- --count 1000 --seed 42 --distribution normal | ./show-distribution.py
```

The Rust program generates the data. The Python script visualizes the data.
This separation keeps the Rust example focused on reproducible generation and
uses Python for interactive plotting.

The script requires Plotly:

```bash
python3 -m pip install plotly
```

## Randomness And Program Design

For small examples, constructing the RNG directly in `main` is fine. For larger
programs, it is often better to pass an RNG into functions that need random
values.

For example, the design used in the distribution method:

```rust
fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64
```

makes the source of randomness explicit. The function does not create its own
hidden RNG. That improves reproducibility because the caller controls the seed
and the order of random draws.

This design habit is especially important for simulations. Hidden random
number generators make it harder to reproduce a result or explain why two runs
differ.

## Suggested Hands-On Work

Use this sequence as a practical lab.

1. Run the example with the default settings:

   ```bash
   cd source-code/random-numbers
   cargo run
   ```

2. Generate five uniform samples with an explicit seed:

   ```bash
   cargo run -- --count 5 --seed 42 --distribution uniform
   ```

3. Run the same command again and verify that the output is identical.

4. Change only the seed and compare the output.

5. Generate normally distributed samples:

   ```bash
   cargo run -- --count 5 --seed 42 --distribution normal
   ```

6. Run `cargo run -- --help` and inspect the accepted distribution values.

7. Change the normal distribution parameters in `RealDistribution::from_kind`,
   for example to mean `10.0` and standard deviation `2.0`.

8. Add a new distribution choice to `DistributionKind`, such as an exponential
   distribution from `rand_distr`, then add the corresponding variant to
   `RealDistribution`.

9. Generate a larger sample and visualize it:

   ```bash
   cargo run -- --count 1000 --seed 42 --distribution normal | ./show-distribution.py
   ```

10. Record the command used to generate a plot and rerun it to confirm that the
    same seed reproduces the same data.

## Discussion Points

This module is a good place to emphasize:

- Reproducibility requires controlling the seed and RNG algorithm.
- Randomness should be part of the run configuration, not hidden global state.
- The RNG is mutable because sampling advances its state.
- CLI enums make accepted random-distribution choices explicit.
- Separating CLI choices from runtime distribution objects keeps the design
  flexible.
- Text output and shell pipelines are simple but powerful for generated data.
- Visualization is useful for sanity-checking random samples.

## Connection To Later Modules

Randomness appears in larger examples as input generation and initialization:

- The `hashmap-hashset` example uses seeded randomness to generate and corrupt
  DNA-like sequence data.
- Julia set examples do not require randomness, which makes them useful for
  deterministic numerical comparison.
- The N-body example uses random initialization, where seeds and distribution
  choices are important for reproducible simulations.

Once participants are comfortable with reproducible random generation, they are
ready to study the integrated Julia set example and then the larger simulation
example.
