use anyhow::{Error, Result};
use std::cmp::max;
use std::collections::VecDeque;
use std::io;

fn main() -> Result<()> {
    let passcode = io::stdin()
        .lines()
        .next()
        .ok_or_else(|| Error::msg("Expected passcode from stdin"))??;

    println!("Part 1: {}", min_path_bfs(&passcode)?);
    println!("Part 2: {}", max_path_bfs(&passcode)?);

    Ok(())
}

fn min_path_bfs(passcode: &str) -> Result<String> {
    let mut queue = VecDeque::new();
    queue.push_back(Node::default());

    while let Some(current) = queue.pop_front() {
        if current.coord == (3, 3) {
            return Ok(current.path.iter().collect());
        }
        current
            .neighbors(passcode)
            .into_iter()
            .for_each(|n| queue.push_back(n));
    }

    Err(Error::msg("No path found"))
}

fn max_path_bfs(passcode: &str) -> Result<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(Node::default());
    let mut longest = 0;

    while let Some(current) = queue.pop_front() {
        if current.coord == (3, 3) {
            longest = max(longest, current.path.len());
            // Don't let the path continue through the vault room or the search will never end...
            continue;
        }
        current
            .neighbors(passcode)
            .into_iter()
            .for_each(|n| queue.push_back(n));
    }

    Ok(longest)
}

type Coordinate = (isize, isize);

const ORIENTATIONS: [&(isize, isize, char); 4] =
    [&(-1, 0, 'U'), &(1, 0, 'D'), &(0, -1, 'L'), &(0, 1, 'R')];

#[derive(Debug, Default)]
struct Node {
    coord: Coordinate,
    path: Vec<char>,
}

impl Node {
    fn neighbors(&self, passcode: &str) -> Vec<Node> {
        let hash = md5::compute(format!(
            "{}{}",
            passcode,
            self.path.iter().collect::<String>()
        ));
        let hash = format!("{:x}", hash);
        hash.chars()
            .take(4)
            .zip(ORIENTATIONS.iter())
            .filter_map(|(char, &&(dx, dy, dir))| {
                let new_coord = (self.coord.0 + dx, self.coord.1 + dy);
                if !(0..4).contains(&new_coord.0) || !(0..4).contains(&new_coord.1) {
                    return None;
                }
                match char {
                    'b' | 'c' | 'd' | 'e' | 'f' => {
                        let mut neighbor = Self {
                            coord: new_coord,
                            path: self.path.clone(),
                        };
                        neighbor.path.push(dir);
                        Some(neighbor)
                    }
                    _ => None,
                }
            })
            .collect()
    }
}
