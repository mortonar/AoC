use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, stdin},
};

use anyhow::{Result, anyhow, bail};

fn main() -> Result<()> {
    let grid = parse_input()?;

    println!("Part 1: {}", part1(&grid)?);
    println!("Part 2: {}", part2(&grid)?);

    Ok(())
}

fn parse_input() -> Result<Grid> {
    let cells: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.chars().collect()))
        .collect::<Result<_>>()?;

    for &c in cells.iter().flat_map(|r| r.iter()) {
        if !matches!(c, 'S' | 'E' | 'a'..='z') {
            bail!("invalid char: {c}");
        }
    }

    Ok(Grid { cells })
}

fn part1(grid: &Grid) -> Result<usize> {
    let start = grid
        .find_cell('S')
        .ok_or_else(|| anyhow!("missing start"))?;
    let finish = grid.find_cell('E').ok_or_else(|| anyhow!("missing end"))?;

    let height = |c: char| -> usize {
        match c {
            'S' => 0,
            'E' => 25,
            c => c as usize - 'a' as usize,
        }
    };

    let end = |loc: (usize, usize)| -> bool { loc == finish };

    grid.shortest_path_bfs(start, end, height)
}

fn part2(grid: &Grid) -> Result<usize> {
    let start = grid
        .find_cell('E')
        .ok_or_else(|| anyhow!("missing start"))?;

    let height = |c: char| -> usize {
        match c {
            'S' => 25,
            'E' => 0,
            c => 'z' as usize - c as usize,
        }
    };

    let end = |loc: (usize, usize)| -> bool { grid.cells[loc.0][loc.1] == 'a' };

    grid.shortest_path_bfs(start, end, height)
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn find_cell(&self, c: char) -> Option<(usize, usize)> {
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j] == c {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn shortest_path_bfs(
        &self,
        start: (usize, usize),
        end: impl Fn((usize, usize)) -> bool,
        height: impl Fn(char) -> usize,
    ) -> Result<usize> {
        #[derive(Debug, Copy, Clone)]
        struct Context {
            loc: (usize, usize),
            steps: usize,
        }

        let initial = Context {
            loc: start,
            steps: 0,
        };

        let mut queue = VecDeque::from([initial]);
        let mut visited = HashSet::from([initial.loc]);

        while let Some(current) = queue.pop_front() {
            if end(current.loc) {
                return Ok(current.steps);
            }

            let (ci, cj) = current.loc;

            let from_h = height(self.cells[ci][cj]);

            for (next_i, next_j) in [(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .filter_map(|(id, jd)| self.bounds_check((ci as isize + id, cj as isize + jd)))
            {
                let to_h = height(self.cells[next_i][next_j]);
                if to_h > from_h + 1 {
                    continue;
                }

                let next = (next_i, next_j);
                if visited.insert(next) {
                    queue.push_back(Context {
                        loc: next,
                        steps: current.steps + 1,
                    });
                }
            }
        }

        bail!("no path found from start to end")
    }

    fn bounds_check(&self, loc: (isize, isize)) -> Option<(usize, usize)> {
        let (ci, cj) = loc;

        if ci < 0 || cj < 0 {
            return None;
        }

        let (ci, cj) = (ci as usize, cj as usize);
        if ci >= self.cells.len() || cj >= self.cells[ci].len() {
            return None;
        }

        Some((ci, cj))
    }
}
