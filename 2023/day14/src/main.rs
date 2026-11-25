use std::{
    collections::HashMap,
    io::{BufRead, stdin},
};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let platform = parse_input()?;

    println!("Part 1: {}", part1(&platform));
    println!("Part 2: {}", part2(&platform));

    Ok(())
}

fn parse_input() -> Result<Platform> {
    let mut cells = Vec::new();
    for line in stdin().lock().lines() {
        let row = line?
            .chars()
            .map(|c| match c {
                'O' => Ok(Cell::Round),
                '#' => Ok(Cell::Cube),
                '.' => Ok(Cell::Open),
                _ => bail!("invalid cell character: {c}"),
            })
            .collect::<Result<Vec<_>>>()?;
        cells.push(row);
    }
    Ok(Platform { cells })
}

fn part1(platform: &Platform) -> usize {
    let mut platform = platform.clone();
    platform.tilt_north();
    platform.load_north()
}

fn part2(platform: &Platform) -> usize {
    let mut platform = platform.clone();
    let mut seen = HashMap::new();
    for cycle_num in 1usize.. {
        platform.cycle();

        if let Some(previous) = seen.get(&platform) {
            let cycle_length = cycle_num - previous;
            let remaining = (1_000_000_000 - cycle_num) % cycle_length;
            for _ in 0..remaining {
                platform.cycle();
            }
            return platform.load_north();
        } else {
            seen.insert(platform.clone(), cycle_num);
        }
    }
    panic!("cycle not found")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Round,
    Cube,
    Open,
}

impl Platform {
    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        for to_move in 1..self.cells.len() {
            for moving in (1..=to_move).rev() {
                self.move_row(moving, moving - 1);
            }
        }
    }

    fn tilt_south(&mut self) {
        let rows = self.cells.len();
        for to_move in (0..rows - 1).rev() {
            for moving in to_move..=rows - 2 {
                self.move_row(moving, moving + 1);
            }
        }
    }

    fn move_row(&mut self, from: usize, to: usize) {
        for col in 0..self.cells[from].len() {
            if self.cells[from][col] == Cell::Round && self.cells[to][col] == Cell::Open {
                self.cells[to][col] = Cell::Round;
                self.cells[from][col] = Cell::Open;
            }
        }
    }

    fn tilt_east(&mut self) {
        let cols = self.cells[0].len();
        for to_move in (0..cols - 1).rev() {
            for moving in to_move..=cols - 2 {
                self.move_col(moving, moving + 1);
            }
        }
    }

    fn tilt_west(&mut self) {
        for to_move in 1..self.cells[0].len() {
            for moving in (1..=to_move).rev() {
                self.move_col(moving, moving - 1);
            }
        }
    }

    fn move_col(&mut self, from: usize, to: usize) {
        for row in 0..self.cells.len() {
            if self.cells[row][from] == Cell::Round && self.cells[row][to] == Cell::Open {
                self.cells[row][to] = Cell::Round;
                self.cells[row][from] = Cell::Open;
            }
        }
    }

    fn load_north(&self) -> usize {
        (1..)
            .zip(self.cells.iter().rev())
            .map(|(r, row)| row.iter().filter(|c| **c == Cell::Round).count() * r)
            .sum()
    }
}
