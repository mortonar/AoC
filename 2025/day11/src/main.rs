use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::stdin;

fn main() -> Result<()> {
    let graph = parse_graph()?;
    let part1 = path_count(&graph, "you", "out");
    println!("Part 1: {part1}");

    // Path memoization doesn't work if you're trying to filter on paths involving "fft" and "dac"
    // because you don't know which part of the path should include them. BUT it does work if you
    // consider all paths. Breaking this search problem up into steps then becomes a good fit for
    // DFS with memoization. The only reason we can do that is that there are no paths from "dac"
    // to "fft" - so we don't have to worry about double-counting.
    let part2 = [["svr", "fft"], ["fft", "dac"], ["dac", "out"]]
        .iter()
        .map(|&[from, to]| path_count(&graph, from, to))
        .product::<usize>();
    println!("Part 2: {part2}");

    Ok(())
}

fn path_count(graph: &Graph, start: &str, goal: &str) -> usize {
    let start = DFSNode::new(vec![start.to_string()]);
    let mut memo: HashMap<String, usize> = HashMap::new();
    paths_to_out_dfs(graph, start, goal, &mut memo)
}

fn paths_to_out_dfs(
    graph: &Graph,
    current: DFSNode,
    goal: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(cached) = memo.get(current.head()) {
        return *cached;
    }

    if current.head() == goal {
        return 1;
    }

    let mut result = 0;
    for next in graph.connections.get(current.head()).unwrap() {
        if !current.path.contains(next) {
            let mut path = current.path.clone();
            path.push(next.to_string());

            result += paths_to_out_dfs(graph, DFSNode::new(path), goal, memo);
        }
    }

    memo.insert(current.head().to_string(), result);
    result
}

struct DFSNode {
    path: Vec<String>,
}

impl DFSNode {
    fn new(path: Vec<String>) -> Self {
        Self { path }
    }
    fn head(&self) -> &str {
        self.path.last().unwrap().as_str()
    }
}

#[derive(Debug)]
struct Graph {
    connections: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new(connections: HashMap<String, HashSet<String>>) -> Self {
        Self { connections }
    }
}

fn parse_graph() -> Result<Graph> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for line in stdin().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();

        let node = tokens[0];
        let node = node[0..node.len() - 1].to_string();

        for connect_to in tokens.iter().skip(1) {
            let connect_to = connect_to.to_string();

            // Ensure the node we're connecting to at least has an empty connection list if there
            // are no connections from it.
            connections.entry(connect_to.clone()).or_default();

            connections
                .entry(node.clone())
                .and_modify(|conns| {
                    conns.insert(connect_to.clone());
                })
                .or_insert(HashSet::from([connect_to]));
        }
    }

    Ok(Graph::new(connections))
}
