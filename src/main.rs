use clap::Parser;
use std::time::Instant;
use vorderman::round::NumbersRound;
use vorderman::solver::find_solution;

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

    let now = Instant::now();
    let solution = find_solution(numbers_round, false);
    let time_taken = now.elapsed().as_secs_f32();

    match solution {
        Some(s) => {
            println!("Found solution: {} in {} seconds.", s.0, time_taken,);
        }
        None => {
            println!(
                "No solutions exist. Search complete in {} seconds.",
                time_taken,
            );
        }
    };
}
