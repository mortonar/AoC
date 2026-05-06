use anyhow::{Result, anyhow, bail};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let map = parse_input()?;
    println!("Part 1: {}", map.solve(&[map.entrance])?);

    let (part2_map, robots) = map.split_for_part2();
    println!("Part 2: {}", part2_map.solve(&robots)?);

    Ok(())
}

fn parse_input() -> Result<Map> {
    let mut grid = Vec::new();
    let mut keys = Vec::new();
    let mut entrance = None;

    for (y, line) in stdin().lock().lines().enumerate() {
        let row = line?.trim().chars().collect::<Vec<_>>();
        for (x, &c) in row.iter().enumerate() {
            match c {
                '@' => entrance = Some(Coords::new(x, y)),
                'a'..='z' => keys.push((Coords::new(x, y), c)),
                'A'..='Z' | '#' | '.' => {}
                _ => bail!("Invalid character '{}' at ({},{})", c, x, y),
            }
        }
        grid.push(row);
    }

    let entrance = entrance.ok_or_else(|| anyhow!("No entrance found"))?;
    grid[entrance.y][entrance.x] = '.';

    Ok(Map {
        grid,
        keys,
        entrance,
    })
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    keys: Vec<(Coords, char)>,
    entrance: Coords,
}

impl Map {
    fn solve(&self, starts: &[Coords]) -> Result<usize> {
        CompressedGraph::build(self, starts).shortest_path_collect_all(starts.len())
    }

    fn split_for_part2(&self) -> (Map, [Coords; 4]) {
        let mut part2 = self.clone();
        let Coords { x, y } = self.entrance;

        for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            part2.grid[(y as isize + dy) as usize][(x as isize + dx) as usize] = '#';
        }

        let robots = [(-1, -1), (1, -1), (-1, 1), (1, 1)].map(|(dx, dy)| {
            let (rx, ry) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            part2.grid[ry][rx] = '.';
            Coords::new(rx, ry)
        });

        (part2, robots)
    }

    fn in_bounds(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if y >= 0
            && y < self.grid.len() as isize
            && x >= 0
            && x < self.grid[y as usize].len() as isize
            && self.grid[y as usize][x as usize] != '#'
        {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }
}

#[derive(Debug, Clone)]
struct CompressedGraph {
    nodes: Vec<Node>,
    adj_list: Vec<Vec<Edge>>,
    all_keys_mask: KeyMask,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    pos: Coords,
    kind: NodeKind,
}

#[derive(Debug, Clone, Copy)]
enum NodeKind {
    Start,
    Key(u8),
}

type KeyMask = u32;

#[derive(Debug, Clone, Copy)]
struct Edge {
    to_node: usize,
    dist: usize,
    required: KeyMask,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct SearchState {
    robot_node_indices: [usize; 4],
    robot_count: u8,
    keys: KeyMask,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct QueueEntry {
    cost: usize,
    state: SearchState,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CompressedGraph {
    fn build(map: &Map, starts: &[Coords]) -> Self {
        let mut nodes: Vec<Node> = starts
            .iter()
            .map(|&pos| Node {
                pos,
                kind: NodeKind::Start,
            })
            .collect();

        let mut key_node_indices = [None::<usize>; 26];
        let mut all_keys_mask = 0;
        for &(pos, key) in &map.keys {
            let idx = key.key_idx();
            key_node_indices[idx] = Some(nodes.len());
            nodes.push(Node {
                pos,
                kind: NodeKind::Key(idx as u8),
            });
            all_keys_mask |= key_bit(idx);
        }

        let adj_list = (0..nodes.len())
            .map(|source_id| Self::edges_from_source_bfs(map, &nodes, &key_node_indices, source_id))
            .collect();

        CompressedGraph {
            nodes,
            adj_list,
            all_keys_mask,
        }
    }

    fn edges_from_source_bfs(
        map: &Map,
        nodes: &[Node],
        key_node_indices: &[Option<usize>; 26],
        source_id: usize,
    ) -> Vec<Edge> {
        #[derive(Clone, Copy)]
        struct BfsState {
            pos: Coords,
            dist: usize,
            required: KeyMask,
        }

        let mut queue = VecDeque::new();
        queue.push_back(BfsState {
            pos: nodes[source_id].pos,
            dist: 0,
            required: 0,
        });

        let mut visited: HashMap<Coords, Vec<(KeyMask, usize)>> = HashMap::new();
        visited.insert(nodes[source_id].pos, vec![(0, 0)]);

        let mut edge_candidates: HashMap<usize, Vec<(KeyMask, usize)>> = HashMap::new();

        while let Some(cur) = queue.pop_front() {
            let Coords { x, y } = cur.pos;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let Some((nx, ny)) = map.in_bounds(x as isize + dx, y as isize + dy) else {
                    continue;
                };
                let tile = map.grid[ny][nx];

                let required = if tile.is_door() {
                    cur.required | tile.door_bit()
                } else {
                    cur.required
                };

                let next = BfsState {
                    pos: Coords::new(nx, ny),
                    dist: cur.dist + 1,
                    required,
                };

                // Have we been to this space previously but with same or fewer steps/keys? If so,
                // skip expansion.
                let frontier = visited.entry(next.pos).or_default();
                if Self::is_dominated(frontier, required, next.dist) {
                    continue;
                }
                // If not, remove any visited states that are worse.
                Self::keep_non_dominated(frontier, required, next.dist);
                queue.push_back(next);

                if tile.is_key() {
                    let to_id =
                        key_node_indices[tile.key_idx()].expect("key tile must have a graph node");
                    if to_id != source_id {
                        let opts = edge_candidates.entry(to_id).or_default();
                        if !Self::is_dominated(opts, required, next.dist) {
                            Self::keep_non_dominated(opts, required, next.dist);
                        }
                    }
                }
            }
        }

        edge_candidates
            .into_iter()
            .flat_map(|(to, opts)| {
                opts.into_iter().map(move |(required, dist)| Edge {
                    to_node: to,
                    dist,
                    required,
                })
            })
            .collect()
    }

    fn is_dominated(existing: &[(KeyMask, usize)], req: KeyMask, dist: usize) -> bool {
        existing
            .iter()
            .any(|&(or, od)| od <= dist && (or & req) == or)
    }

    fn keep_non_dominated(states: &mut Vec<(KeyMask, usize)>, req: KeyMask, dist: usize) {
        // Keep old if: new is longer OR new requires more keys
        states.retain(|&(or, od)| dist > od || (req & or) != req);
        states.push((req, dist));
    }

    fn shortest_path_collect_all(&self, robot_count: usize) -> Result<usize> {
        // Map to start Node indices in graph
        let mut start_robots = [0usize; 4];
        start_robots[..robot_count]
            .iter_mut()
            .enumerate()
            .for_each(|(i, r)| *r = i);

        let start = SearchState {
            robot_node_indices: start_robots,
            keys: 0,
            robot_count: robot_count as u8,
        };

        let mut visited: HashMap<SearchState, usize> = HashMap::from([(start, 0)]);
        let mut queue = BinaryHeap::from([QueueEntry {
            cost: 0,
            state: start,
        }]);

        while let Some(QueueEntry { cost, state }) = queue.pop() {
            if cost > visited[&state] {
                continue;
            }
            if state.keys == self.all_keys_mask {
                return Ok(cost);
            }

            for robot_idx in 0..state.robot_count as usize {
                for &edge in &self.adj_list[state.robot_node_indices[robot_idx]] {
                    let NodeKind::Key(key_index) = self.nodes[edge.to_node].kind else {
                        continue;
                    };
                    let key_bit = key_bit(key_index as usize);

                    // We've already collected this key
                    if state.keys & key_bit != 0 {
                        continue;
                    }
                    // We don't have all the required keys for this edge
                    if edge.required & !state.keys != 0 {
                        continue;
                    }

                    let mut next = state;
                    next.robot_node_indices[robot_idx] = edge.to_node;
                    next.keys |= key_bit;
                    let next_cost = cost + edge.dist;

                    if visited.get(&next).is_none_or(|&v| next_cost < v) {
                        visited.insert(next, next_cost);
                        queue.push(QueueEntry {
                            cost: next_cost,
                            state: next,
                        });
                    }
                }
            }
        }

        bail!("Path not found")
    }
}

trait KeyDoor {
    fn key_idx(&self) -> usize;
    fn door_bit(&self) -> KeyMask;
    fn is_door(&self) -> bool;
    fn is_key(&self) -> bool;
}

impl KeyDoor for char {
    // a = 0, b = 0, ...
    fn key_idx(&self) -> usize {
        (*self as u8 - b'a') as usize
    }

    fn door_bit(&self) -> KeyMask {
        1 << (*self as u8 - b'A')
    }

    fn is_door(&self) -> bool {
        self.is_ascii_uppercase()
    }

    fn is_key(&self) -> bool {
        self.is_ascii_lowercase()
    }
}

fn key_bit(idx: usize) -> KeyMask {
    1 << idx
}
