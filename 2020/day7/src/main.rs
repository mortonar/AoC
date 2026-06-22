use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let graph = parse_input()?;

    println!("Part 1: {}", graph.paths_to("shiny gold"));
    println!("Part 2: {}", graph.total_bags("shiny gold", 1) - 1);

    Ok(())
}

fn parse_input() -> Result<BagGraph> {
    let mut adj_list = HashMap::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.trim().split_ascii_whitespace().collect();

        let bag: String = tokens[0..=1].to_vec().join(" ");

        adj_list.insert(bag.clone(), HashSet::new());
        if line.contains("no other bags") {
            continue;
        }
        for i in 4..tokens.len() {
            if tokens[i].contains("bag") {
                let (amount, color) = (
                    tokens[i - 3].parse()?,
                    tokens[i - 2..=(i - 1)].to_vec().join(" "),
                );
                adj_list.get_mut(&bag).unwrap().insert((color, amount));
            }
        }
    }

    Ok(BagGraph { adj_list })
}

#[derive(Debug)]
struct BagGraph {
    /// Container -> contains with count
    adj_list: HashMap<String, HashSet<(String, usize)>>,
}

impl BagGraph {
    fn paths_to(&self, color: &str) -> usize {
        let mut paths = 0;
        let mut queue = VecDeque::from(vec![color]);
        let mut visited = HashSet::new();
        visited.insert(color);

        while let Some(bag) = queue.pop_front() {
            let contains = self
                .adj_list
                .iter()
                .filter(|(_container, contained)| contained.iter().any(|(b, _count)| b == bag));
            for (container, _contained) in contains.into_iter() {
                if visited.insert(container) {
                    paths += 1;
                    queue.push_back(container);
                }
            }
        }

        paths
    }

    fn total_bags(&self, color: &str, count: usize) -> usize {
        let contained = self.adj_list.get(color).unwrap();
        if contained.is_empty() {
            count
        } else {
            contained
                .iter()
                .map(|(bag, c)| count * self.total_bags(bag, *c))
                .sum::<usize>()
                + count
        }
    }
}
