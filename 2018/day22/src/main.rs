use anyhow::Result;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use text_io::scan;

fn main() -> Result<()> {
    let (depth, target) = parse_input()?;

    let (risk_level, risk_table) = risk_level(depth, target);
    println!("Part 1: {risk_level}");
    println!("Part 2: {}", fastest_path_dijkstra(target, &risk_table));

    Ok(())
}

fn parse_input() -> Result<(usize, (usize, usize))> {
    let (depth, x, y): (usize, usize, usize);
    scan!("depth: {}", depth);
    scan!("target: {},{}", x, y);
    Ok((depth, (x, y)))
}

fn risk_level(depth: usize, target: (usize, usize)) -> (usize, Vec<Vec<Stats>>) {
    // Calculate a buffer past the target for potential use in part 2's search
    let (tx, ty) = (target.0 + 100, target.1 + 100);
    let mut risk_table: Vec<Vec<Stats>> = vec![vec![Stats::default(); tx + 1]; ty + 1];
    let mut risk_level = 0;

    for y in 0..=ty {
        for x in 0..=tx {
            risk_table[y][x].geo_index = match (y, x) {
                (0, 0) => 0,
                (y, x) if (x, y) == target => 0,
                (0, x) => x * 16807,
                (y, 0) => y * 48271,
                _ => risk_table[y - 1][x].erosion_level * risk_table[y][x - 1].erosion_level,
            };

            risk_table[y][x].erosion_level = (risk_table[y][x].geo_index + depth) % 20183;

            let region_type_val = risk_table[y][x].erosion_level % 3;
            risk_table[y][x].region_type = region_type_val.into();
            if y <= target.1 && x <= target.0 {
                risk_level += region_type_val;
            }
        }
    }

    (risk_level, risk_table)
}

fn fastest_path_dijkstra((tx, ty): (usize, usize), risk_table: &[Vec<Stats>]) -> usize {
    let mut queue = BinaryHeap::new();
    // Visited states: (x, y, equipment)
    let mut dist: HashMap<(usize, usize, Equipment), usize> = HashMap::new();

    let start = SearchContext::default();
    queue.push(Reverse(start));
    dist.insert((start.x, start.y, start.equipped), 0);

    while let Some(Reverse(current)) = queue.pop() {
        if (current.x, current.y) == (tx, ty) && current.equipped == Equipment::Torch {
            return current.minutes;
        }

        let key = (current.x, current.y, current.equipped);
        if dist.get(&key).is_some_and(|&d| d < current.minutes) {
            continue;
        }

        for neighbor in current.neighbors(risk_table) {
            let n_key = (neighbor.x, neighbor.y, neighbor.equipped);
            let entry = dist.entry(n_key).or_insert(usize::MAX);
            if neighbor.minutes < *entry {
                *entry = neighbor.minutes;
                queue.push(Reverse(neighbor));
            }
        }
    }

    panic!("No path found to target");
}

#[derive(Debug, Default, Clone, Copy)]
struct Stats {
    geo_index: usize,
    erosion_level: usize,
    region_type: RegionType,
}

#[derive(Debug, Default, Clone, Copy)]
enum RegionType {
    #[default]
    Rocky,
    Wet,
    Narrow,
}

impl From<usize> for RegionType {
    fn from(value: usize) -> Self {
        match value {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            _ => RegionType::Narrow,
        }
    }
}

impl RegionType {
    const fn valid_equipment(self) -> [Equipment; 2] {
        match self {
            RegionType::Rocky => [Equipment::ClimbingGear, Equipment::Torch],
            RegionType::Wet => [Equipment::ClimbingGear, Equipment::None],
            RegionType::Narrow => [Equipment::Torch, Equipment::None],
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct SearchContext {
    minutes: usize,
    y: usize,
    x: usize,
    equipped: Equipment,
}

impl PartialOrd for SearchContext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchContext {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.minutes.cmp(&other.minutes)
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum Equipment {
    #[default]
    Torch,
    ClimbingGear,
    None,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl SearchContext {
    fn neighbors(&self, risk_table: &[Vec<Stats>]) -> Vec<SearchContext> {
        let mut neighbors = Vec::new();

        // Switch equipment without moving (7 minutes)
        for equipped in risk_table[self.y][self.x].region_type.valid_equipment() {
            if equipped != self.equipped {
                neighbors.push(SearchContext {
                    minutes: self.minutes + 7,
                    equipped,
                    ..*self
                });
            }
        }

        // Move to adjacent cell keeping same equipment (1 minute)
        for (dy, dx) in DIRECTIONS {
            let (ny, nx) = (
                self.y.wrapping_add_signed(dy),
                self.x.wrapping_add_signed(dx),
            );
            // Out of bounds
            if ny >= risk_table.len() || nx >= risk_table[ny].len() {
                continue;
            }

            if risk_table[ny][nx]
                .region_type
                .valid_equipment()
                .contains(&self.equipped)
            {
                neighbors.push(SearchContext {
                    y: ny,
                    x: nx,
                    minutes: self.minutes + 1,
                    equipped: self.equipped,
                });
            }
        }

        neighbors
    }
}
