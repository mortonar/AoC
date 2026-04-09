use anyhow::{Error, Result};
use std::io::stdin;
use std::iter::successors;

fn main() -> Result<()> {
    let module_masses = parse_input()?;

    let sum: usize = module_masses.iter().map(|mass| mass / 3 - 2).sum();
    println!("Part 1: {sum}");

    let sum: usize = module_masses
        .iter()
        .map(|mass| {
            successors(Some(*mass), |m| (m / 3).checked_sub(2))
                // Skip successors first (*mass)
                .skip(1)
                .sum::<usize>()
        })
        .sum();
    println!("Part 2: {sum}");

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    stdin()
        .lines()
        .map(|l| l?.parse().map_err(Error::from))
        .collect()
}
