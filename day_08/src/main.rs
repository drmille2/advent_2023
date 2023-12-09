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

type Element = Vec<char>;
type Map = HashMap<Element, (Element, Element)>;

#[derive(Debug, Copy, Clone)]
enum Move {
    L,
    R,
}

#[derive(Debug)]
struct Atlas {
    map: Map,
    loc: Element,
    locs: Vec<Element>,
}

impl Atlas {
    fn travel(&mut self, m: Move) {
        let next = self.map.get(&self.loc).unwrap().clone();
        // println!("travelling from {:?} in direction {:?}", self.loc, m);
        match m {
            Move::L => self.loc = next.0,
            Move::R => self.loc = next.1,
        };
    }

    fn travel_all(&mut self, m: Move) {
        let mut locs = Vec::new();
        let loc_copy = self.locs.clone();
        for loc in loc_copy {
            let next = self.map.get(&loc).unwrap().clone();
            match m {
                Move::L => locs.push(next.0),
                Move::R => locs.push(next.1),
            };
        }
        // println!("travelling from {:?} in direction {:?}", self.locs, m);
        self.locs = locs;
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

fn is_all_end(s: &[Element]) -> bool {
    s.iter().map(|e| e.last().unwrap()).all(|c| c == &'Z')
}

fn solve_part1(s: &str) -> usize {
    let (map, moves) = parse_input(s);
    let mut i = 0;
    let mut i_tot = 0;
    let mut atlas = Atlas {
        map,
        loc: vec!['A', 'A', 'A'],
        locs: Vec::new(),
    };
    let num_moves = &moves.len();
    loop {
        if &i == num_moves {
            i = 0
        };
        let m = moves[i];
        atlas.travel(m);
        i += 1;
        i_tot += 1;
        if atlas.loc == vec!['Z', 'Z', 'Z'] {
            println!("stopping at {:?}", atlas.loc);
            break;
        }
    }
    i_tot
}

fn solve_part2(s: &str) -> usize {
    let (map, moves) = parse_input(s);
    let mut i = 0;
    let mut i_tot = 0;
    let map_clone = map.clone();
    let locs: Vec<Element> = map_clone
        .keys()
        .filter(|t| *t.last().unwrap() == 'A')
        .cloned()
        .collect();
    let mut atlas = Atlas {
        map,
        loc: Vec::new(),
        locs,
    };
    let num_moves = &moves.len();
    loop {
        if &i == num_moves {
            i = 0
        };
        let m = moves[i];
        atlas.travel_all(m);
        i += 1;
        i_tot += 1;
        if is_all_end(&atlas.locs) {
            println!("stopping at {:?}", atlas.locs);
            break;
        }
    }
    i_tot
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
