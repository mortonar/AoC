use anyhow::{Result, anyhow};
use regex::Regex;
use std::io;

fn main() -> Result<()> {
    let grid = parse_grid()?;

    let pairs = grid.viable_pairs();
    println!("Part 1: {}", pairs.len());

    // Print the grid and solve part 2 by hand. My input looked like this:
    // S..............................
    // ...............................
    // ..X............................
    // ..X............................
    // ..X......................._....   <-- Empty spot in this row
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // ..X............................
    // G.X............................
    //
    // An optimal solution is moving the empty (_) space up around the wall and down above G. Then
    // shuffling it and G (5 moves) to bring G up 32 spaces up to S.
    // 3 Up
    // 26 Left
    // 31 Down
    // 32 * 5 Up
    // total of 220 moves
    println!("Part 2:");
    let goal = grid.nodes.last().unwrap().first().unwrap().coords;
    for row in grid.nodes.iter() {
        for node in row.iter() {
            if node.coords == (0, 0) {
                print!("S");
            } else if node.coords == goal {
                print!("G");
            } else if node.used == 0 {
                print!("_");
            } else if pairs.contains(&node.coords) {
                print!(".");
            } else {
                // Node's data is too big to move anywhere else.
                print!("X");
            }
        }
        println!();
    }

    Ok(())
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Vec<StorageNode>>,
}

impl Grid {
    fn viable_pairs(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for n1 in self.nodes.iter().flat_map(|row| row.iter()) {
            for n2 in self.nodes.iter().flat_map(|row| row.iter()) {
                if n1 == n2 {
                    continue;
                }

                if !n1.is_empty() && n2.can_fit(n1) {
                    pairs.push(n1.coords);
                }
            }
        }
        pairs
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct StorageNode {
    coords: (usize, usize),
    size: usize,
    used: usize,
    has_goal_data: bool,
}

impl StorageNode {
    fn is_empty(&self) -> bool {
        self.used == 0
    }

    fn available(&self) -> usize {
        self.size - self.used
    }

    fn can_fit(&self, other: &Self) -> bool {
        other.used <= self.available()
    }
}

fn parse_grid() -> Result<Grid> {
    // Convert this to rows of columns structure (NOTE: these are parsed in order)
    let mut nodes = Vec::new();
    let node_parser = NodeParser::new()?;
    for line in io::stdin().lines().skip(2) {
        let line = line?;
        let node = node_parser.parse(&line)?;
        if nodes.len() < node.coords.0 + 1 {
            nodes.push(Vec::new());
        }
        nodes[node.coords.0].push(node);
    }
    Ok(Grid { nodes })
}

struct NodeParser {
    node_matcher: Regex,
}

impl NodeParser {
    fn new() -> Result<Self> {
        Ok(Self {
            node_matcher: Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)")?,
        })
    }

    fn parse(&self, s: &str) -> Result<StorageNode> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();

        if !self.node_matcher.is_match(tokens[0]) {
            return Err(anyhow!("Invalid node: {}", tokens[0]));
        }

        let (_full, [x, y]) = self
            .node_matcher
            .captures(tokens[0])
            .map(|c| c.extract())
            .unwrap();

        Ok(StorageNode {
            coords: (x.parse()?, y.parse()?),
            size: self.strip_suffix(tokens[1]).parse()?,
            used: self.strip_suffix(tokens[2]).parse()?,
            has_goal_data: false,
        })
    }

    fn strip_suffix<'a>(&self, s: &'a str) -> &'a str {
        let mut chars = s.chars();
        chars.next_back();
        chars.as_str()
    }
}
