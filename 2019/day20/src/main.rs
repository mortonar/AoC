use anyhow::{Result, bail};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;

fn main() -> Result<()> {
    let map = parse_input()?;

    println!("Part 1: {}", map.min_steps_bfs(false)?);
    println!("Part 2: {}", map.min_steps_bfs(true)?);

    Ok(())
}

fn parse_input() -> Result<Maze> {
    let mut cells = Vec::new();
    let (mut max_y, mut max_x) = (0, 0);
    for line in std::io::stdin().lock().lines() {
        let row: Vec<_> = line?.chars().collect();
        max_y += 1;
        max_x = max_x.max(row.len());
        cells.push(row);
    }
    let in_bounds = |x: isize, y: isize| {
        if y >= 0 && y < cells.len() as isize && x >= 0 && x < cells[y as usize].len() as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    };

    let outer_x = [2, max_x - 3];
    let outer_y = [2, max_y - 3];
    let mut start = Coords { x: 0, y: 0 };
    let mut end = Coords { x: 0, y: 0 };
    let mut intermediate_teles: HashMap<String, (Coords, isize)> = HashMap::new();
    let mut teleports = HashMap::new();
    for (y, row) in cells.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if !c.is_ascii_uppercase() {
                continue;
            }

            for (xd, yd) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = (x as isize + xd, y as isize + yd);
                let Some((x, y)) = in_bounds(x, y) else {
                    continue;
                };

                if !cells[y][x].is_ascii_uppercase() {
                    continue;
                }
                let label = format!("{}{}", c, cells[y][x]);

                // Find open space to tag with label or skip if wrong about label orientation -
                // the open space is either one more space in this direction or two spaces in
                // the opposite direction.
                let (x_label_1, y_label_1) = (x as isize + xd, y as isize + yd);
                let (x_label_2, y_label_2) = (x as isize + -2 * xd, y as isize + -2 * yd);
                let label_pos = in_bounds(x_label_1, y_label_1)
                    .filter(|&(x, y)| cells[y][x] == '.')
                    .or_else(|| {
                        in_bounds(x_label_2, y_label_2).filter(|&(x, y)| cells[y][x] == '.')
                    });
                let Some((x, y)) = label_pos else {
                    continue;
                };
                let current = Coords { x, y };
                let depth = if outer_x.contains(&x) || outer_y.contains(&y) {
                    -1
                } else {
                    1
                };
                match label.as_str() {
                    "AA" => start = current,
                    "ZZ" => end = current,
                    _ => {
                        if let Some((prev, prev_depth)) = intermediate_teles.remove(&label) {
                            teleports.insert(prev, (current, prev_depth));
                            teleports.insert(current, (prev, depth));
                        } else {
                            intermediate_teles.insert(label, (current, depth));
                        }
                    }
                }
            }
        }
    }

    Ok(Maze {
        cells,
        start,
        end,
        teleports,
    })
}

#[derive(Debug)]
struct Maze {
    cells: Vec<Vec<char>>,
    start: Coords,
    end: Coords,
    // from -> (to, depth change)
    teleports: HashMap<Coords, (Coords, isize)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Maze {
    fn min_steps_bfs(&self, with_depth: bool) -> Result<usize> {
        let mut queue = BinaryHeap::new();
        queue.push(SearchNode {
            coords: self.start,
            steps: 0,
            depth: 0,
        });
        let mut visited = HashMap::new();
        visited.insert((self.start, 0), 0);

        while let Some(current) = queue.pop() {
            if current.coords == self.end && current.depth == 0 {
                return Ok(current.steps);
            }

            let Coords { x, y } = current.coords;
            let mut neighbors: Vec<_> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(xd, yd)| self.in_bounds(x as isize + xd, y as isize + yd))
                .filter(|&(x, y)| self.cells[y][x] == '.')
                .map(|(x, y)| (Coords { x, y }, 0))
                .collect();

            if let Some(&(tele_to, depth)) = self.teleports.get(&current.coords) {
                // Outer teleporters are disabled at the base level
                if with_depth && current.depth == 0 && depth == -1 {
                    continue;
                }

                neighbors.push((tele_to, depth));
            }

            for (neighbor, depth_change) in neighbors {
                let new_depth = if with_depth {
                    current.depth + depth_change
                } else {
                    0
                };

                if let Some(&previous) = visited.get(&(neighbor, new_depth))
                    && previous < current.steps + 1
                {
                    continue;
                }

                queue.push(SearchNode {
                    coords: neighbor,
                    steps: current.steps + 1,
                    depth: new_depth,
                });
                visited.insert((neighbor, new_depth), current.steps + 1);
            }
        }

        bail!("No solution found")
    }

    fn in_bounds(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if y >= 0
            && y < self.cells.len() as isize
            && x >= 0
            && x < self.cells[y as usize].len() as isize
        {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}

#[derive(Eq)]
struct SearchNode {
    coords: Coords,
    steps: usize,
    depth: isize,
}
impl PartialEq<Self> for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords && self.steps == other.steps
    }
}
impl PartialOrd<Self> for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}
