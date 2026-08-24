use anyhow::{Result, anyhow, bail};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let map = parse_input()?;

    println!("Part 1: {}", part1(&map)?);
    println!("Part 2: {}", part2(&map)?);

    Ok(())
}

fn parse_input() -> Result<RiskMap> {
    let input = stdin()
        .lock()
        .lines()
        .collect::<std::io::Result<Vec<_>>>()?
        .join("\n");
    parse_risk_map(&input)
}

fn parse_risk_map(input: &str) -> Result<RiskMap> {
    let cells = input
        .lines()
        .enumerate()
        .map(|(line_idx, l)| {
            l.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    c.to_digit(10).map(|d| d as usize).ok_or_else(|| {
                        anyhow!(
                            "Invalid digit '{c}' at line {}, column {}",
                            line_idx + 1,
                            col_idx + 1
                        )
                    })
                })
                .collect::<Result<Vec<usize>>>()
        })
        .collect::<Result<Vec<Vec<usize>>>>()?;
    Ok(RiskMap { cells })
}

fn part1(map: &RiskMap) -> Result<usize> {
    map.min_risk_path()
}

fn part2(map: &RiskMap) -> Result<usize> {
    let map = map.expand();
    map.min_risk_path()
}

#[derive(Debug, Clone)]
struct RiskMap {
    cells: Vec<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct Context {
    head: (usize, usize),
    risk: usize,
}

impl RiskMap {
    fn min_risk_path(&self) -> Result<usize> {
        let top_left = (0, 0);
        let bottom = self.cells.len() - 1;
        let bottom_right = (bottom, self.cells[bottom].len() - 1);
        self.min_risk_path_dij(top_left, bottom_right)
    }

    fn min_risk_path_dij(&self, start: (usize, usize), end: (usize, usize)) -> Result<usize> {
        let mut queue = BinaryHeap::from([Context {
            head: start,
            risk: 0,
        }]);
        let mut visited = HashSet::from([start]);

        while let Some(current) = queue.pop() {
            if current.head == end {
                return Ok(current.risk);
            }

            let (i, j) = current.head;
            for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if let Some((ni, nj)) = self.in_bounds((i as isize + di, j as isize + dj))
                    && visited.insert((ni, nj))
                {
                    let mut next = current.clone();
                    next.push((ni, nj), self.cells[ni][nj]);
                    queue.push(next);
                }
            }
        }

        bail!("Path not found")
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

    fn expand(&self) -> RiskMap {
        let height = self.cells.len();
        let width = self.cells[0].len();
        let mut cells = vec![vec![0; width * 5]; height * 5];

        for i in 0..cells.len() {
            for j in 0..cells[i].len() {
                let base = self.cells[i % height][j % width];
                let increment = i / height + j / width;
                cells[i][j] = ((base + increment - 1) % 9) + 1;
            }
        }

        RiskMap { cells }
    }
}

impl Context {
    fn push(&mut self, cell: (usize, usize), risk: usize) {
        self.head = cell;
        self.risk += risk;
    }
}

impl Eq for Context {}

impl PartialEq<Self> for Context {
    fn eq(&self, other: &Self) -> bool {
        self.risk.cmp(&other.risk) == Ordering::Equal
    }
}

impl PartialOrd<Self> for Context {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Context {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_sample_grid_like_the_puzzle_description() {
        let map = parse_risk_map(include_str!("../sample1.txt")).unwrap();
        let expanded = map.expand();

        assert_eq!(&expanded.cells[0][..10], &[1, 1, 6, 3, 7, 5, 1, 7, 4, 2]);
        assert_eq!(&expanded.cells[0][10..20], &[2, 2, 7, 4, 8, 6, 2, 8, 5, 3]);
    }

    #[test]
    fn finds_sample_part2_answer() {
        let map = parse_risk_map(include_str!("../sample1.txt")).unwrap();
        assert_eq!(part2(&map).unwrap(), 315);
    }
}
