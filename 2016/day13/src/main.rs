use anyhow::{Error, Result, anyhow};
use std::collections::{HashSet, VecDeque};
use std::env::args;
use std::hash::{Hash, Hasher};
use std::io::BufRead;

fn main() -> Result<()> {
    let favorite: usize = std::io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| Error::msg("No input provided"))??
        .parse()?;

    let args: Vec<usize> = args()
        .skip(1)
        .take(2)
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    let goal = match args.as_slice() {
        [x, y] => (*x, *y),
        _ => return Err(Error::msg("Expected exactly 2 coordinate arguments")),
    };

    let (steps, visited) = min_steps_bfs(goal, favorite)?;
    println!("Part 1: {}", steps);
    println!(
        "Part 2: {}",
        visited.iter().filter(|v| v.steps <= 50).count()
    );

    Ok(())
}

fn min_steps_bfs(goal: Coordinate, favorite: usize) -> Result<(usize, Visited)> {
    let mut queue = VecDeque::new();
    queue.push_back(Node::default());

    let mut visited: Visited = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if current.coord == goal {
            return Ok((current.steps, visited));
        }

        if let Some(v) = visited.get(&current)
            && v.steps <= current.steps
        {
            continue;
        }
        visited.insert(current.clone());

        current
            .neighbors(favorite)
            .into_iter()
            .for_each(|n| queue.push_back(n));
    }

    Err(anyhow!(
        "No paths found for goal {:?} using favorite {}",
        goal,
        favorite
    ))
}

type Coordinate = (usize, usize);
type Visited = HashSet<Node>;

#[derive(Clone, Debug, Eq)]
struct Node {
    coord: Coordinate,
    steps: usize,
}

const ORIENTATIONS: [&(isize, isize); 4] = [&(-1, 0), &(0, 1), &(1, 0), &(0, -1)];

impl Node {
    fn new(coord: Coordinate, steps: usize) -> Self {
        Self { coord, steps }
    }

    fn neighbors(&self, favorite: usize) -> Vec<Node> {
        ORIENTATIONS
            .iter()
            .filter_map(|&&(dx, dy)| {
                if let (Some(x), Some(y)) = (
                    self.coord.0.checked_add_signed(dx),
                    self.coord.1.checked_add_signed(dy),
                ) {
                    let num = (x * x) + (3 * x) + (2 * x * y) + (y) + (y * y) + favorite;
                    let num_ones = num.count_ones();
                    if num_ones.is_multiple_of(2) {
                        return Some(Node::new((x, y), self.steps + 1));
                    }
                }
                None
            })
            .collect()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new((1, 1), 0)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}
