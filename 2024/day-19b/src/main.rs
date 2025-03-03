use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut lines = io::stdin().lock().lines();

    let towel_line = lines.next().context("No available towels given")??;
    let towels: Vec<Vec<char>> = towel_line
        .split(", ")
        .map(|t| t.chars().collect())
        .collect();

    let _blank = lines.next().context("Expected blank line")??;

    let mut designs: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        designs.push(line?.chars().collect());
    }

    // Save time by storing computed possibilities for given arrangements
    let mut memo: HashMap<Vec<char>, usize> = HashMap::new();
    let possible: usize = designs
        .iter()
        .map(|d| possible_arrange(d, &towels, &mut memo))
        .sum();
    println!("{}", possible);

    Ok(())
}

fn possible_arrange(
    design: &[char],
    avail_towels: &Vec<Vec<char>>,
    memo: &mut HashMap<Vec<char>, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(cached) = memo.get(&design.to_vec()) {
        return *cached;
    }

    let mut possible = 0;
    for to_try in avail_towels.iter() {
        let towel_len = to_try.len();
        let des_len = design.len();
        if des_len >= towel_len && design[0..towel_len].eq(&to_try[..]) {
            possible += possible_arrange(&design[towel_len..], avail_towels, memo);
        }
    }
    memo.insert(design.to_vec(), possible);

    possible
}
