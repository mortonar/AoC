use std::{
    io::{BufRead, stdin},
    iter::once,
};

use anyhow::Result;

fn main() -> Result<()> {
    let schematic = parse_input()?;

    println!("Part 1: {}", part1(&schematic));
    println!("Part 2: {}", part2(&schematic));

    Ok(())
}

fn parse_input() -> Result<Schematic> {
    let cells = stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.chars().collect()))
        .collect::<Result<_>>()?;
    Ok(Schematic { cells })
}

fn part1(schematic: &Schematic) -> usize {
    schematic.parts().map(|pn| pn.num).sum()
}

fn part2(schematic: &Schematic) -> usize {
    schematic.gear_ratio_sum()
}

#[derive(Debug)]
struct Schematic {
    cells: Vec<Vec<char>>,
}

/// A number occupying columns `start..=end` of `row`.
#[derive(Debug)]
struct PartNum {
    num: usize,
    row: usize,
    start: usize,
    end: usize,
}

impl Schematic {
    fn get(&self, row: usize, col: usize) -> Option<char> {
        self.cells.get(row)?.get(col).copied()
    }

    fn numbers(&self) -> Vec<PartNum> {
        let mut numbers = Vec::new();
        for (row, line) in self.cells.iter().enumerate() {
            // (start column, value so far) of the number being read
            let mut current: Option<(usize, usize)> = None;
            // Trailing '.' ends any number at the end of the line
            for (col, &c) in line.iter().chain(once(&'.')).enumerate() {
                match (c.to_digit(10), current) {
                    (Some(d), Some((start, n))) => current = Some((start, n * 10 + d as usize)),
                    (Some(d), None) => current = Some((col, d as usize)),
                    (None, Some((start, num))) => {
                        numbers.push(PartNum {
                            num,
                            row,
                            start,
                            end: col - 1,
                        });
                        current = None;
                    }
                    (None, None) => {}
                }
            }
        }
        numbers
    }

    fn parts(&self) -> impl Iterator<Item = PartNum> {
        self.numbers().into_iter().filter(|pn| {
            pn.neighbours()
                .any(|(row, col)| self.get(row, col).is_some_and(is_symbol))
        })
    }

    fn gear_ratio_sum(&self) -> usize {
        let parts: Vec<_> = self.parts().collect();
        self.positions()
            .filter(|&(row, col)| self.get(row, col) == Some('*'))
            .filter_map(|(row, col)| {
                let adj: Vec<_> = parts.iter().filter(|p| p.is_adjacent(row, col)).collect();
                (adj.len() == 2).then(|| adj[0].num * adj[1].num)
            })
            .sum()
    }

    fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(row, line)| (0..line.len()).map(move |col| (row, col)))
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

impl PartNum {
    /// All positions in the bounding box around the number (including the number itself).
    fn neighbours(&self) -> impl Iterator<Item = (usize, usize)> {
        let cols = self.start.saturating_sub(1)..=self.end + 1;
        (self.row.saturating_sub(1)..=self.row + 1)
            .flat_map(move |row| cols.clone().map(move |col| (row, col)))
    }

    fn is_adjacent(&self, row: usize, col: usize) -> bool {
        self.row.abs_diff(row) <= 1 && self.start.saturating_sub(1) <= col && col <= self.end + 1
    }
}
