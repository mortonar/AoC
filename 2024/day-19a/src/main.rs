use anyhow::{Context, Result};
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

    let possible = designs.iter().filter(|d| is_possible(d, &towels)).count();
    println!("{}", possible);

    Ok(())
}

fn is_possible(design: &[char], avail_towels: &Vec<Vec<char>>) -> bool {
    if design.is_empty() {
        return true;
    }

    for to_try in avail_towels.iter() {
        let len = to_try.len();
        let des_len = design.len();
        if des_len >= len && design[0..len].eq(&to_try[..]) {
            if is_possible(&design[len..], avail_towels) {
                return true;
            }
        }
    }

    false
}
