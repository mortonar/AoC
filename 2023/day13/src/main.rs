use std::io::{BufRead, stdin};

use anyhow::Result;

fn main() -> Result<()> {
    let patterns = parse_input()?;

    println!("Part 1: {}", part1(&patterns));
    println!("Part 2: {}", part2(&patterns));

    Ok(())
}

fn parse_input() -> Result<Vec<Pattern>> {
    let mut patterns = Vec::new();

    let mut cells = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;

        if line.is_empty() {
            patterns.push(Pattern { cells });
            cells = Vec::new();
            continue;
        }

        cells.push(line.chars().map(|c| c == '#').collect());
    }
    patterns.push(Pattern { cells });

    Ok(patterns)
}

fn part1(patterns: &[Pattern]) -> usize {
    find_reflections(patterns, 0)
}

fn part2(patterns: &[Pattern]) -> usize {
    find_reflections(patterns, 1)
}

fn find_reflections(patterns: &[Pattern], cell_diff: usize) -> usize {
    let (left, above) = patterns
        .iter()
        .filter_map(|p| p.find_reflection(cell_diff))
        .fold((0, 0), |(left, above), r| match r {
            Reflection::Vertical(l) => (left + l, above),
            Reflection::Horizontal(a) => (left, above + a),
        });
    left + 100 * above
}

#[derive(Debug)]
struct Pattern {
    cells: Vec<Vec<bool>>,
}

impl Pattern {
    fn find_reflection(&self, cell_diff: usize) -> Option<Reflection> {
        (0..self.cells.len() - 1)
            .find(|&r| self.row_reflection(r, cell_diff))
            .map(|r| Reflection::Horizontal(r + 1))
            .or_else(|| {
                (0..self.cells[0].len() - 1)
                    .find(|&c| self.col_reflection(c, cell_diff))
                    .map(|c| Reflection::Vertical(c + 1))
            })
    }

    fn row_reflection(&self, row: usize, cell_diff: usize) -> bool {
        let rows = self.cells.len();
        let (mut top, mut bottom) = (row as isize, row + 1);
        let mut diff = 0;

        while top > -1 && bottom < rows {
            diff += self.row_cmp(top as usize, bottom);

            if diff > cell_diff {
                return false;
            }

            (top, bottom) = (top - 1, bottom + 1);
        }

        diff == cell_diff
    }

    fn col_reflection(&self, col: usize, cell_diff: usize) -> bool {
        let cols = self.cells[0].len();
        let (mut left, mut right) = (col as isize, col + 1);
        let mut diff = 0;

        while left > -1 && right < cols {
            diff += self.col_cmp(left as usize, right);

            if diff > cell_diff {
                return false;
            }

            (left, right) = (left - 1, right + 1);
        }

        diff == cell_diff
    }

    fn row_cmp(&self, top: usize, bottom: usize) -> usize {
        self.cells[top]
            .iter()
            .zip(self.cells[bottom].iter())
            .filter(|&(&t, &b)| t != b)
            .count()
    }

    fn col_cmp(&self, left: usize, right: usize) -> usize {
        self.cells
            .iter()
            .filter(|row| row[left] != row[right])
            .count()
    }
}

#[derive(Debug)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}
