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

    // Go through each node on the fair path and consider the cheats for that node by scanning
    // forward in the remaining nodes in the fair path. We can "cheat" to that node if a coordinate
    // diff says we're <= 20 picoseconds away.
    //
    // NOTE: This will have valid moves show up as cheats if we were looking to save <= 20
    // picoseconds but good thing we're looking for >= 100 :)
    let mut count = 0;
    for (cheat_from_idx, cheat_from) in fair_path.points.iter().enumerate() {
        for (cheat_to_idx, cheat_to) in fair_path.points.iter().enumerate().skip(cheat_from_idx + 1)
        {
            let cheat_distance = cheat_from.absolute_diff(cheat_to);
            if cheat_distance <= 20 {
                let savings = (cheat_to_idx - cheat_from_idx) - cheat_distance;
                if savings >= to_save {
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
        self.fair_dfs(&mut path);
        path
    }

    fn fair_dfs(&self, current_path: &mut Path) {
        if current_path.last().eq(&self.end) {
            return;
        }

        for neighbor in self.neighbors(current_path.last()) {
            if !current_path.contains(&neighbor) {
                current_path.push(neighbor);
                return self.fair_dfs(current_path);
            }
        }
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = vec![];
        for (i, j) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (x, y) = (point.0 as isize + i, point.1 as isize + j);
            if self.in_bounds(x, y) {
                let p = Point(x as usize, y as usize);
                if self[p] != '#' {
                    neighbors.push(p);
                }
            }
        }
        neighbors
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.grid.len() as isize && y >= 0 && y < self.grid[x as usize].len() as isize
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
}

impl Index<Point> for Track {
    type Output = char;

    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point(usize, usize);

impl Point {
    fn absolute_diff(&self, other: &Point) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
