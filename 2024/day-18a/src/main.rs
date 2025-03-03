use anyhow::{anyhow, Context, Result};
use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::{stdin, BufRead};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let max_dim: usize = args.get(1).context("No square dim specified")?.parse()?;
    dbg!(max_dim);
    // Note the "+ 1". If the range is [0,70], we need a Grid of size 71
    let mut grid = Grid::new(max_dim + 1);
    // grid.print();

    let bytes_to_take: usize = args.get(2).context("No bytes to take specified")?.parse()?;
    let mut lines = stdin().lock().lines();
    for _ in 0..bytes_to_take {
        let line = lines
            .next()
            .context(format!("Expected at least {} bytes", bytes_to_take))??;

        let mut coords = Vec::new();
        for token in line.split(",") {
            coords.push(token.parse()?);
            if coords.len() > 2 {
                return Err(anyhow!("More than two coordinates supplied: {}", &line));
            }
        }
        // Coordinates given in X,Y: X is distance from left edge; Y distance from top edge
        let point = Point::new(coords[1], coords[0]);
        grid.corrupt(&point);
    }
    // grid.print();

    let start = Point::new(0, 0);
    let end = Point::new(max_dim, max_dim);
    let min_steps = grid
        .bfs(start, end)
        .context(format!("No path from {:?} to {:?} in grid", start, end))?;
    println!("{}", min_steps - 1);

    Ok(())
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

    fn corrupt(&mut self, point: &Point) {
        self.cells[point.x][point.y] = true;
    }

    fn is_corrupt(&self, point: &Point) -> bool {
        self.cells[point.x][point.y]
    }

    fn bfs(&self, start: Point, end: Point) -> Option<usize> {
        if self.is_corrupt(&start) || self.is_corrupt(&end) {
            return None;
        }

        let mut queue = VecDeque::new();
        queue.push_back(Path {
            points: vec![start],
        });
        let mut visited: HashMap<Point, usize> = HashMap::new();

        while let Some(path) = queue.pop_front() {
            let next = path.last();

            for neighbor in self.neighbors(&next) {
                if !path.contains(&neighbor)
                    && queue.iter().all(|p| !p.points.contains(&neighbor))
                    && (!visited.contains_key(&neighbor) || visited[&neighbor] > path.length())
                {
                    let mut points = path.points.clone();
                    points.push(neighbor);
                    queue.push_back(Path { points });
                }
            }

            visited
                .entry(path.last())
                .and_modify(|m| *m = min(*m, path.length()))
                .or_insert(path.length());
        }

        visited.get(&end).copied()
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

    // fn print(&self) {
    //     for row in &self.cells {
    //         for cell in row {
    //             let to_print = if *cell { '#' } else { '.' };
    //             print!("{}", to_print);
    //         }
    //         println!();
    //     }
    //     println!(
    //         "--------------------------------------------------------------------------------"
    //     );
    // }
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

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

impl Path {
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
