use anyhow::{anyhow, Context, Result};
use std::env;
use std::io::BufRead;
use std::ops::Index;

fn main() -> Result<()> {
    let mut track: Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        track.push(line?.trim().chars().collect());
    }
    let track = Track::new(track)?;

    let fair_path = track.find_fair_path();

    let args = env::args().collect::<Vec<String>>();
    let to_save: usize = match args.get(1) {
        Some(c) => c.parse()?,
        None => 100,
    };

    let mut count = 0;
    for (cheat_from, node) in fair_path.points.iter().enumerate() {
        for neighbor in track.neighbors(node, true) {
            let cheat_to = fair_path
                .points
                .iter()
                .position(|p| p.eq(&neighbor))
                .unwrap();
            if cheat_to > cheat_from {
                let saved = cheat_to - 2 - cheat_from;
                if saved >= to_save {
                    count += 1;
                }
            }
        }
    }
    println!("{} cheats saved at least {}", count, to_save);

    Ok(())
}

struct Track {
    grid: Vec<Vec<char>>,
    start: Point,
    end: Point,
}

impl Track {
    fn new(grid: Vec<Vec<char>>) -> Result<Self> {
        let mut start = None;
        let mut end = None;
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                match c {
                    'S' => {
                        if start.is_none() {
                            start = Some(Point(i, j))
                        } else {
                            return Err(anyhow!("Multiple starts found"));
                        }
                    }
                    'E' => {
                        if end.is_none() {
                            end = Some(Point(i, j))
                        } else {
                            return Err(anyhow!("Multiple ends found"));
                        }
                    }
                    _ => {}
                }
            }
        }
        let start = start.context("Start not found")?;
        let end = end.context("End not found")?;

        Ok(Self { grid, start, end })
    }

    fn find_fair_path(&self) -> Path {
        let mut path = Path::new(vec![self.start]);
        self.dfs(&mut path);
        path
    }

    fn dfs(&self, current_path: &mut Path) {
        if current_path.last().eq(&self.end) {
            return;
        }

        for neighbor in self.neighbors(current_path.last(), false) {
            if !current_path.contains(&neighbor) {
                current_path.push(neighbor);
                return self.dfs(current_path);
            }
        }
    }

    fn neighbors(&self, point: &Point, disable_collision: bool) -> Vec<Point> {
        let mut neighbors = vec![];
        for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (x, y) = (
                (point.0 as isize + i) as usize,
                (point.1 as isize + j) as usize,
            );
            if !disable_collision {
                // Given the boarder, everything one hop away is in bounds.
                if self.grid[x][y] != '#' {
                    neighbors.push(Point(x, y));
                }
            } else if self.grid[x][y] == '#' {
                let x = point.0 as isize + 2 * i;
                let y = point.1 as isize + 2 * j;
                if x >= 0
                    && x < self.grid.len() as isize
                    && y >= 0
                    && y < self.grid[0].len() as isize
                {
                    let (x, y) = (x as usize, y as usize);
                    if self.grid[x][y] != '#' {
                        neighbors.push(Point(x, y));
                    }
                }
            }
        }
        neighbors
    }
}

#[derive(Debug, Clone)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn last(&self) -> &Point {
        self.points.last().unwrap()
    }

    fn push(&mut self, point: Point) {
        self.points.push(point);
    }

    fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    fn time(&self) -> usize {
        // -1 for the starting node
        self.points.len() - 1
    }
}

impl Index<Point> for Track {
    type Output = char;

    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point(usize, usize);
