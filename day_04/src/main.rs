use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 4)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

fn solve_part1(s: &str) -> u32 {
    0
}

fn solve_part2(s: &str) -> u32 {
    0
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
