use anyhow::{anyhow, Context, Result};
use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fmt::{Display, Formatter};
use std::io::{stdin, BufRead};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let max_dim: usize = args.get(1).context("No square dim specified")?.parse()?;
    // Note the "+ 1". If the range is [0,70], we need a Grid of size 71
    let mut grid = Grid::new(max_dim + 1);

    let start = Point::new(0, 0);
    let end = Point::new(max_dim, max_dim);

    let mut bytes = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let mut coords = Vec::new();
        for token in line.split(",") {
            coords.push(token.parse()?);
            if coords.len() > 2 {
                return Err(anyhow!("More than two coordinates supplied: {}", &line));
            }
        }
        // Coordinates given in X,Y: X is distance from left edge; Y distance from top edge
        let point = Point::new(coords[1], coords[0]);
        bytes.push(point);
    }

    // Binary search for the path-breaking byte
    let mut low = 0;
    let mut high = bytes.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;

        grid.clear();
        grid.corrupt_until(&bytes[0..mid]);

        if grid.bfs(start, end) {
            grid.corrupt(&bytes[mid]);
            if !grid.bfs(start, end) {
                println!("{}", &bytes[mid]);
                return Ok(());
            } else {
                low = mid + 1;
            }
        } else {
            high = mid - 1;
        }
    }

    Err(anyhow!("No bytes broke paths to the exit"))
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            // Grids are square
            cells: vec![vec![false; size]; size],
        }
    }

    fn clear(&mut self) {
        self.cells
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|x| *x = false));
    }

    fn corrupt_until(&mut self, points: &[Point]) {
        points.iter().for_each(|p| self.corrupt(p))
    }

    fn corrupt(&mut self, point: &Point) {
        self.cells[point.x][point.y] = true;
    }

    fn is_corrupt(&self, point: &Point) -> bool {
        self.cells[point.x][point.y]
    }

    fn bfs(&self, start: Point, end: Point) -> bool {
        if self.is_corrupt(&start) || self.is_corrupt(&end) {
            return false;
        }

        let mut queue = VecDeque::new();
        queue.push_back(Path::new(vec![start]));
        let mut visited: HashMap<Point, usize> = HashMap::new();

        while let Some(path) = queue.pop_front() {
            let next = path.last();

            if next == end {
                return true;
            }

            for neighbor in self.neighbors(&next) {
                if !path.contains(&neighbor)
                    && queue.iter().all(|p| !p.points.contains(&neighbor))
                    && (!visited.contains_key(&neighbor) || visited[&neighbor] > path.length())
                {
                    let mut points = path.points.clone();
                    points.push(neighbor);
                    queue.push_back(Path::new(points));
                }
            }

            visited
                .entry(path.last())
                .and_modify(|m| *m = min(*m, path.length()))
                .or_insert(path.length());
        }

        false
    }

    // Return all non-corrupt neighbors that are in bounds.
    fn neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let orientations = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let grid_max = self.cells.len() as isize;

        for (x, y) in orientations {
            let (x, y) = (x + point.x as isize, y + point.y as isize);
            if x >= 0 && y >= 0 && x < grid_max && y < grid_max {
                let (x, y) = (x as usize, y as usize);
                let neighbor = Point { x, y };
                if !self.is_corrupt(&neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.y, self.x)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn length(&self) -> usize {
        self.points.len()
    }

    fn last(&self) -> Point {
        self.points.last().unwrap().clone()
    }

    fn contains(&self, point: &Point) -> bool {
        self.points.contains(&point)
    }
}
