use clap::Parser;
use enum_primitive_derive::Primitive;
use num_traits::ToPrimitive;
use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 7)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

const BASE: u32 = 16;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Primitive)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: usize,
    value: usize,
}

impl Card {
    fn new(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Hand {}

impl Hand {
    fn new(s: &str) -> Self {
        let (cards_str, bet_str) = s.split_once(' ').unwrap();
        let bet = bet_str.parse::<usize>().unwrap();
        let mut cards = Vec::new();
        for c in cards_str.chars() {
            cards.push(Card::new(c).unwrap())
        }
        let value = hand_value(&cards) as usize + sum_card_value(&cards);
        Hand { cards, bet, value }
    }
}

fn hand_value(cards: &[Card]) -> u32 {
    let exp: u32 = 6;

    if is_n_ofakind(cards, 5) {
        // println!("{:?} is five of a kind", cards);
        return 9 * BASE.pow(exp);
    }
    if is_n_ofakind(cards, 4) {
        // println!("{:?} is four of a kind", cards);
        return 8 * BASE.pow(exp);
    }
    if is_fullhouse(cards) {
        // println!("{:?} is full house", cards);
        return 7 * BASE.pow(exp);
    }
    if is_n_ofakind(cards, 3) {
        // println!("{:?} is three of a kind", cards);
        return 6 * BASE.pow(exp);
    }
    if is_twopair(cards) {
        // println!("{:?} is two pair", cards);
        return 5 * BASE.pow(exp);
    }
    if is_n_ofakind(cards, 2) {
        // println!("{:?} is two of a kind", cards);
        return 4 * BASE.pow(exp);
    }
    0
}

fn sum_card_value(cards: &[Card]) -> usize {
    let mut out = 0;
    for (idx, card) in cards.iter().rev().enumerate() {
        out += BASE.pow(idx as u32) * card.to_u32().unwrap();
    }
    // println!("sum of card values {:?} = {}", cards, out);
    out as usize
}

fn count_cards(cards: &[Card]) -> HashMap<&Card, usize> {
    let mut counts = HashMap::new();
    for c in cards.iter() {
        counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    counts
}

fn is_n_ofakind(cards: &[Card], n: usize) -> bool {
    let counts = count_cards(cards);
    counts.values().filter(|c| **c == n).count() > 0
}

fn is_fullhouse(cards: &[Card]) -> bool {
    let counts = count_cards(cards);
    is_n_ofakind(cards, 2) && is_n_ofakind(cards, 3)
}

fn is_twopair(cards: &[Card]) -> bool {
    let counts = count_cards(cards);
    let out = counts.into_values().filter(|v| v == &2).count() == 2;
    out
}

fn solve_part1(s: &str) -> usize {
    let mut hands = Vec::new();
    for h in s.split_terminator('\n') {
        hands.push(Hand::new(h))
    }
    hands.sort();

    println!("Hands (sorted): {:?}", hands);

    let mut out = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let hand_val = (idx + 1) * hand.bet;
        println!("rank {} hand: {:?} winnings: {}", idx + 1, hand, hand_val);
        out += (idx + 1) * hand.bet
    }
    out
}

fn solve_part2(s: &str) -> i64 {
    0
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
