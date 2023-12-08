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

const OAK_5: u32 = 9 * BASE.pow(6);
const OAK_4: u32 = 8 * BASE.pow(6);
const FULLH: u32 = 7 * BASE.pow(6);
const OAK_3: u32 = 6 * BASE.pow(6);
const TWOPR: u32 = 5 * BASE.pow(6);
const OAK_2: u32 = 4 * BASE.pow(6);

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
    Joker = 0,
}

#[derive(Debug)]
struct Hand {
    _cards: Vec<Card>,
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
    fn new(s: &str, p: usize) -> Self {
        let (cards_str, bet_str) = s.split_once(' ').unwrap();
        let bet = bet_str.parse::<usize>().unwrap();
        let mut cards = Vec::new();
        for c in cards_str.chars() {
            cards.push(Card::new(c).unwrap())
        }
        if p == 2 {
            cards = jokerize(cards);
        }
        let value = hand_value(&cards) as usize + sum_card_value(&cards);
        Hand {
            _cards: cards,
            bet,
            value,
        }
    }
}

fn hand_value(cards: &[Card]) -> u32 {
    let counts = count_cards(cards);
    let num_jokers = *counts.get(&Card::Joker).unwrap_or(&0);

    if is_n_ofakind(cards, 5) || num_jokers == 5 {
        return OAK_5;
    }

    if is_n_ofakind(cards, 4) {
        if num_jokers > 0 {
            return OAK_5;
        } else {
            return OAK_4;
        }
    }

    if is_fullhouse(cards) {
        return FULLH;
    }

    if is_n_ofakind(cards, 3) {
        match num_jokers {
            1 => return OAK_4,
            2 => return OAK_5,
            _ => return OAK_3,
        };
    }

    if is_twopair(cards) {
        if num_jokers > 0 {
            return FULLH;
        } else {
            return TWOPR;
        }
    }

    if is_n_ofakind(cards, 2) {
        match num_jokers {
            1 => return OAK_3,
            2 => return OAK_4,
            3 => return OAK_5,
            _ => return OAK_2,
        };
    }

    match num_jokers {
        1 => OAK_2,
        2 => OAK_3,
        3 => OAK_4,
        4 => OAK_5,
        _ => 0,
    }
}

fn sum_card_value(cards: &[Card]) -> usize {
    let mut out = 0;
    let mut card_val;
    for (idx, card) in cards.iter().rev().enumerate() {
        card_val = card.to_u32().unwrap();
        out += BASE.pow(idx as u32) * card_val;
    }
    out as usize
}

fn jokerize(cards: Vec<Card>) -> Vec<Card> {
    cards
        .into_iter()
        .map(|c| if c == Card::J { Card::Joker } else { c })
        .collect()
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
    counts
        .into_iter()
        .filter(|c| c.0 != &Card::Joker)
        .filter(|c| c.1 == n)
        .count()
        > 0
}

fn is_fullhouse(cards: &[Card]) -> bool {
    is_n_ofakind(cards, 2) && is_n_ofakind(cards, 3)
}

fn is_twopair(cards: &[Card]) -> bool {
    let counts = count_cards(cards);
    counts
        .into_iter()
        .filter(|c| c.0 != &Card::Joker)
        .filter(|c| c.1 == 2)
        .count()
        == 2
}

fn solve_part1(s: &str) -> usize {
    let mut hands = Vec::new();
    for h in s.split_terminator('\n') {
        hands.push(Hand::new(h, 1))
    }
    hands.sort();

    let mut out = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        out += (idx + 1) * hand.bet
    }
    out
}

fn solve_part2(s: &str) -> usize {
    let mut hands = Vec::new();
    for h in s.split_terminator('\n') {
        hands.push(Hand::new(h, 2))
    }
    hands.sort();

    let mut out = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        out += (idx + 1) * hand.bet
    }
    out
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
