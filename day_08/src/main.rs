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

type Place = Vec<char>;
type Map = HashMap<Place, (Place, Place)>;

#[derive(Debug, Copy, Clone)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct Atlas {
    map: Map,
    loc: Place,
    locs: Vec<Place>,
}

impl Dir {
    fn new(c: char) -> Option<Self> {
        match c {
            'L' => Some(Dir::L),
            'R' => Some(Dir::R),
            _ => None,
        }
    }
}

fn travel(map: &Map, origin: Place, direction: Dir) -> Place {
    let next = map.get(&origin).unwrap().clone();
    // println!("travelling from {:?} in direction {:?}", self.loc, m);
    match direction {
        Dir::L => next.0,
        Dir::R => next.1,
    }
}

fn parse_input(s: &str) -> (Map, Vec<Dir>) {
    let (first, rem) = s.split_once('\n').unwrap();
    let moves = first.chars().map(|c| Dir::new(c).unwrap()).collect();

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

fn calculate_route(map: &Map, origin: Place, end: Place, route: Vec<Dir>) -> usize {
    let num_moves = &route.len();
    let mut i = 0;
    let mut i_tot = 0;
    let mut location: Place = origin;

    loop {
        if &i == num_moves {
            i = 0
        };
        let direction = route[i];
        location = travel(map, location, direction);
        i += 1;
        i_tot += 1;
        if location == end {
            println!("stopping at {:?}", location);
            break;
        }
    }
    i_tot
}

// fn is_all_end(s: &[Place]) -> bool {
//     s.iter().map(|e| e.last().unwrap()).all(|c| c == &'Z')
// }

fn solve_part1(s: &str) -> usize {
    let (map, route) = parse_input(s);
    calculate_route(&map, vec!['A', 'A', 'A'], vec!['Z', 'Z', 'Z'], route)
}

// fn solve_part2(s: &str) -> usize {
//     let (map, moves) = parse_input(s);
//     let mut i = 0;
//     let mut i_tot = 0;
//     let map_clone = map.clone();
//     let locs: Vec<Element> = map_clone
//         .keys()
//         .filter(|t| *t.last().unwrap() == 'A')
//         .cloned()
//         .collect();
//     let mut atlas = Atlas {
//         map,
//         loc: Vec::new(),
//         locs,
//     };
//     let num_moves = &moves.len();
//     loop {
//         if &i == num_moves {
//             i = 0
//         };
//         let m = moves[i];
//         atlas.travel_all(m);
//         i += 1;
//         i_tot += 1;
//         if is_all_end(&atlas.locs) {
//             println!("stopping at {:?}", atlas.locs);
//             break;
//         }
//     }
//     i_tot
// }

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    // println!("Part 2: {}", solve_part2(input));
}
