use anyhow::{Context, Result};
use std::cmp::{min, Ordering};
use std::collections::{HashMap, VecDeque};
use std::io::BufRead;
use std::ops::{Add, Index};

fn main() -> Result<()> {
    let mut maze = Maze::default();
    for line in std::io::stdin().lock().lines() {
        maze.push(line?.trim().chars().collect());
    }
    let start = maze.find('S').context("Start not found")?;
    let _end = maze.find('E').context("End not found")?;

    println!(
        "{}",
        maze.shortest_path_bfs(start).context("Path not found")?
    );

    Ok(())
}

#[derive(Default)]
struct Maze {
    grid: Vec<Vec<char>>,
}

impl Maze {
    fn push(&mut self, row: Vec<char>) {
        self.grid.push(row);
    }

    fn find(&self, to_find: char) -> Option<Point> {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == to_find {
                    return Some(Point(i, j));
                }
            }
        }
        None
    }

    fn shortest_path_bfs(&self, start: Point) -> Option<u64> {
        let mut queue = VecDeque::new();
        queue.push_back(Node {
            coords: start,
            score: 0,
            orient: Orient::East,
        });
        let mut visited: HashMap<Point, u64> = HashMap::new();
        let mut shortest: Option<u64> = None;

        while let Some(p) = queue.pop_front() {
            if self[p.coords] == 'E' {
                if let Some(s) = shortest {
                    shortest = Some(min(s, p.score));
                } else {
                    shortest = Some(p.score);
                }
                continue;
            }

            for (orient, score) in p.orient.turns() {
                let next = p.coords + orient;
                if self[next] != '#'
                    && (!visited.contains_key(&next) || visited[&next] > p.score + score)
                {
                    queue.push_back(Node {
                        coords: next,
                        score: p.score + score,
                        orient,
                    });
                }
            }

            visited
                .entry(p.coords)
                .and_modify(|m| *m = min(*m, p.score))
                .or_insert(p.score);
        }

        shortest
    }
}

struct Node {
    coords: Point,
    score: u64,
    orient: Orient,
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(&self))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl Index<Point> for Maze {
    type Output = char;

    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point(usize, usize);

impl Add<Orient> for Point {
    type Output = Point;

    fn add(self, rhs: Orient) -> Self::Output {
        let rhs = rhs.value();
        Point(
            (self.0 as isize + rhs.0) as usize,
            (self.1 as isize + rhs.1) as usize,
        )
    }
}

#[derive(Copy, Clone)]
enum Orient {
    North,
    East,
    South,
    West,
}

impl Orient {
    fn value(&self) -> (isize, isize) {
        match self {
            Orient::North => (-1, 0),
            Orient::East => (0, 1),
            Orient::South => (1, 0),
            Orient::West => (0, -1),
        }
    }

    fn turns(&self) -> Vec<(Self, u64)> {
        match self {
            Orient::North => vec![
                (Orient::North, 1),
                (Orient::East, 1001),
                (Orient::South, 2001),
                (Orient::West, 1001),
            ],
            Orient::East => vec![
                (Orient::North, 1001),
                (Orient::East, 1),
                (Orient::South, 1001),
                (Orient::West, 2001),
            ],
            Orient::South => vec![
                (Orient::North, 2001),
                (Orient::East, 1001),
                (Orient::South, 1),
                (Orient::West, 1001),
            ],
            Orient::West => vec![
                (Orient::North, 1001),
                (Orient::East, 2001),
                (Orient::South, 1001),
                (Orient::West, 1),
            ],
        }
    }
}
