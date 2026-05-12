use clap::{Parser, ValueEnum};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;
use rand_distr::{Distribution, Normal, Uniform};

#[derive(Clone, ValueEnum, Debug)]
enum DistributionKind {
    Uniform,
    Normal,
}

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

enum RealDistribution {
    Uniform(Uniform<f64>),
    Normal(Normal<f64>),
}

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

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        match self {
            Self::Uniform(distribution) => distribution.sample(rng),
            Self::Normal(distribution) => distribution.sample(rng),
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut rng = ChaCha12Rng::seed_from_u64(args.seed);
    let distribution = RealDistribution::from_kind(args.distribution);
    for _ in 0..args.count {
        let random_number = distribution.sample(&mut rng);
        println!("{}", random_number);
    }
}
