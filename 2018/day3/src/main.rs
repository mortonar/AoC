use anyhow::{Error, Result, bail};
use std::collections::HashSet;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let claims = parse_input()?;

    let claiming_ids = get_claiming_ids(&claims);

    let mut overlapping = 0;
    let mut all_ids: HashSet<_> = claims.iter().map(|c| c.id).collect();
    for claiming_ids in claiming_ids.iter().flat_map(|row| row.iter()) {
        if claiming_ids.len() > 1 {
            overlapping += 1;
            claiming_ids.iter().for_each(|id| {
                all_ids.remove(id);
            });
        }
    }
    println!("Part 1: {overlapping}");
    if let Some(sep) = all_ids.iter().next() {
        println!("Part 2: {sep}");
    } else {
        bail!("There should be one non-overlapping claim");
    }

    Ok(())
}

fn parse_input() -> Result<Vec<Claim>> {
    stdin().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug)]
struct Claim {
    id: usize,
    coords: (usize, usize),
    dimensions: (usize, usize),
}

impl FromStr for Claim {
    type Err = Error;

    // Swap row/col for coords and dimensions to make 2-D array indexing easier
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();

        let id: usize = tokens[0][1..].parse()?;

        let row_col: Vec<_> = tokens[2].split(&[',', ':']).collect();
        let coords = (row_col[1].parse()?, row_col[0].parse()?);

        let dims: Vec<_> = tokens[3].split("x").collect();
        let dimensions = (dims[1].parse()?, dims[0].parse()?);

        Ok(Self {
            id,
            coords,
            dimensions,
        })
    }
}

const SQUARE_SIZE: usize = 1_000;

/// Record which claims are on each square inch
fn get_claiming_ids(claims: &[Claim]) -> Vec<Vec<Vec<usize>>> {
    let mut counts = vec![vec![vec![]; SQUARE_SIZE]; SQUARE_SIZE];
    for claim in claims.iter() {
        for i in 0..claim.dimensions.0 {
            for j in 0..claim.dimensions.1 {
                counts[i + claim.coords.0][j + claim.coords.1].push(claim.id)
            }
        }
    }
    counts
}
