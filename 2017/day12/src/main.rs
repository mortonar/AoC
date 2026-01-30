use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let graph = parse_input()?;
    let components = graph.connected_components();
    let containing_0 = components
        .iter()
        .find(|c| c.contains(&0))
        .ok_or(anyhow!("Component containing 0 not found"))?
        .iter()
        .count();
    println!("Part 1: {containing_0}");
    println!("Part 2: {}", components.len());
    Ok(())
}

#[derive(Debug)]
struct Graph {
    programs: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    fn connected_components(&self) -> Vec<HashSet<usize>> {
        let mut groups = Vec::new();
        let mut visited = HashSet::new();

        for &start in self.programs.keys() {
            if visited.contains(&start) {
                continue;
            }

            let mut component = HashSet::new();
            let mut stack = vec![start];

            while let Some(node) = stack.pop() {
                if !visited.insert(node) {
                    continue;
                }
                component.insert(node);

                if let Some(neighbors) = self.programs.get(&node) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }

            groups.push(component);
        }

        groups
    }
}

fn parse_input() -> Result<Graph> {
    let mut programs = HashMap::new();

    for line in stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split(" <-> ").collect();
        let left: usize = tokens[0].parse()?;
        let right: Vec<usize> = tokens[1]
            .split(',')
            .map(|r| r.trim().parse())
            .collect::<Result<_, _>>()?;
        for &r in &right {
            programs.entry(left).or_insert_with(HashSet::new).insert(r);
            programs.entry(r).or_insert_with(HashSet::new).insert(left);
        }
    }

    Ok(Graph { programs })
}
