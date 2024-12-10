use clap::Parser;

mod days;


use days::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: usize,
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        // Add more days here
        _ => eprintln!("Day {} is not implemented yet!", cli.day),
    }
}
