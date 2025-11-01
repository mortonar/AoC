use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::io::BufRead;

// This problem absolutely put me through the ringer. I tried everything from turning the state
// representation into a bitmask (and back again...painfully) to A* search with lousy (but valid)
// heuristic. Everything.
//
// At the end of the day BFS sufficient for a solution. It's even fast! But the absolute CRUCIAL bit
// - and the only thing that matters in terms of runtime - is normalizing the state representations
// when tracking which states you've visited. Items of different elements all behave the same so
// they should be treated the same - despite the fancy labels.
fn main() -> Result<()> {
    // Element -> [M-Floor, G-Floor]
    let mut items: HashMap<String, Vec<u8>> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        let floor = match tokens[1] {
            "first" => 0,
            "second" => 1,
            "third" => 2,
            "fourth" => 3,
            _ => return Err(anyhow::anyhow!("invalid floor {}", tokens[1])),
        };

        for (i, t) in tokens.iter().enumerate().skip(1) {
            if !t.contains("generator") && !t.contains("microchip") {
                continue;
            }

            let element = tokens[i - 1];
            let element = element
                .strip_suffix("-compatible")
                .unwrap_or(element)
                .to_string();

            items.entry(element.clone()).or_insert(vec![255; 2]);
            if t.contains("generator") {
                items.get_mut(&element).unwrap()[1] = floor;
            } else {
                items.get_mut(&element).unwrap()[0] = floor;
            }
        }
    }

    // [[E1M-F, E1G-F], [E2M-F, E2G-F], ...]
    let mut items: Vec<Vec<u8>> = items.into_values().collect_vec();

    println!("Part 1: {}", min_steps_bfs(items.clone())?);

    // Throw the extra element pairs on first floor.
    items.push(vec![0u8, 0u8]);
    items.push(vec![0u8, 0u8]);
    println!("Part 2: {}", min_steps_bfs(items)?);

    Ok(())
}

fn min_steps_bfs(init_state: Vec<Vec<u8>>) -> Result<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(Node::new(init_state));

    // (elevator, normalized Node.item_pairs) -> # of steps to reach
    let mut visited: HashMap<(u8, Vec<Vec<u8>>), usize> = HashMap::new();

    while let Some(current) = queue.pop_front() {
        if current.is_end_state() {
            return Ok(current.steps);
        }

        // All element components behave the same so states that are identical but with different
        // element names swapped out are truly identical. Trimming identical states like this cuts
        // down enormously on the search space.
        let mut normalized = current.item_pairs.clone();
        normalized.sort_unstable();
        let key = (current.elevator, normalized);
        if let Some(&v_steps) = visited.get(&key) {
            if v_steps <= current.steps {
                continue;
            }
        }
        visited.insert(key, current.steps);

        current
            .neighbors()
            .into_iter()
            .for_each(|n| queue.push_back(n));
    }

    Err(anyhow::anyhow!("No path found!"))
}

#[derive(Clone, Debug)]
struct Node {
    item_pairs: Vec<Vec<u8>>,
    elevator: u8,
    steps: usize,
}

impl Node {
    fn new(item_pairs: Vec<Vec<u8>>) -> Self {
        Self {
            item_pairs,
            elevator: 0,
            steps: 0,
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        let movable = self.movable_indices();
        for moving in movable
            .iter()
            .cloned()
            .combinations(1)
            .chain(movable.iter().cloned().combinations(2))
            .collect::<Vec<_>>()
        {
            for &dir in &[1, -1] {
                let new_elevator = self.elevator as i8 + dir;
                if !(0..=3).contains(&new_elevator) {
                    continue;
                }
                let new_elevator = new_elevator as u8;
                let mut neighbor = self.clone();
                neighbor.elevator = new_elevator;
                neighbor.steps += 1;
                for &(i, j) in moving.iter() {
                    neighbor.item_pairs[i][j] = new_elevator;
                }
                if neighbor.is_valid_state() {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }

    // Get indices of all elements on the same floor as the elevator - e.g. (3,1) is the 4th generator
    fn movable_indices(&self) -> Vec<(usize, usize)> {
        self.item_pairs
            .iter()
            .enumerate()
            .flat_map(|(i, pair)| pair.iter().enumerate().map(move |(j, item)| (i, j, *item)))
            .filter_map(|(i, j, item)| {
                if item == self.elevator {
                    Some((i, j))
                } else {
                    None
                }
            })
            .collect()
    }

    // For each microchip: either we have our generator or no other generators are on our floor
    fn is_valid_state(&self) -> bool {
        for floor in 0..=3 {
            let gens: Vec<_> = self.item_pairs.iter().filter(|p| p[1] == floor).collect();
            for p in self.item_pairs.iter() {
                if p[0] == floor && p[1] != floor && !gens.is_empty() {
                    return false;
                }
            }
        }
        true
    }

    // Everything is on the 4th floor
    fn is_end_state(&self) -> bool {
        self.item_pairs
            .iter()
            .flat_map(|p| p.iter())
            .all(|i| *i == 3)
    }
}
