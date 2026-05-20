use clap::Parser;
use rand::{RngExt, SeedableRng};
use smart_pointers::bst::{insert, search};
use smart_pointers::tree::Tree;
use std::collections::BTreeSet;

#[derive(Parser)]
#[clap(version, about = "build a binary search tree")]
struct Args {
    #[clap(short, long, default_value = "20")]
    num_elements: usize,
    #[clap(short, long, default_value = "42")]
    seed: u64,
}

fn main() {
    let args = Args::parse();
    let mut tree = Tree::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(args.seed);
    let max_value = 5 * args.num_elements as u64;
    let mut values = BTreeSet::new();
    for _ in 0..args.num_elements {
        let value = rng.random_range(0..max_value);
        insert(&mut tree, value);
        values.insert(value);
    }
    println!("{tree}");
    let mut values_found = BTreeSet::new();
    for value in 0..max_value {
        if search(&tree, &value) {
            values_found.insert(value);
        }
    }
    println!("{:?} found", values_found);
    println!("{:?} expected", values);
}
