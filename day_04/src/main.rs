use clap::Parser;
use std::{
    cmp::min,
    collections::{BTreeMap, HashMap},
    fs,
};

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

#[derive(Debug)]
struct Card {
    _number: usize,
    _winning: Vec<usize>,
    _actual: Vec<usize>,
    won: HashMap<usize, usize>,
}

impl Card {
    fn new(s: &str) -> Self {
        let mut won = HashMap::new();

        let (pre, suf) = s.split_once(':').unwrap();
        let (win, act) = suf.split_once('|').unwrap();
        let (_, num) = pre.split_once(' ').unwrap();

        let winning: Vec<usize> = win
            .trim()
            .split(' ')
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let actual: Vec<usize> = act
            .trim()
            .split(' ')
            .filter(|x| !x.trim().is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let number = num.trim().parse::<usize>().unwrap();

        for n in winning.iter() {
            won.insert(*n, 0);
        }
        for n in actual.iter() {
            won.entry(*n).and_modify(|e| *e += 1);
        }

        Card {
            _number: number,
            _winning: winning,
            _actual: actual,
            won,
        }
    }

    fn points(&self) -> usize {
        let exp: u32 = self.won.values().sum::<usize>() as u32;
        if exp == 0 {
            0
        } else {
            let base: usize = 2;
            let score = base.pow(exp - 1);
            score
        }
    }
}
fn solve_part1(s: &str) -> usize {
    let mut out = 0;
    for row in s.split_terminator('\n') {
        let card = Card::new(row);
        out += card.points();
    }
    out
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
