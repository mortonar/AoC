use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn main() -> Result<()> {
    let mut adj_list = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let from_to = line.split('-').collect::<Vec<&str>>();
        assert_eq!(from_to.len(), 2, "Invalid input line");
        for (i, j) in [(0, 1), (1, 0)] {
            adj_list
                .entry(from_to[i].to_string())
                .or_insert(vec![])
                .push(from_to[j].to_string());
        }
    }
    let graph = Graph::new(adj_list);

    let largest = graph.largest_strongly_connected_component();
    let mut password: Vec<String> = largest.iter().map(|e| e.clone()).collect();
    password.sort();
    println!("{}", password.join(","));

    Ok(())
}

struct Graph {
    adj_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(adj_list: HashMap<String, Vec<String>>) -> Self {
        Self { adj_list }
    }

    fn largest_strongly_connected_component(&self) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut largest_component = HashSet::new();

        for node in self.adj_list.keys() {
            if !visited.contains(node) {
                let mut component = HashSet::new();
                self.dfs(node, &mut component, &mut visited);

                if component.len() > largest_component.len() {
                    largest_component = component;
                }
            }
        }

        largest_component
    }

    fn dfs(&self, node: &str, component: &mut HashSet<String>, visited: &mut HashSet<String>) {
        visited.insert(node.to_string());
        component.insert(node.to_string());

        for neighbor in self.adj_list[node].iter() {
            if !component.contains(neighbor)
                // All nodes in the component must be directly connected to each other
                && component
                    .iter()
                    .all(|c| self.adj_list[c].contains(neighbor))
            {
                self.dfs(neighbor, component, visited);
            }
        }
    }
}
