use anyhow::{Result, bail};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let layout = parse_input()?;

    let stabilized = layout.stabilize();
    println!("Part 1: {}", stabilized.occupied());

    let config = Config {
        adj_select: AdjacentSelect::SightLine,
        occ_threshold: 5,
    };
    let layout = Layout { config, ..layout };
    let stabilized = layout.stabilize();
    println!("Part 2: {}", stabilized.occupied());

    Ok(())
}

fn parse_input() -> Result<Layout> {
    let mut cells = Vec::new();
    for line in stdin().lock().lines() {
        let mut row = Vec::new();
        for ch in line?.trim().chars() {
            match ch {
                '.' => row.push(Cell::Floor),
                'L' => row.push(Cell::Empty),
                '#' => row.push(Cell::Occupied),
                _ => bail!("Unrecognized character '{ch}'"),
            }
        }
        cells.push(row);
    }

    let config = Config::default();

    Ok(Layout { cells, config })
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Layout {
    cells: Vec<Vec<Cell>>,
    config: Config,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Config {
    adj_select: AdjacentSelect,
    occ_threshold: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            adj_select: AdjacentSelect::Adjacent,
            occ_threshold: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum AdjacentSelect {
    /// Each of 8 adjacent spaces
    Adjacent,
    /// First seat (non-floor) in each of 8 sightlines
    SightLine,
}

const DIRS: [[isize; 2]; 8] = [
    [0, 1],
    [0, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [1, 1],
    [-1, -1],
    [1, -1],
];

impl Layout {
    fn adjacent(&self, i: usize, j: usize) -> Vec<Cell> {
        let mut adjacent = Vec::new();
        for [di, dj] in DIRS {
            let (mut i, mut j) = (i as isize, j as isize);
            loop {
                i += di;
                j += dj;

                // Count out of bounds as floor
                if i < 0
                    || i >= self.cells.len() as isize
                    || j < 0
                    || j >= self.cells[i as usize].len() as isize
                {
                    adjacent.push(Cell::Floor);
                    break;
                }

                let (i, j) = (i as usize, j as usize);
                if self.cells[i][j] != Cell::Floor
                    || self.config.adj_select != AdjacentSelect::SightLine
                {
                    adjacent.push(self.cells[i][j]);
                    break;
                }
            }
        }
        adjacent
    }

    fn evolve(&self) -> Layout {
        let mut next = self.clone();
        for (i, row) in self.cells.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let adjacent_occ = self
                    .adjacent(i, j)
                    .iter()
                    .filter(|&&adj| adj == Cell::Occupied)
                    .count();
                if cell == Cell::Empty && adjacent_occ == 0 {
                    next.cells[i][j] = Cell::Occupied;
                } else if cell == Cell::Occupied && adjacent_occ >= self.config.occ_threshold {
                    next.cells[i][j] = Cell::Empty;
                }
            }
        }
        next
    }

    fn stabilize(&self) -> Layout {
        let mut current = self.clone();
        loop {
            let next = current.evolve();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }

    fn occupied(&self) -> usize {
        self.cells
            .iter()
            .flatten()
            .filter(|&&seat| seat == Cell::Occupied)
            .count()
    }
}
