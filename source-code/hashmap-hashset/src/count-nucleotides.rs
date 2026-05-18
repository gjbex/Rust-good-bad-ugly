use clap::Parser;
use hashmap_hashset::{VALID_NUCLEOTIDES, is_valid_nucleotide};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Read};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Count the number of each nucleotide in a DNA sequence."
)]
struct Args {
    /// The file containing the DNA sequence
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut counts = HashMap::new();
    let mut error_tokens = HashSet::new();
    let file = std::fs::File::open(args.file).expect("Failed to open the DNA sequence file");
    let reader = BufReader::new(file);

    for byte in reader.bytes() {
        let nucleotide = byte.expect("Failed to read the DNA sequence file") as char;
        match nucleotide {
            nucleotide if is_valid_nucleotide(nucleotide) => {
                *counts.entry(nucleotide).or_insert(0) += 1;
            }
            nucleotide if nucleotide.is_whitespace() => {}
            _ => {
                error_tokens.insert(nucleotide);
            }
        }
    }

    for nucleotide in VALID_NUCLEOTIDES {
        counts.entry(nucleotide).or_insert(0);
        println!("{nucleotide}: {}", counts[&nucleotide]);
    }
    eprintln!("Error tokens: {error_tokens:?}");
}
