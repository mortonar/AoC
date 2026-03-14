use anyhow::{Error, Result};
use std::io::stdin;

fn main() -> Result<()> {
    let root = parse_input()?.to_node(&mut 0);
    println!("Part 1 {}", root.metadata_sum());
    println!("Part 2 {}", root.value());

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;

    line.trim()
        .split_ascii_whitespace()
        .map(|n| n.parse().map_err(Error::from))
        .collect()
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

trait ToNode {
    fn to_node(&self, idx: &mut usize) -> Node;
}

impl ToNode for Vec<usize> {
    fn to_node(&self, idx: &mut usize) -> Node {
        let (children, metadata) = (self[*idx], self[*idx + 1]);
        *idx += 2;

        let children: Vec<_> = (0..children).map(|_| self.to_node(idx)).collect();

        let metadata: Vec<_> = (0..metadata).map(|i| self[*idx + i]).collect();
        *idx += metadata.len();

        Node { children, metadata }
    }
}

impl Node {
    fn metadata_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.children.iter().map(Node::metadata_sum).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                // Convert from 1-based index to 0-based
                .filter(|&&m| m > 0)
                .filter_map(|&m| self.children.get(m - 1))
                .map(Node::value)
                .sum()
        }
    }
}
