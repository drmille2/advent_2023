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

#[derive(Debug)]
struct Span {
    start: i64,
    end: i64,
    offset: i64,
}

#[derive(Debug)]
struct ItemMap {
    spans: Vec<Span>,
}

impl Span {
    fn new(s: &str) -> Self {
        // println!("creating new span from {}", s);
        let (dst, rem) = s.split_once(' ').unwrap();
        let (src, len) = rem.split_once(' ').unwrap();

        let start = src.parse::<i64>().unwrap();
        let end = start + len.parse::<i64>().unwrap();
        let offset = dst.parse::<i64>().unwrap() - start;

        Span { start, end, offset }
    }

    fn contains(&self, n: i64) -> bool {
        n >= self.start && n <= self.end
    }

    fn get(&self, n: i64) -> i64 {
        // if !self.contains(n) {
        //     return None;
        // }

        // Some(n + self.offset)
        n + self.offset
    }
}

impl ItemMap {
    fn new(s: &str) -> Self {
        let mut spans = Vec::new();
        // println!("creating new item map from {}", s);
        for row in s.split_terminator('\n') {
            if !row.contains(':') && !row.trim().is_empty() {
                spans.push(Span::new(row))
            }
        }
        ItemMap { spans }
    }

    // fn contains(&self, n: i64) -> bool {
    //     self.spans.iter().any(|x| x.contains(n))
    // }

    // fn get(&self, n: i64) -> i64 {
    //     self.spans.iter().fold(n, |acc, x| x.get(acc))
    // }

    fn get(&self, n: i64) -> i64 {
        for s in self.spans.iter() {
            if s.contains(n) {
                return s.get(n);
            }
        }
        n
    }
}

fn parse_input(s: &str) -> (Vec<i64>, Vec<ItemMap>) {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    let (seeds_str, rem) = s.split_once('\n').unwrap();
    let (_, seed_nums) = seeds_str.split_once(' ').unwrap();
    for n in seed_nums.split(' ') {
        seeds.push(n.parse::<i64>().unwrap());
    }

    for map_str in rem.split("\n\n") {
        if !map_str.trim().is_empty() {
            let imap = ItemMap::new(map_str);
            println!("New Item Map: {:?}", imap);
            maps.push(ItemMap::new(map_str));
        }
    }

    (seeds, maps)
}

fn solve_part1(s: &str) -> i64 {
    let (seeds, maps) = parse_input(s);
    let mut starting_locs = Vec::new();
    for seed in seeds {
        starting_locs.push((seed, maps.iter().fold(seed, |acc, x| x.get(acc))));
    }
    println!("starting locations: {:?}", starting_locs);
    let min = starting_locs
        .into_iter()
        .reduce(|acc, e| if e.1 < acc.1 { e } else { acc });
    min.unwrap().1
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
