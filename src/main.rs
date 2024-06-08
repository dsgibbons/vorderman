use clap::Parser;
use std::time::{self, Instant};
use vorderman::round::NumbersRound;
use vorderman::search::search;

/// Generate and solve a random numbers round.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    smalls: u8,
}

fn main() {
    let args = Args::parse();

    let numbers_round = NumbersRound::new(args.smalls).unwrap();

    println!("{:?}", numbers_round);

    let now = Instant::now();
    let solution = search(numbers_round).unwrap();

    println!(
        "Found solution: {} in {} seconds.",
        solution.0,
        now.elapsed().as_secs_f32()
    );
}
