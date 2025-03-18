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

    let conn_components = graph.connected_components();
    println!("{}", conn_components.len());

    Ok(())
}

struct Graph {
    adj_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(adj_list: HashMap<String, Vec<String>>) -> Self {
        Self { adj_list }
    }

    fn connected_components(&self) -> HashSet<String> {
        let mut components: HashSet<String> = HashSet::new();

        for (from, tos) in &self.adj_list {
            if from.starts_with('t') {
                for i in 0..tos.len() - 1 {
                    for j in i + 1..tos.len() {
                        if self.adj_list[&tos[i]].contains(&tos[j]) {
                            let mut joined =
                                vec![from.to_string(), tos[i].to_string(), tos[j].to_string()];
                            joined.sort();
                            components.insert(joined.join("-"));
                        }
                    }
                }
            }
        }

        components
    }
}
