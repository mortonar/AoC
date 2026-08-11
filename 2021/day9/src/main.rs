use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let map = parse_input()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn parse_input() -> Result<Map> {
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
    Ok(Map { cells })
}

fn part1(map: &Map) -> usize {
    map.low_points()
        .iter()
        .fold(0, |acc, &(_, _, x)| acc + x as usize + 1)
}

fn part2(map: &Map) -> usize {
    let mut basins = map.basins();
    basins.sort_by_key(|b| b.len());
    basins.iter().rev().take(3).map(|b| b.len()).product()
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<u8>>,
}

impl Map {
    fn low_points(&self) -> Vec<(usize, usize, u8)> {
        let mut low_points = Vec::new();
        for (i, row) in self.cells.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let is_low = [(-1, 0), (0, -1), (0, 1), (1, 0)].iter().all(|(di, dj)| {
                    let (ni, nj) = (i as isize + di, j as isize + dj);
                    if ni < 0
                        || ni >= self.cells.len() as isize
                        || nj < 0
                        || nj >= row.len() as isize
                    {
                        return true;
                    }

                    let (ni, nj) = (ni as usize, nj as usize);
                    self.cells[ni][nj] > cell
                });

                if is_low {
                    low_points.push((i, j, cell))
                }
            }
        }
        low_points
    }

    fn basins(&self) -> Vec<HashSet<(usize, usize)>> {
        self.low_points()
            .iter()
            .map(|&(i, j, _c)| self.bfs((i, j)))
            .collect()
    }

    fn bfs(&self, (i, j): (usize, usize)) -> HashSet<(usize, usize)> {
        let mut queue = vec![(i, j)];
        let mut visited = HashSet::new();
        visited.insert((i, j));

        while let Some((ci, cj)) = queue.pop() {
            for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (ni, nj) = (ci as isize + di, cj as isize + dj);
                if ni < 0
                    || ni >= self.cells.len() as isize
                    || nj < 0
                    || nj >= self.cells[ni as usize].len() as isize
                {
                    continue;
                }

                let (ni, nj) = (ni as usize, nj as usize);
                if self.cells[ni][nj] != 9 && visited.insert((ni, nj)) {
                    queue.push((ni, nj));
                }
            }
        }

        visited
    }
}
