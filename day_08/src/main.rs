use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 8)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

// type Element = [char; 3];
type Element = Vec<char>;
type Map = HashMap<Element, (Element, Element)>;

#[derive(Debug, Copy, Clone)]
enum Move {
    L,
    R,
}

fn travel(s: &Element, n: &Map, m: Move) -> Element {
    let (l, r) = n.get(s).unwrap();
    println!("travelling from {:?} in direction {:?}", s, m);
    match m {
        Move::L => l.clone(),
        Move::R => r.clone(),
    }
}

impl Move {
    fn new(c: char) -> Option<Self> {
        match c {
            'L' => Some(Move::L),
            'R' => Some(Move::R),
            _ => None,
        }
    }
}

fn parse_input(s: &str) -> (Map, Vec<Move>) {
    let (first, rem) = s.split_once('\n').unwrap();
    let moves = first.chars().map(|c| Move::new(c).unwrap()).collect();

    let mut map = HashMap::new();
    for line in rem.split_terminator('\n') {
        if let Some((node, rem)) = line.split_once('=') {
            let node_name: Vec<char> = node.trim().chars().collect();
            let (l_str, r_str) = rem.split_once(',').unwrap();
            map.insert(
                node_name,
                (
                    l_str.trim().replace('(', "").chars().collect(),
                    r_str.trim().replace(')', "").chars().collect(),
                ),
            );
        }
    }

    (map, moves)
}

fn solve_part1(s: &str) -> usize {
    let (map, moves) = parse_input(s);
    let mut i = 0;
    let mut i_tot = 0;
    let mut loc: Element = vec!['A', 'A', 'A'];
    let num_moves = &moves.len();
    loop {
        if &i == num_moves {
            i = 0
        };
        let m = moves[i];
        loc = travel(&loc, &map, m);
        i += 1;
        i_tot += 1;
        if loc == vec!['Z', 'Z', 'Z'] {
            println!("stopping at {:?}", loc);
            break;
        }
    }
    i_tot
}

fn solve_part2(s: &str) -> usize {
    0
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
