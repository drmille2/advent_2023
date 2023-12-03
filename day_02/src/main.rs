use clap::Parser;
use std::{collections::HashMap, fs};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 2)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, EnumIter)]
enum Cube {
    Red,
    Green,
    Blue,
}

type CubeSet = Vec<Cube>;
type Game = Vec<CubeSet>;

#[derive(Debug, Clone)]
struct Bag {
    colors: HashMap<Cube, usize>,
}

impl Bag {
    fn new(red: usize, green: usize, blue: usize) -> Self {
        let mut c: CubeSet = Vec::new();
        for _ in 1..red {
            c.push(Cube::Red);
        }
        for _ in 1..green {
            c.push(Cube::Green);
        }
        for _ in 1..blue {
            c.push(Cube::Blue);
        }
        Bag {
            colors: count_cubes(c),
        }
    }
    fn can_game(&self, g: Game) -> bool {
        g.into_iter()
            .map(|gr| self.can_grab(gr))
            .all(|prev| prev)
    }

    fn can_grab(&self, g: CubeSet) -> bool {
        let colors = count_cubes(g);
        for col in Cube::iter() {
            match col {
                Cube::Red => {
                    if colors.get(&Cube::Red) > self.colors.get(&Cube::Red) {
                        return false;
                    }
                }
                Cube::Green => {
                    if colors.get(&Cube::Green) > self.colors.get(&Cube::Green) {
                        return false;
                    }
                }
                Cube::Blue => {
                    if colors.get(&Cube::Blue) > self.colors.get(&Cube::Blue) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn bag_power(&self) -> usize {
        self.colors
            .clone()
            .into_values()
            .reduce(|acc, e| acc * e)
            .unwrap()
    }
}

fn count_cubes(c: CubeSet) -> HashMap<Cube, usize> {
    let mut colors = HashMap::new();
    for u in c.into_iter() {
        match u {
            Cube::Red => colors.entry(Cube::Red).and_modify(|t| *t += 1).or_insert(1),
            Cube::Green => colors
                .entry(Cube::Green)
                .and_modify(|t| *t += 1)
                .or_insert(1),
            Cube::Blue => colors
                .entry(Cube::Blue)
                .and_modify(|t| *t += 1)
                .or_insert(1),
        };
    }
    colors
}

fn min_bag(g: Game) -> Bag {
    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;

    for grab in g.into_iter() {
        let grab_colors = count_cubes(grab);
        if let Some(red) = grab_colors.get(&Cube::Red) {
            if red >= &reds {
                reds = *red
            };
        }
        if let Some(green) = grab_colors.get(&Cube::Green) {
            if green >= &greens {
                greens = *green
            };
        }
        if let Some(blue) = grab_colors.get(&Cube::Blue) {
            if blue >= &blues {
                blues = *blue
            };
        }
    }

    let mut colors = HashMap::new();
    colors.insert(Cube::Red, reds);
    colors.insert(Cube::Green, greens);
    colors.insert(Cube::Blue, blues);

    Bag { colors }
}

fn parse_grab(s: &str) -> CubeSet {
    let mut out = Vec::new();
    for cubes in s.split(',') {
        let (count, color) = cubes.trim().split_once(' ').unwrap();
        let count_int = count.parse::<usize>().unwrap();
        if color.contains("green") {
            for _ in 0..count_int {
                out.push(Cube::Green)
            }
        } else if color.contains("red") {
            for _ in 0..count_int {
                out.push(Cube::Red)
            }
        } else if color.contains("blue") {
            for _ in 0..count_int {
                out.push(Cube::Blue)
            }
        }
    }
    out
}

fn parse_input(s: &str) -> Vec<Game> {
    let mut out: Vec<Game> = Vec::new();
    for row in s.split_terminator('\n') {
        let mut game: Vec<CubeSet> = Vec::new();
        let trimmed = row.split_once(':').unwrap().1;
        for g in trimmed.split(';') {
            game.push(parse_grab(g));
        }
        out.push(game);
    }
    out
}

fn solve_part1(s: &str) -> usize {
    let bag = Bag::new(12, 13, 14);
    let mut index_sum = 0;
    for (index, game) in parse_input(s).into_iter().enumerate() {
        if bag.can_game(game) {
            index_sum += index + 1;
        }
    }
    index_sum
}

fn solve_part2(s: &str) -> usize {
    let mut powers = 0;
    for game in parse_input(s).into_iter() {
        let game_bag = min_bag(game);
        powers += game_bag.bag_power();
    }
    powers
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
