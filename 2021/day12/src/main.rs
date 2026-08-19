use anyhow::Result;
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let graph = parse_input()?;

    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));

    Ok(())
}

fn parse_input() -> Result<Graph> {
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    for (line_no, line) in stdin().lock().lines().enumerate() {
        let line = line?;

        let (from, to) = line
            .split_once('-')
            .ok_or_else(|| anyhow::anyhow!("Invalid input line(L{}): {}", line_no + 1, line))?;

        adj_list
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        adj_list
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }

    Ok(Graph { adj_list })
}

fn part1(graph: &Graph) -> usize {
    graph.path_count_dfs("end", &mut Context::new("start", Filter::UniqueSmall))
}

fn part2(graph: &Graph) -> usize {
    graph.path_count_dfs("end", &mut Context::new("start", Filter::DupeSmall))
}

#[derive(Debug)]
struct Graph {
    adj_list: HashMap<String, Vec<String>>,
}

struct Context {
    current: Vec<String>,
    used: HashMap<String, usize>,
    filter: Filter,
}

enum Filter {
    UniqueSmall,
    DupeSmall,
}

trait CaveExt {
    fn is_small_cave(&self) -> bool;
}

impl CaveExt for str {
    fn is_small_cave(&self) -> bool {
        self.chars().all(|c| c.is_ascii_lowercase())
    }
}

impl CaveExt for String {
    fn is_small_cave(&self) -> bool {
        self.as_str().is_small_cave()
    }
}

impl Context {
    fn new(start: &str, filter: Filter) -> Self {
        let current = vec![start.to_string()];
        let used = HashMap::from([(start.to_string(), 1)]);

        Self {
            current,
            used,
            filter,
        }
    }

    fn head(&self) -> &String {
        self.current.last().unwrap()
    }

    fn push(&mut self, to: String) {
        self.current.push(to.clone());
        self.used.entry(to).and_modify(|c| *c += 1).or_insert(1);
    }

    fn pop(&mut self) {
        let to = self.current.pop().unwrap();
        self.used.entry(to).and_modify(|c| *c -= 1);
    }

    fn filter_out(&self, to: &String) -> bool {
        let small_cave = to.is_small_cave();
        let to_visited = self.used.get(to).copied().unwrap_or(0);

        match self.filter {
            Filter::UniqueSmall => small_cave && to_visited > 0,
            Filter::DupeSmall => {
                let other_small_dupe = self
                    .used
                    .iter()
                    .any(|(c, &v)| c != to && v > 1 && c.is_small_cave());
                ((to == "start" || to == "end") && to_visited > 0)
                    || (small_cave && to_visited > 1)
                    || (small_cave && to_visited == 1 && other_small_dupe)
            }
        }
    }
}

impl Graph {
    fn path_count_dfs(&self, end: &str, current: &mut Context) -> usize {
        if current.head() == end {
            return 1;
        }

        let mut path_count = 0;
        for to in self.adj_list.get(current.head()).unwrap_or(&vec![]) {
            if current.filter_out(to) {
                continue;
            }

            current.push(to.clone());
            path_count += self.path_count_dfs(end, current);
            current.pop();
        }

        path_count
    }
}
