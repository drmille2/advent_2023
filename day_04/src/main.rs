use clap::Parser;
use std::{collections::HashMap, fs};

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

#[derive(Debug, Clone)]
struct Card {
    number: usize,
    won: HashMap<usize, usize>,
    score: usize,
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

        let score = won.clone().values().sum();

        Card { number, won, score }
    }

    fn points(&self) -> usize {
        let exp: u32 = self.won.values().sum::<usize>() as u32;
        if exp == 0 {
            0
        } else {
            let base: usize = 2;
            base.pow(exp - 1)
        }
    }
}

fn scratch_cards(
    card_counts: &mut HashMap<usize, usize>,
    card_lookup: HashMap<usize, Card>,
) -> usize {
    let mut cards_scratched = 0;
    for num in 1..=card_lookup.len() {
        let card = card_lookup.get(&(num)).unwrap();
        for _ in 0..*card_counts.get(&num).unwrap_or(&0) {
            card_counts.entry(num).and_modify(|x| *x -= 1);
            for idx in num..(num + card.score) {
                card_counts.entry(idx + 1).and_modify(|x| *x += 1);
            }
            cards_scratched += 1;
        }
    }
    cards_scratched
}

fn solve_part1(s: &str) -> usize {
    let mut out = 0;
    for row in s.split_terminator('\n') {
        let card = Card::new(row);
        out += card.points();
    }
    out
}

fn solve_part2(s: &str) -> usize {
    let mut card_lookup = HashMap::new();
    let mut card_counts = HashMap::new();

    for row in s.split_terminator('\n') {
        let card = Card::new(row);
        card_lookup.insert(card.number, card.clone());
        card_counts.insert(card.number, 1);
    }

    scratch_cards(&mut card_counts, card_lookup)
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
