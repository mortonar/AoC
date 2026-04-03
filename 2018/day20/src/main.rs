use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::stdin;

fn main() -> Result<()> {
    let regex = parse_input()?;

    let mut graph = Graph::default();
    build_graph(&regex, (0, 0), &mut graph);
    let (max_depth, far_rooms) = max_depth_bfs(&graph);
    println!("Part 1: {max_depth}");
    println!("Part 2: {far_rooms}");

    Ok(())
}

// Trim the '^' and '$' anchors since they're always supplied
fn parse_input() -> Result<Vec<char>> {
    let mut regex = String::new();
    stdin().read_line(&mut regex)?;
    Ok(regex[1..regex.trim().len() - 1].chars().collect())
}

type Graph = HashMap<(isize, isize), HashSet<(isize, isize)>>;

type Positions = Vec<(isize, isize)>;

fn build_graph(regex: &[char], start: (isize, isize), graph: &mut Graph) {
    // Stack of: (branch start positions, accumulated ends)
    let mut branch_stack: Vec<(Positions, Positions)> = Vec::new();
    // Current branch ends
    let mut positions: Positions = vec![start];

    for &c in regex.iter() {
        match c {
            '(' => {
                // Save current positions as branch starts
                branch_stack.push((positions.clone(), Vec::new()));
            }
            '|' => {
                // Current branch is done — accumulate its ending positions then reset current
                // positions to the branch starts
                let (starts, ends) = branch_stack.last_mut().unwrap();
                ends.extend_from_slice(&positions);
                positions = starts.clone();
            }
            ')' => {
                // Final branch done — union its exits with all prior branch exits
                let (_, mut ends) = branch_stack.pop().unwrap();
                ends.extend_from_slice(&positions);
                // Cut down on duplicate positions
                ends.sort_unstable();
                ends.dedup();
                positions = ends;
            }
            'N' | 'E' | 'S' | 'W' => {
                positions = positions
                    .iter()
                    .map(|pos| {
                        let new_pos = pos.apply_dir(c);
                        graph.entry(*pos).or_default().insert(new_pos);
                        // Add reverse edge for BFS
                        graph.entry(new_pos).or_default().insert(*pos);
                        new_pos
                    })
                    .collect();
                // Cut down on duplicate positions
                positions.sort_unstable();
                positions.dedup();
            }
            dir => panic!("Invalid direction: {dir}"),
        }
    }
}

fn max_depth_bfs(graph: &Graph) -> (usize, usize) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(((0, 0), 0));
    visited.insert((0, 0));

    let mut max_depth = 0;
    let mut far_rooms = 0;

    while let Some((pos, depth)) = queue.pop_front() {
        max_depth = max_depth.max(depth);

        if depth >= 1000 {
            far_rooms += 1;
        }

        if let Some(adjacent) = graph.get(&pos) {
            for &adj in adjacent {
                if visited.insert(adj) {
                    queue.push_back((adj, depth + 1));
                }
            }
        }
    }

    (max_depth - 1, far_rooms)
}

trait ApplyDir {
    fn apply_dir(&self, dir: char) -> Self;
}

impl ApplyDir for (isize, isize) {
    fn apply_dir(&self, dir: char) -> Self {
        match dir {
            'N' => (self.0 - 1, self.1),
            'S' => (self.0 + 1, self.1),
            'W' => (self.0, self.1 - 1),
            'E' => (self.0, self.1 + 1),
            dir => panic!("Invalid direction: {dir}"),
        }
    }
}
