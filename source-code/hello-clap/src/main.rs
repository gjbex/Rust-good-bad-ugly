use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Say hello to someone", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    /// Whether to greet in uppercase
    #[arg(short, long, default_value_t = false)]
    uppercase: bool,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        if args.uppercase {
            println!("HELLO, {}!", args.name.to_uppercase());
        } else {
            println!("Hello, {}!", args.name);
        }
    }
}
