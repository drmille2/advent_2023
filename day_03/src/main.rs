use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 3)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Schematic {
    rows: Vec<String>,
    width: usize,
    parts: Vec<Part>,
}

#[derive(Debug)]
struct Part {
    val: usize,
    start_pos: Coord,
    end_pos: Coord,
}

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        self.is_ascii_digit() || *self == '.'
    }
}

impl Schematic {
    fn new(s: &str) -> Self {
        let mut rows = Vec::new();
        let mut parts = Vec::new();
        for (num, row) in s.split_terminator('\n').enumerate() {
            rows.push(String::from(row));
            let mut p = Schematic::parse_row(row, num);
            parts.append(&mut p);
        }
        let width = rows[0].clone().len();
        Schematic { rows, width, parts }
    }

    fn parse_row(row: &str, row_num: usize) -> Vec<Part> {
        let mut parts = Vec::new();
        let mut part = Vec::new();
        for c in row.chars().enumerate() {
            if c.1.is_ascii_digit() {
                part.push(c.1)
            }
            if !c.1.is_ascii_digit() && !part.is_empty() {
                parts.push(Part {
                    val: String::from_iter(part.clone()).parse::<usize>().unwrap(),
                    start_pos: (c.0 - part.len(), row_num),
                    end_pos: (c.0 - 1, row_num),
                });
                part = Vec::new();
            }
        }
        if !part.is_empty() {
            parts.push(Part {
                val: String::from_iter(part.clone()).parse::<usize>().unwrap(),
                start_pos: (row.len() - part.len(), row_num),
                end_pos: (row.len() - 1, row_num),
            });
        }
        parts
    }

    fn get_border(&self, start_pos: Coord, end_pos: Coord) -> (usize, usize, usize, usize) {
        // is start
        let start_row = if start_pos.1 == 0 { 0 } else { start_pos.1 - 1 };
        // is bottom
        let end_row = if start_pos.1 == self.rows.len() - 1 {
            self.rows.len() - 1
        } else {
            start_pos.1 + 1
        };
        // is left
        let start_col = if start_pos.0 == 0 { 0 } else { start_pos.0 - 1 };
        // is right
        let end_col = if end_pos.0 == self.width - 1 {
            self.width - 1
        } else {
            end_pos.0 + 1
        };

        (start_row, end_row, start_col, end_col)
    }

    fn get_adj_chars(&self, part_num: usize) -> String {
        let mut out = Vec::new();
        let (start_row, end_row, start_col, end_col) =
            self.get_border(self.parts[part_num].start_pos, self.parts[part_num].end_pos);
        for r in start_row..=end_row {
            for c in start_col..=end_col {
                if let Some(adj) = self.rows[r].chars().nth(c) {
                    out.push(adj)
                }
            }
        }
        String::from_iter(out)
    }

    fn get_adj_gears(&self, part_num: usize) -> Vec<Coord> {
        let mut out = Vec::new();
        let (start_row, end_row, start_col, end_col) =
            self.get_border(self.parts[part_num].start_pos, self.parts[part_num].end_pos);
        for r in start_row..=end_row {
            for c in start_col..=end_col {
                if let Some(adj) = self.rows[r].chars().nth(c) {
                    if adj == '*' {
                        out.push((c, r))
                    }
                }
            }
        }
        out
    }
}

fn solve_part1(s: &str) -> usize {
    let schem = Schematic::new(s);
    let mut out = 0;
    for (idx, part) in schem.parts.iter().enumerate() {
        if !schem.get_adj_chars(idx).chars().all(|c| c.is_symbol()) {
            out += part.val;
        } else {
        }
    }
    out
}

fn solve_part2(s: &str) -> usize {
    let schem = Schematic::new(s);
    let mut out = 0;
    let mut gears: HashMap<Coord, Vec<usize>> = HashMap::new();
    for idx in 0..schem.parts.len() {
        for coord in schem.get_adj_gears(idx) {
            gears
                .entry(coord)
                .and_modify(|t| t.push(idx))
                .or_insert(vec![idx]);
        }
    }
    for cnx in gears.into_values() {
        if cnx.len() == 2 {
            out += schem.parts[cnx[0]].val * schem.parts[cnx[1]].val
        }
    }
    out
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
