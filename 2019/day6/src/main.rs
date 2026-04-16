use anyhow::{Result, bail};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::BufRead;
use text_io::try_scan;

fn main() -> Result<()> {
    let graph = parse_input()?;

    println!("Part 1: {}", graph.orbit_count_dfs("COM", 0));
    println!("Part 2: {}", graph.min_transfers_bfs("YOU", "SAN")?);

    Ok(())
}

fn parse_input() -> Result<Graph> {
    let mut edges = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let (orbited, orbiting): (String, String);
        try_scan!(line.bytes() => "{}){}", orbited, orbiting);

        edges
            .entry(orbited)
            .and_modify(|al: &mut HashSet<_>| {
                al.insert(orbiting.clone());
            })
            .or_insert(HashSet::from([orbiting]));
    }

    Ok(Graph { edges })
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn orbit_count_dfs(&self, current: &str, depth: usize) -> usize {
        // Depth is direct + indirect
        if !self.edges.contains_key(current) {
            return depth;
        }

        depth
            + self.edges[current]
                .iter()
                .map(|orbiting| self.orbit_count_dfs(orbiting, depth + 1))
                .sum::<usize>()
    }

    fn min_transfers_bfs(&self, source: &str, target: &str) -> Result<usize> {
        let mut undirected_edges = self.edges.clone();
        for (from, to) in &self.edges {
            for t in to.iter() {
                undirected_edges
                    .entry(t.to_owned())
                    .and_modify(|al: &mut HashSet<_>| {
                        al.insert(from.to_owned());
                    })
                    .or_insert(HashSet::from([from.to_owned()]));
            }
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((source, 0));
        visited.insert(source);

        while let Some((current, transfers)) = queue.pop_front() {
            // - 2 since we want the distance between the objects source and target are orbiting
            if current == target {
                return Ok(transfers - 2);
            }

            if let Some(neighbors) = undirected_edges.get(current) {
                for neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back((neighbor, transfers + 1));
                    }
                }
            }
        }

        bail!("No transfer path found")
    }
}
