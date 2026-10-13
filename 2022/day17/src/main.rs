use std::{
    collections::HashMap,
    io::{BufRead, stdin},
};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let dirs = parse_input()?;

    println!("Part 1: {}", part1(&dirs));
    println!("Part 2: {}", part2(&dirs));

    Ok(())
}

fn parse_input() -> Result<Vec<Dir>> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;

    let mut dirs = Vec::new();
    for c in line.trim().chars() {
        let d = match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => bail!("unrecognized character: {c}"),
        };
        dirs.push(d);
    }
    Ok(dirs)
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

fn part1(dirs: &[Dir]) -> usize {
    let mut board = Board::new(10_000);
    let mut jet_idx = 0;

    for s in shapes().iter().cycle().take(2022) {
        board.place(s, dirs, &mut jet_idx);
    }

    board.height()
}

fn part2(dirs: &[Dir]) -> usize {
    const TOTAL: usize = 1_000_000_000_000;

    let shapes = shapes();
    let mut board = Board::new(100_000);
    let mut jet_idx = 0;

    // (shape index, jet index, surface profile) -> (rock count, height at that point)
    let mut seen: HashMap<(usize, usize, Vec<usize>), (usize, usize)> = HashMap::new();
    let mut extra_height = 0;

    let mut i = 0;
    while i < TOTAL {
        let shape_idx = i % shapes.len();
        let key = (shape_idx, jet_idx % dirs.len(), board.surface_profile());

        if let Some(&(prev_i, prev_height)) = seen.get(&key) {
            let cycle_len = i - prev_i;
            let cycle_height = board.height() - prev_height;
            let cycles = (TOTAL - i) / cycle_len;

            extra_height += cycles * cycle_height;
            i += cycles * cycle_len;

            // We've fast-forwarded; stop looking for further cycles and just
            // simulate the short remainder normally.
            seen.clear();
        } else {
            seen.insert(key, (i, board.height()));
        }

        board.place(&shapes[shape_idx], dirs, &mut jet_idx);
        i += 1;
    }

    board.height() + extra_height
}

type Shape = Vec<Vec<char>>;

struct Board {
    cells: Vec<Vec<char>>,
    // Row index of the highest occupied cell so far (cells.len() - 1 means empty/floor)
    top: usize,
}

impl Board {
    fn new(length: usize) -> Self {
        let cells = vec![vec!['.'; 7]; length];
        let top = cells.len() - 1;
        Self { cells, top }
    }

    fn place(&mut self, shape: &Shape, dirs: &[Dir], jet_idx: &mut usize) {
        let mut current = (self.top - 3 - shape.len(), 2);

        loop {
            let dir = &dirs[*jet_idx % dirs.len()];
            *jet_idx += 1;
            current = self.push(dir, shape, current);

            let next = (current.0 + 1, current.1);
            if self.collides(shape, next) {
                self.settle_rock(shape, current);
                self.top = self.top.min(current.0);
                break;
            } else {
                current = next;
            }
        }
    }

    // Relative depth, from the current overall top, of the first occupied cell
    // in each column. Along with the next shape/jet, this is enough to detect
    // a repeating state.
    fn surface_profile(&self) -> Vec<usize> {
        (0..7)
            .map(|col| {
                let mut row = self.top;
                while row < self.cells.len() && self.cells[row][col] != '#' {
                    row += 1;
                }
                row - self.top
            })
            .collect()
    }

    fn push(&self, dir: &Dir, shape: &Shape, (x, y): (usize, usize)) -> (usize, usize) {
        let pushed = match dir {
            Dir::Left if y > 0 => (x, y - 1),
            Dir::Right if y < 6 => (x, y + 1),
            _ => (x, y),
        };

        if self.collides(shape, pushed) {
            (x, y)
        } else {
            pushed
        }
    }

    fn collides(&self, shape: &Shape, (x, y): (usize, usize)) -> bool {
        if x + shape.len() > self.cells.len() - 1 || y + shape[0].len() > 7 {
            return true;
        }

        for (s_x, s_row) in shape.iter().enumerate() {
            for (s_y, s_c) in s_row.iter().enumerate() {
                if *s_c == '#' && self.cells[x + s_x][y + s_y] == '#' {
                    return true;
                }
            }
        }
        false
    }

    fn settle_rock(&mut self, shape: &Shape, (x, y): (usize, usize)) {
        for (s_x, s_row) in shape.iter().enumerate() {
            for (s_y, s_c) in s_row.iter().enumerate() {
                if *s_c == '#' {
                    self.cells[x + s_x][y + s_y] = '#';
                }
            }
        }
    }

    fn height(&self) -> usize {
        self.cells.len() - 1 - self.top
    }
}

fn shapes() -> Vec<Shape> {
    vec![
        vec![vec!['#', '#', '#', '#']],
        vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ],
        vec![
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ],
        vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']],
        vec![vec!['#', '#'], vec!['#', '#']],
    ]
}
