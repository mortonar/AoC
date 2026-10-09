use std::{
    collections::{BTreeSet, BinaryHeap, HashMap, VecDeque},
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let graph = parse_input()?;

    println!("Part 1: {}", part1(&graph)?);
    println!("Part 2: {}", part2(&graph)?);

    Ok(())
}

fn parse_input() -> Result<Graph> {
    let valves = stdin()
        .lock()
        .lines()
        .map(|line| {
            let valve: Valve = line?.parse()?;
            Ok((valve.label.clone(), valve))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    Ok(Graph { valves })
}

fn part1(graph: &Graph) -> Result<usize> {
    Ok(graph
        .best_per_opened_set(30)
        .into_values()
        .max()
        .unwrap_or(0))
}

fn part2(graph: &Graph) -> Result<usize> {
    let best = graph.best_per_opened_set(26);
    let entries: Vec<_> = best.into_iter().collect();

    // Find the best (highest relief) pair of distinct open sets
    let mut best_combined = 0;
    for (i, (mine, mine_relieved)) in entries.iter().enumerate() {
        for (theirs, theirs_relieved) in &entries[i + 1..] {
            if mine.is_disjoint(theirs) {
                best_combined = best_combined.max(mine_relieved + theirs_relieved);
            }
        }
    }

    Ok(best_combined)
}

impl Graph {
    // For every distinct set of opened valves reachable within `start_time` minutes,
    // find the best total pressure relieved by ending on exactly that set.
    fn best_per_opened_set(&self, start_time: usize) -> HashMap<BTreeSet<String>, usize> {
        let distances = self.distances();
        let useful: Vec<String> = self
            .valves
            .values()
            .filter(|v| v.flow > 0)
            .map(|v| v.label.clone())
            .collect();

        let start = Context {
            loc: "AA".to_string(),
            relieved: 0,
            time: start_time,
            opened: BTreeSet::new(),
        };
        let mut queue = BinaryHeap::from([start]);
        let mut best: HashMap<BTreeSet<String>, usize> = HashMap::new();

        while let Some(next) = queue.pop() {
            let entry = best.entry(next.opened.clone()).or_insert(0);
            *entry = (*entry).max(next.relieved);

            for target in &useful {
                if next.opened.contains(target) {
                    continue;
                }

                let cost = distances[&next.loc][target] + 1;
                if cost > next.time {
                    continue;
                }

                let time = next.time - cost;
                let mut opened = next.opened.clone();
                opened.insert(target.clone());

                queue.push(Context {
                    loc: target.clone(),
                    relieved: next.relieved + time * self.valves[target].flow,
                    time,
                    opened,
                });
            }
        }

        best
    }

    // Shortest distance (in minutes) between every valve with nonzero flow, plus "AA"
    fn distances(&self) -> HashMap<String, HashMap<String, usize>> {
        self.valves
            .values()
            .filter(|v| v.flow > 0 || v.label == "AA")
            .map(|v| (v.label.clone(), self.bfs_from(&v.label)))
            .collect()
    }

    fn bfs_from(&self, start: &str) -> HashMap<String, usize> {
        let mut dist = HashMap::from([(start.to_string(), 0)]);
        let mut queue = VecDeque::from([start.to_string()]);

        while let Some(cur) = queue.pop_front() {
            let d = dist[&cur];
            for next in &self.valves[&cur].tunnels {
                if !dist.contains_key(next) {
                    dist.insert(next.clone(), d + 1);
                    queue.push_back(next.clone());
                }
            }
        }

        dist
    }
}

#[derive(Debug)]
struct Graph {
    valves: HashMap<String, Valve>,
}

#[derive(Debug)]
struct Valve {
    label: String,
    flow: usize,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone)]
struct Context {
    loc: String,
    relieved: usize,
    time: usize,
    opened: BTreeSet<String>,
}

impl Ord for Context {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.relieved.cmp(&other.relieved)
    }
}

impl PartialOrd for Context {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Context {}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let Some((flow, tunnels)) = s.split_once("; ") else {
            bail!("ill-formatted Valve: {s}");
        };

        let tokens: Vec<_> = flow.split_ascii_whitespace().collect();
        let label = tokens[1].to_string();
        let flow = tokens[4][5..].parse()?;

        let tunnels = tunnels
            .split_ascii_whitespace()
            .skip(4)
            .map(|t| t.strip_suffix(',').unwrap_or(t).to_string())
            .collect();

        Ok(Valve {
            label,
            flow,
            tunnels,
        })
    }
}
