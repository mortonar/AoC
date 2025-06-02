use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::env;
use std::io::BufRead;

fn main() -> Result<()> {
    let to_store: usize = env::args().nth(1).unwrap_or("150".to_string()).parse()?;

    let containers: Vec<usize> = {
        let mut containers = Vec::new();
        for line in std::io::stdin().lock().lines() {
            containers.push(line?.parse()?);
        }
        containers
    };

    let min = fill_bfs(&containers, to_store, false);
    println!("Part 1: {}", min);
    let min = fill_bfs(&containers, to_store, true);
    println!("Part 2: {}", min);

    Ok(())
}

fn fill_bfs(containers: &[usize], to_store: usize, find_min: bool) -> usize {
    let mut queue = VecDeque::new();
    containers.iter().enumerate().for_each(|(i, &c)| {
        if to_store >= c {
            queue.push_back((vec![i], to_store - c));
        }
    });

    let mut unique = HashSet::new();
    let mut depth_limit = 0;
    while !queue.is_empty() {
        let (mut path, remaining) = queue.pop_front().unwrap();

        if remaining == 0 {
            if find_min {
                depth_limit = path.len();
                queue.push_back((path, remaining));
                break;
            } else {
                path.sort();
                unique.insert(path);
                continue;
            }
        }

        containers.iter().enumerate().for_each(|(i, &c)| {
            if remaining >= c && !path.contains(&i) {
                let mut path_c = path.clone();
                path_c.push(i);
                queue.push_back((path_c, remaining - c));
            }
        });
    }

    if find_min {
        for (mut path, remaining) in queue.into_iter() {
            if path.len() == depth_limit && remaining == 0 {
                path.sort();
                unique.insert(path);
            }
        }
    }
    unique.len()
}
