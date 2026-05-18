use clap::Parser;
use hashmap_hashset::VALID_NUCLEOTIDES;
use rand::{SeedableRng, seq::IndexedRandom};
use rand_chacha::ChaCha12Rng;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about = "Random number generator")]
struct Args {
    /// The number of nucleotides to generate
    #[arg(short, long, default_value_t = 800)]
    count: usize,
    /// The seed for the random number generator
    #[arg(short, long, default_value_t = 1234)]
    seed: u64,
    /// The file name to write the random numbers to
    #[arg(short, long, default_value = "data.txt")]
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut rng = ChaCha12Rng::seed_from_u64(args.seed);
    let file = std::fs::File::create(args.file).expect("Unable to create file");
    let mut output = std::io::BufWriter::new(file);
    let mut line_counter = 0;

    for _ in 0..args.count {
        let random_nucleotide = VALID_NUCLEOTIDES
            .choose(&mut rng)
            .expect("valid nucleotide list should not be empty");
        write!(output, "{random_nucleotide}").expect("Unable to write file");
        line_counter += 1;

        if line_counter == 80 {
            writeln!(output).expect("Unable to write file");
            line_counter = 0;
        }
    }
}
