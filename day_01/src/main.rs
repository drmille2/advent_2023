use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 1)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

fn replace_num_words(s: &str) -> String {
    s.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

fn contains_num_word(s: &str) -> bool {
    s.contains("one")
        || s.contains("two")
        || s.contains("three")
        || s.contains("four")
        || s.contains("five")
        || s.contains("six")
        || s.contains("seven")
        || s.contains("eight")
        || s.contains("nine")
}

fn extract_int_from_string(s: &str) -> u32 {
    s.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn get_written_digit(s: &str) -> u32 {
    extract_int_from_string(&replace_num_words(s))
}

fn get_first_digit(s: &str) -> u32 {
    let mut n = 1;
    while n <= s.len() {
        if let Some(num) = s.chars().nth(n - 1).unwrap().to_digit(10) {
            return num;
        }
        match contains_num_word(&s[0..n]) {
            true => {
                return get_written_digit(&s[0..n]);
            }
            false => n += 1,
        }
    }
    println!("something went wrong, no first digit found");
    0
}

fn get_last_digit(s: &str) -> u32 {
    let mut n = 1;
    while n <= s.len() {
        if let Some(num) = s.chars().nth(s.len() - n).unwrap().to_digit(10) {
            return num;
        }
        match contains_num_word(&s[s.len() - n..]) {
            true => {
                return get_written_digit(&s[s.len() - n..]);
            }
            false => n += 1,
        }
    }
    0
}

fn line_to_num(s: &str) -> u32 {
    let n: String = s.chars().filter(|c| c.is_numeric()).collect();

    // handle lines with only 1 number correctly
    if n.len() == 1 {
        let m = n.parse::<u32>().unwrap();
        return 10 * m + m;
    }

    n.chars()
        .enumerate()
        .filter(|c| (c.0 == 0) || (c.0 == n.len() - 1))
        .map(|c| c.1)
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn parse_input(s: &str) -> Vec<&str> {
    s.split_terminator('\n').collect()
}

fn solve_part1(s: &str) -> u32 {
    parse_input(s).into_iter().map(line_to_num).sum()
}

fn solve_part2(s: &str) -> u32 {
    parse_input(s)
        .into_iter()
        .map(|l| 10 * get_first_digit(l) + get_last_digit(l))
        .sum()
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
