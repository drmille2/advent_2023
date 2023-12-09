use clap::Parser;
use std::{collections::VecDeque, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 9)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

type Series = VecDeque<isize>;

fn calc_deriv(x: &VecDeque<isize>) -> Series {
    let mut out = VecDeque::new();
    for n in 1..x.len() {
        out.push_back(x[n] - x[n - 1]);
    }
    out
}

fn rextend_series(x: &mut Series) {
    // if x.into_iter().windows(2).all(|x| x[0] == x[1]) {
    if x.iter().all(|a| a == &x[0]) {
        x.push_back(x[0]);
        return;
    }

    let mut d = calc_deriv(x);
    rextend_series(&mut d);
    let next = *x.iter().last().unwrap() + d.into_iter().last().unwrap();
    x.push_back(next);
}

// fn lextend_series(x: &mut Series) {
//     if x.windows(2).all(|x| x[0] == x[1]) {
//         x.push(x[0]);
//         return;
//     }

//     let mut d = calc_deriv(x);
//     lextend_series(&mut d);
//     let next = x.last().unwrap() + *d.last().unwrap();
//     x.push(next);
// }

fn parse_input(s: &str) -> Vec<Series> {
    let mut out = Vec::new();
    for line in s.split_terminator('\n') {
        let mut series = VecDeque::new();
        for num in line.split(' ') {
            series.push_back(num.parse::<isize>().unwrap());
        }
        out.push(series);
    }
    out
}

fn solve_part1(s: &str) -> isize {
    let all_series = parse_input(s);
    let mut out = 0;
    for mut series in all_series {
        rextend_series(&mut series);
        out += series.into_iter().last().unwrap();
    }
    // let mut series = all_series[2].clone();
    // println!("series {:?}", series);
    out
}

// fn solve_part2(s: &str) -> usize {
//     0
// }

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    // println!("Part 2: {}", solve_part2(input));
}
