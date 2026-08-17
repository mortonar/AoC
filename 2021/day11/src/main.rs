use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let grid = parse_input()?;

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())
}

fn parse_input() -> Result<Grid> {
    let cells = stdin()
        .lock()
        .lines()
        .enumerate()
        .map(|(line_idx, l)| {
            let l = l?;
            l.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    c.to_digit(10).map(|d| d as u8).ok_or_else(|| {
                        anyhow!(
                            "Invalid digit '{c}' at line {}, column {}",
                            line_idx + 1,
                            col_idx + 1
                        )
                    })
                })
                .collect::<Result<Vec<u8>>>()
        })
        .collect::<Result<Vec<Vec<u8>>>>()?;
    Ok(Grid { cells })
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    (0..100).map(|_| grid.step()).sum()
}

fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut step = 0;
    let total = grid.cells.len() * grid.cells[0].len();
    loop {
        step += 1;
        let flashes = grid.step();
        if flashes == total {
            return step;
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<u8>>,
}

impl Grid {
    fn step(&mut self) -> usize {
        let mut flashed = HashSet::new();
        let mut to_flash = Vec::new();
        for (i, row) in self.cells.iter_mut().enumerate() {
            for (j, o) in row.iter_mut().enumerate() {
                *o += 1;
                if *o > 9 {
                    to_flash.push((i, j));
                    flashed.insert((i, j));
                }
            }
        }

        while let Some((i, j)) = to_flash.pop() {
            for (di, dj) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if let Some((i, j)) = self.in_bounds((i as isize + di, j as isize + dj)) {
                    let adj = &mut self.cells[i][j];
                    if *adj == 9 && flashed.insert((i, j)) {
                        to_flash.push((i, j));
                    }
                    *adj += 1;
                }
            }
        }

        flashed.iter().for_each(|&(i, j)| {
            self.cells[i][j] = 0;
        });

        flashed.len()
    }

    fn in_bounds(&self, (i, j): (isize, isize)) -> Option<(usize, usize)> {
        if i < 0
            || i >= self.cells.len() as isize
            || j < 0
            || j >= self.cells[i as usize].len() as isize
        {
            None
        } else {
            Some((i as usize, j as usize))
        }
    }
}
