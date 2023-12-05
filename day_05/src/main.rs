use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 5)"
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
    index: i64,
}

#[derive(Debug)]
struct ItemMap {
    spans: Vec<Span>,
}

impl Span {
    fn new(s: &str) -> Self {
        let (dst, rem) = s.split_once(' ').unwrap();
        let (src, len) = rem.split_once(' ').unwrap();

        let start = src.parse::<i64>().unwrap();
        let end = start + len.parse::<i64>().unwrap();
        let offset = dst.parse::<i64>().unwrap() - start;

        Span {
            start,
            end,
            offset,
            index: 0,
        }
    }

    fn contains(&self, n: i64) -> bool {
        n >= self.start && n <= self.end
    }

    fn get(&self, n: i64) -> i64 {
        n + self.offset
    }
}

impl Iterator for Span {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let out = self.start + self.index;
        if out <= self.end {
            self.index += 1;
            Some(out)
        } else {
            None
        }
    }
}

impl ItemMap {
    fn new(s: &str) -> Self {
        let mut spans = Vec::new();
        for row in s.split_terminator('\n') {
            if !row.contains(':') && !row.trim().is_empty() {
                spans.push(Span::new(row))
            }
        }
        ItemMap { spans }
    }

    fn get(&self, n: i64) -> i64 {
        for s in self.spans.iter() {
            if s.contains(n) {
                return s.get(n);
            }
        }
        n
    }
}

fn parse_seeds_p1(s: &str) -> Vec<i64> {
    let mut seeds = Vec::new();
    let (_, seed_nums) = s.split_once(' ').unwrap();
    for n in seed_nums.split(' ') {
        seeds.push(n.parse::<i64>().unwrap());
    }
    seeds
}

fn parse_seeds_p2(s: &str) -> Vec<Span> {
    let mut seeds = Vec::new();
    let (_, seed_nums) = s.split_once(' ').unwrap();

    let mut start = 0;
    let mut end = 0;
    for n in seed_nums.split(' ') {
        let num = n.parse::<i64>().unwrap();
        if start == 0 {
            start = num;
        } else {
            end = start + num - 1;
        }
        if start != 0 && end != 0 {
            seeds.push(Span {
                start,
                end,
                offset: 0,
                index: 0,
            });
            start = 0;
            end = 0;
        }
    }

    seeds
}

fn parse_item_map(s: &str) -> Vec<ItemMap> {
    let mut maps = Vec::new();

    for map_str in s.split("\n\n") {
        if !map_str.trim().is_empty() {
            maps.push(ItemMap::new(map_str));
        }
    }

    maps
}

fn solve_part1(s: &str) -> i64 {
    let (seeds_str, rem) = s.split_once('\n').unwrap();
    let seeds = parse_seeds_p1(seeds_str);
    let maps = parse_item_map(rem);

    seeds
        .into_iter()
        .map(|s| maps.iter().fold(s, |acc, x| x.get(acc)))
        .min()
        .unwrap()
}

fn solve_part2(s: &str) -> i64 {
    let (seeds_str, rem) = s.split_once('\n').unwrap();
    let seeds = parse_seeds_p2(seeds_str);
    let maps = parse_item_map(rem);

    seeds
        .into_iter()
        .map(|span| {
            span.into_iter()
                .map(|s| maps.iter().fold(s, |acc, x| x.get(acc)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
