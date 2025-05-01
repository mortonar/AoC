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
        all_cities.insert(String::from(tokens[0]));
        all_cities.insert(String::from(tokens[2]));
        distance_map
            .entry(String::from(tokens[0]))
            .or_insert(HashMap::new())
            .insert(String::from(tokens[2]), tokens[4].parse()?);
        distance_map
            .entry(String::from(tokens[2]))
            .or_insert(HashMap::new())
            .insert(String::from(tokens[0]), tokens[4].parse()?);
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
