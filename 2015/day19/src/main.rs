use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn main() -> Result<()> {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    let mut lines = std::io::stdin().lock().lines();
    loop {
        let line = lines.next().context("Expected line")??;
        if line.trim().is_empty() {
            break;
        }

        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        rules
            .entry(tokens[0].to_string())
            .and_modify(|r| r.push(tokens[2].to_string()))
            .or_insert_with(|| vec![tokens[2].to_string()]);
    }
    let molecule = lines.next().context("No starting molecule")??;

    let mut molecules = HashSet::new();
    for (key, values) in rules.iter() {
        for (start, _key) in molecule.match_indices(key) {
            for val in values {
                let mut molecule = molecule.clone();
                molecule.replace_range(start..(start + key.len()), val);
                molecules.insert(molecule);
            }
        }
    }
    println!("Part 1: {}", molecules.len());

    // I solved this by hand for my input using the analysis provided here plus a regex tester
    // https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    println!("Part 2: {}", 195);

    Ok(())
}
