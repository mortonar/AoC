use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

fn main() -> Result<()> {
    let map = parse_map()?;

    let mut memo = HashMap::new();
    let min_steps = min_steps(&map, &mut memo, false)?;
    println!("Part 1: {min_steps}");
    let min_steps = crate::min_steps(&map, &mut memo, true)?;
    println!("Part 2: {min_steps}");
    Ok(())
}

/// Map of ((from coords), (to coords)) to the min path between them
type Memo = HashMap<((usize, usize), (usize, usize)), usize>;

/// Perform BFS between all permutations of the locations while bidirectionally memoizing all the
/// intermediate results. Take the minimum order of all permutations.
fn min_steps(map: &Map, memo: &mut Memo, return_to_start: bool) -> Result<usize> {
    let mut nums_locations = map.get_num_locations();
    let start = nums_locations
        .iter()
        .position(|(_i, _j, n)| *n == 0)
        .ok_or_else(|| anyhow!("Could not find '0' cell"))?;
    let start = nums_locations.remove(start);
    let len = nums_locations.len();
    nums_locations
        .into_iter()
        .permutations(len)
        .map(|mut permutation| {
            permutation.insert(0, start);
            if return_to_start {
                permutation.push(start);
            }
            permutation
                .windows(2)
                .map(|win| {
                    fewest_steps_bfs(map, (win[0].0, win[0].1), (win[1].0, win[1].1), memo).unwrap()
                })
                .sum::<usize>()
        })
        .min()
        .ok_or_else(|| anyhow!("No result found"))
}

/// Starting at 0, get the fewest steps to visit all the other numbers at least once.
fn fewest_steps_bfs(
    map: &Map,
    from: (usize, usize),
    to: (usize, usize),
    memo: &mut Memo,
) -> Result<usize> {
    if let Some(cached) = memo.get(&(from, to)) {
        return Ok(*cached);
    }

    let mut queue = VecDeque::new();
    queue.push_back(SearchNode { path: vec![from] });
    let mut visited = HashSet::new();

    while let Some(current) = queue.pop_front() {
        if *current.head() == to {
            // -1 for the starting node
            let length = current.path.len() - 1;
            memo.insert((from, to), length);
            memo.insert((to, from), length);
            return Ok(length);
        }

        if !visited.insert(*current.head()) {
            continue;
        }

        map.get_neighbors(&current)
            .into_iter()
            .for_each(|n| queue.push_back(n));
    }

    Err(anyhow!("No path found"))
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug)]
enum Cell {
    Empty,
    Number(usize),
    Wall,
}

#[derive(Debug, Clone)]
struct SearchNode {
    path: Vec<(usize, usize)>,
}

impl SearchNode {
    fn head(&self) -> &(usize, usize) {
        self.path.last().unwrap()
    }
}

impl Map {
    fn get_num_locations(&self) -> Vec<(usize, usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(move |(i, col)| col.iter().enumerate().map(move |(j, cell)| (i, j, cell)))
            .filter_map(|(i, j, cell)| match cell {
                Cell::Number(n) => Some((i, j, *n)),
                _ => None,
            })
            .collect()
    }

    fn get_neighbors(&self, node: &SearchNode) -> Vec<SearchNode> {
        let (x, y) = *node.head();
        let mut neighbors = Vec::new();
        for &[xd, yd] in ORIENTATIONS.iter() {
            let (x, y) = (x as isize + xd, y as isize + yd);
            if x < 0
                || (x as usize) >= self.cells.len()
                || y < 0
                || (y as usize) >= self.cells[x as usize].len()
            {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            let mut neighbor = node.clone();
            match &self.cells[x][y] {
                Cell::Wall => continue,
                _ => neighbor.path.push((x, y)),
            }
            neighbors.push(neighbor);
        }
        neighbors
    }
}

const ORIENTATIONS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn parse_map() -> Result<Map> {
    let cells: Result<Vec<Vec<_>>> = stdin()
        .lines()
        .map(|line| {
            line?
                .chars()
                .map(<char as TryInto<Cell>>::try_into)
                .collect()
        })
        .collect();
    Ok(Map { cells: cells? })
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '0'..='9' => Ok(Cell::Number(value.to_digit(10).unwrap() as usize)),
            '#' => Ok(Cell::Wall),
            _ => Err(anyhow!("Unrecognized map cell: {value}")),
        }
    }
}
