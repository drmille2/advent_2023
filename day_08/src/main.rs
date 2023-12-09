use clap::Parser;
use std::{cmp::max, collections::HashMap, fs};

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

fn calculate_route(map: &Map, origin: Place, end: Place, route: &Vec<Dir>) -> usize {
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
        if end.len() == 1 && location.last().unwrap() == &end[0] {
            break;
        }
        if location == end {
            break;
        }
    }
    i_tot
}

fn fine_whatever(n: usize) -> bool {
    let limit = f32::sqrt(n as f32) as usize + 1;
    for f in 2..=limit {
        if n % f == 0 {
            return false;
        }
    }
    true
}

fn ugh(n: usize, factors: &[usize]) -> HashMap<usize, usize> {
    let mut out: HashMap<usize, usize> = HashMap::new();
    for f in factors {
        if f == &n {
            break;
        }
        if n % f == 0 {
            out.entry(*f).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    out
}

fn add_factors(a: &mut HashMap<usize, usize>, b: &HashMap<usize, usize>) {
    for (&k, &v) in b {
        a.entry(k).and_modify(|f| *f = max(*f, v)).or_insert(v);
    }
}

fn solve_part1(s: &str) -> usize {
    let (map, route) = parse_input(s);
    calculate_route(&map, vec!['A', 'A', 'A'], vec!['Z', 'Z', 'Z'], &route)
}

fn solve_part2(s: &str) -> usize {
    let (map, route) = parse_input(s);
    let locations: Vec<Place> = map
        .keys()
        .filter(|t| *t.last().unwrap() == 'A')
        .cloned()
        .collect();

    let mut path_lengths = Vec::new();
    for l in locations {
        path_lengths.push(calculate_route(&map, l, vec!['Z'], &route));
    }

    let mut primes = Vec::new();
    for n in 1..=50000 {
        if fine_whatever(n) {
            primes.push(n);
        }
    }

    let mut factorizations = Vec::new();
    for l in path_lengths {
        factorizations.push(ugh(l, &primes));
    }

    let mut lcm: HashMap<usize, usize> = HashMap::new();
    for f in factorizations {
        add_factors(&mut lcm, &f);
    }
    lcm.keys().product()
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
