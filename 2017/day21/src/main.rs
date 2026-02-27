use anyhow::{Error, Result, bail};
use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let start: Pattern = ".#./..#/###".parse()?;
    let mut rules = parse_input()?;

    let end = (0..5).fold(start, |p, _| rules.apply(p));
    println!("Part 1: {}", end.cells_on());
    let end = (0..13).fold(end, |p, _| rules.apply(p));
    println!("Part 2: {}", end.cells_on());

    Ok(())
}

fn parse_input() -> Result<Vec<Rule>> {
    stdin()
        .lines()
        .map(|l| l?.parse())
        .collect::<Result<Vec<_>, Error>>()
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(" => ").collect();
        if tokens.len() != 2 {
            bail!("Rules must have form x => y");
        }
        Ok(Self {
            input: tokens[0].parse()?,
            output: tokens[1].trim_end().parse()?,
            match_cache: HashMap::new(),
        })
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            cells: s
                .split('/')
                .map(|row| row.chars().collect::<Vec<char>>())
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Rule {
    input: Pattern,
    output: Pattern,
    match_cache: HashMap<Pattern, bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pattern {
    cells: Vec<Vec<char>>,
}

trait Apply {
    fn apply(&mut self, pattern: Pattern) -> Pattern;
}

impl Apply for Vec<Rule> {
    fn apply(&mut self, pattern: Pattern) -> Pattern {
        pattern
            .by_blocks()
            .into_iter()
            .map(|p| {
                let rule_idx = self.iter_mut().position(|r| r.matches(&p)).unwrap();
                self[rule_idx].apply(p)
            })
            .collect()
    }
}

impl Rule {
    fn matches(&mut self, pattern: &Pattern) -> bool {
        if pattern.size() != self.input.size() {
            return false;
        }

        if let Some(&cached) = self.match_cache.get(pattern) {
            return cached;
        }

        let mut pattern = pattern.clone();
        for _ in 0..4 {
            if pattern == self.input || pattern.flip() == self.input {
                self.match_cache.insert(pattern.clone(), true);
                return true;
            }
            pattern = pattern.rotate_clockwise();
        }

        self.match_cache.insert(pattern.clone(), false);
        false
    }

    fn apply(&self, _pattern: Pattern) -> Pattern {
        self.output.clone()
    }
}

impl Pattern {
    fn size(&self) -> usize {
        self.cells.len()
    }

    fn block_size(&self) -> usize {
        if self.size().is_multiple_of(2) { 2 } else { 3 }
    }

    fn by_blocks(self) -> Vec<Pattern> {
        self.cells
            .chunks_exact(self.block_size())
            .flat_map(|row_chunk| {
                (0..self.size())
                    .step_by(self.block_size())
                    .map(|col_start| {
                        row_chunk
                            .iter()
                            .map(|row| row[col_start..col_start + self.block_size()].to_vec())
                            .collect()
                    })
            })
            .collect()
    }

    fn cells_on(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|c| **c == '#')
            .count()
    }

    fn flip(&self) -> Self {
        let mut flipped = self.clone();
        flipped.cells.iter_mut().for_each(|row| row.reverse());
        flipped
    }

    fn rotate_clockwise(&self) -> Self {
        let mut rotated = self.clone();
        for i in 0..rotated.cells.len() {
            for j in i + 1..rotated.cells[i].len() {
                let temp = rotated.cells[i][j];
                rotated.cells[i][j] = rotated.cells[j][i];
                rotated.cells[j][i] = temp;
            }
        }
        rotated.flip()
    }
}

impl FromIterator<Vec<char>> for Pattern {
    fn from_iter<T: IntoIterator<Item = Vec<char>>>(iter: T) -> Self {
        Self {
            cells: iter.into_iter().collect(),
        }
    }
}

impl FromIterator<Pattern> for Pattern {
    fn from_iter<T: IntoIterator<Item = Pattern>>(iter: T) -> Self {
        let blocks: Vec<Pattern> = iter.into_iter().collect();
        let block_size = blocks[0].size();
        let blocks_per_row = (blocks.len() as f64).sqrt() as usize;
        let size = block_size * blocks_per_row;
        let mut cells = vec![vec!['.'; size]; size];

        for (i, block) in blocks.iter().enumerate() {
            let row_block = i / blocks_per_row;
            let col_block = i % blocks_per_row;
            for r in 0..block_size {
                for c in 0..block_size {
                    cells[row_block * block_size + r][col_block * block_size + c] =
                        block.cells[r][c];
                }
            }
        }

        Self { cells }
    }
}
