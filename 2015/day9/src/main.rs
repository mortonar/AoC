use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

type City = String;

fn main() -> Result<()> {
    let mut all_cities = HashSet::new();
    let mut distance_map: HashMap<City, HashMap<City, usize>> = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        if tokens.len() != 5 {
            return Err(anyhow::anyhow!("Invalid input: {line}"));
        }
        let from = tokens[0];
        let to = tokens[2];
        let distance = tokens[4].parse::<usize>()?;
        all_cities.insert(from.to_string());
        all_cities.insert(to.to_string());
        distance_map
            .entry(from.to_string())
            .or_insert(HashMap::new())
            .insert(to.to_string(), distance);
        distance_map
            .entry(to.to_string())
            .or_insert(HashMap::new())
            .insert(from.to_string(), distance);
    }

    let distance_iter = all_cities
        .iter()
        .permutations(all_cities.len())
        .map(|perm| perm.windows(2).map(|w| distance_map[w[0]][w[1]]).sum());

    let min: usize = distance_iter.clone().min().unwrap();
    println!("Part 1: {min}");

    let max: usize = distance_iter.max().unwrap();
    println!("Part 2: {max}");

    Ok(())
}
