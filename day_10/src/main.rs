use clap::Parser;
use std::{fmt, fs};

#[derive(Parser, Debug)]
#[command(
    author = "David Miller",
    version = "v1.0.0",
    about = "Advent of Code (Day 10)"
)]

struct Cli {
    #[clap(short, long)]
    input: String,
}

type Coord = (usize, usize);

enum Pipe {
    Vertical,
    Horizontal,
    LowerLeft,
    UpperLeft,
    LowerRight,
    UpperRight,
    Ground,
    Start,
}

impl Pipe {
    fn new(c: char) -> Self {
        match c {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::LowerLeft,
            'J' => Pipe::LowerRight,
            '7' => Pipe::UpperRight,
            'F' => Pipe::UpperLeft,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => Pipe::Ground,
        }
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::Vertical => '║',
                Pipe::Horizontal => '═',
                Pipe::LowerLeft => '╚',
                Pipe::LowerRight => '╝',
                Pipe::UpperRight => '╗',
                Pipe::UpperLeft => '╔',
                Pipe::Ground => ' ',
                Pipe::Start => '◉',
            }
        )
    }
}

struct Diagram {
    tiles: Vec<Vec<Pipe>>,
    start: Coord,
    path: Vec<Coord>,
    verts: Vec<Coord>,
}

impl Diagram {
    fn new(s: &str) -> Self {
        let mut tiles = Vec::new();
        let mut start: Coord = (0, 0);
        for y in s.split_terminator('\n').enumerate() {
            let mut row = Vec::new();
            for x in y.1.chars().enumerate() {
                row.push(Pipe::new(x.1));
                if x.1 == 'S' {
                    start = (x.0, y.0);
                }
            }
            tiles.push(row);
        }
        Diagram {
            tiles,
            start,
            path: Vec::new(),
            verts: Vec::new(),
        }
    }

    fn get_pipe(&self, o: &Coord) -> Option<&Pipe> {
        if o.1 < self.tiles.len() && o.0 < (self.tiles[o.1].len()) {
            Some(&(self.tiles[o.1])[o.0])
        } else {
            None
        }
    }

    fn get_top(&self, c: &Coord) -> Option<Coord> {
        if c.1 > 0 {
            Some((c.0, c.1 - 1))
        } else {
            None
        }
    }

    fn get_bottom(&self, c: &Coord) -> Option<Coord> {
        if c.1 < self.tiles.len() {
            Some((c.0, c.1 + 1))
        } else {
            None
        }
    }

    fn get_left(&self, c: &Coord) -> Option<Coord> {
        if c.0 > 0 {
            Some((c.0 - 1, c.1))
        } else {
            None
        }
    }

    fn get_right(&self, c: &Coord) -> Option<Coord> {
        if c.0 < self.tiles[c.1].len() {
            Some((c.0 + 1, c.1))
        } else {
            None
        }
    }

    // rust point-in-polygon algorithm based on the Jordan Curve Theorem
    // adapted from C implementation found at https://wrfranklin.org/Research/Short_Notes/pnpoly.html
    fn is_interior(&self, o: &Coord) -> bool {
        let x = o.0 as isize;
        let y = o.1 as isize;
        let mut c = false;
        let mut i = 0;
        let mut j = self.verts.len() - 1;
        while i < self.verts.len() {
            let vertex_ix = self.verts[i].0 as isize;
            let vertex_jx = self.verts[j].0 as isize;
            let vertex_iy = self.verts[i].1 as isize;
            let vertex_jy = self.verts[j].1 as isize;
            if ((vertex_iy > y) != (vertex_jy > y))
                && (x
                    < (vertex_jx - vertex_ix) * (y - vertex_iy) / (vertex_jy - vertex_iy)
                        + vertex_ix)
            {
                c = !c;
            }
            j = i;
            i += 1;
        }

        c
    }

    fn get_conn_pipe_seg(&self, o: &Coord) -> (Option<Coord>, Option<Coord>) {
        let prev: Option<Coord>;
        let next: Option<Coord>;
        match self.get_pipe(o).unwrap() {
            Pipe::Start => {
                let mut out = Vec::new();
                // check left
                if let Some(left) = self.get_left(o) {
                    match self.get_pipe(&left).unwrap() {
                        Pipe::Horizontal | Pipe::LowerLeft | Pipe::UpperLeft => {
                            out.push(Some(left))
                        }
                        _ => (),
                    }
                };

                // check right
                if let Some(right) = self.get_right(o) {
                    match self.get_pipe(&right).unwrap() {
                        Pipe::Horizontal | Pipe::LowerRight | Pipe::UpperRight => {
                            out.push(Some(right))
                        }
                        _ => (),
                    }
                };

                // check top
                if let Some(top) = self.get_top(o) {
                    match self.get_pipe(&top).unwrap() {
                        Pipe::Vertical | Pipe::UpperRight | Pipe::UpperLeft => out.push(Some(top)),
                        _ => (),
                    }
                };

                // check bottom
                if let Some(bottom) = self.get_bottom(o) {
                    match self.get_pipe(&bottom).unwrap() {
                        Pipe::Vertical | Pipe::LowerRight | Pipe::LowerLeft => {
                            out.push(Some(bottom))
                        }
                        _ => (),
                    }
                };

                // should have two by now or else our input is invalid
                prev = out[0];
                next = out[1];
            }
            Pipe::Vertical => {
                // return top & bottom adjacencies
                prev = self.get_top(o);
                next = self.get_bottom(o);
            }
            Pipe::Horizontal => {
                // returns left & right adjacencies
                prev = self.get_left(o);
                next = self.get_right(o);
            }
            Pipe::LowerLeft => {
                // returns top & right adjacencies
                prev = self.get_top(o);
                next = self.get_right(o);
            }
            Pipe::LowerRight => {
                // returns top & left adjacencies
                prev = self.get_top(o);
                next = self.get_left(o);
            }
            Pipe::UpperRight => {
                // returns bottom & left adjacencies.1
                prev = self.get_bottom(o);
                next = self.get_left(o);
            }
            Pipe::UpperLeft => {
                // returns bottom & right adjacencies
                prev = self.get_bottom(o);
                next = self.get_right(o);
            }
            Pipe::Ground => {
                prev = None;
                next = None;
            }
        }
        (prev, next)
    }

    fn get_path(&mut self, o: &Coord) {
        let origin = *o;
        let mut path = vec![origin];
        let mut cur = origin;
        loop {
            let (a, b) = self.get_conn_pipe_seg(&cur);

            if (a == Some(origin) || b == Some(origin)) && path.len() > 2 {
                break;
            }

            if a.is_none() || path.contains(&a.unwrap()) {
                path.push(b.unwrap());
                cur = b.unwrap();
                continue;
            };

            path.push(a.unwrap());
            cur = a.unwrap();
        }

        let verts: Vec<Coord> = path
            .iter()
            .filter(|p| {
                matches!(
                    self.get_pipe(p).unwrap(),
                    &Pipe::LowerLeft
                        | &Pipe::LowerRight
                        | &Pipe::UpperLeft
                        | &Pipe::UpperRight
                        | &Pipe::Start
                )
            })
            .copied()
            .collect();
        self.path = path;
        self.verts = verts;
    }
}

impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for row in self.tiles.iter() {
            for pipe in row {
                output.push_str(&pipe.to_string());
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

fn solve_part1(s: &str) -> usize {
    let mut diagram = Diagram::new(s);
    println!("Diagram: \n{}", diagram);
    println!("Start position = {:?}", diagram.start);
    let start = diagram.start;
    diagram.get_path(&start);
    diagram.path.len() / 2
}

fn solve_part2(s: &str) -> usize {
    let mut diagram = Diagram::new(s);
    let start = diagram.start;
    diagram.get_path(&start);

    let height = diagram.tiles.len();
    let width = diagram.tiles[0].len();

    let mut interior = 0;
    for x in 0..width {
        for y in 0..height {
            if diagram.path.contains(&(x, y)) {
                continue;
            }
            if diagram.is_interior(&(x, y)) {
                interior += 1;
            }
        }
    }

    interior
}

fn main() {
    let cli_args = Cli::parse();
    let input = &fs::read_to_string(cli_args.input).unwrap();
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}
