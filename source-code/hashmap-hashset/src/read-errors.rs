use clap::Parser;
use hashmap_hashset::{ERROR_TOKENS, is_valid_nucleotide};
use rand::{RngExt, SeedableRng, seq::IndexedRandom};
use rand_chacha::ChaCha12Rng;
use std::io::{BufReader, Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about = "introduce read errors into file")]
struct Args {
    /// The file to read
    #[arg(short, long)]
    file: String,
    /// The probability of a read error (0.0 to 1.0)
    #[arg(short, long, default_value_t = 0.1)]
    error_rate: f64,
    /// The file to write the output to
    #[arg(short, long)]
    output: String,
    /// The seed for the random number generator
    #[arg(short, long, default_value_t = 1234)]
    seed: u64,
}

fn main() {
    let args = Args::parse();
    let mut rng = ChaCha12Rng::seed_from_u64(args.seed);
    let input_file = std::fs::File::open(args.file).expect("Unable to open file");
    let input = BufReader::new(input_file);
    let output_file = std::fs::File::create(args.output).expect("Unable to create output file");
    let mut output = std::io::BufWriter::new(output_file);

    for byte in input.bytes() {
        let nucleotide = byte.expect("Unable to read file") as char;
        if is_valid_nucleotide(nucleotide) && rng.random_bool(args.error_rate) {
            let error_token = ERROR_TOKENS
                .choose(&mut rng)
                .expect("error token list should not be empty");
            write!(output, "{error_token}").expect("Unable to write file");
        } else {
            write!(output, "{nucleotide}").expect("Unable to write file");
        }
    }
}
