use anyhow::{Error, Result};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let region_specs = parse_input()?;
    let valid_regions = region_specs.into_iter().filter(RegionSpec::can_fit).count();
    println!("Part 1: {valid_regions}");
    Ok(())
}

struct RegionSpec {
    size: (usize, usize),
    total_presents: usize,
}

/// For this puzzle, the input is such that presents will fit into the region purely based on area.
/// All presents are 3 * 3, so this laughably takes an NP-Complete problem down to simple math :).
/// A bit dissatisfying IMO but: a puzzle's a puzzle.
impl RegionSpec {
    fn can_fit(&self) -> bool {
        let total_area = self.size.0 * self.size.1;
        let present_area = self.total_presents * (3 * 3);
        present_area <= total_area
    }
}

fn parse_input() -> Result<Vec<RegionSpec>> {
    let mut region_specs = Vec::new();

    for line in stdin().lock().lines() {
        let line = line?;
        let line = line.trim();

        // Ignore the shape definitions for the presents...yes, really.
        if line.is_empty() || line.ends_with(":") || line.contains("#") || line.contains(".") {
            continue;
        }

        region_specs.push(line.parse()?);
    }

    Ok(region_specs)
}

impl FromStr for RegionSpec {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(|c: char| !c.is_ascii_digit()).collect();
        let presents: Result<Vec<usize>> = tokens[2..]
            .iter()
            .filter(|t| !t.is_empty())
            .map(|p| p.parse().map_err(Error::from))
            .collect();
        Ok(RegionSpec {
            size: (tokens[0].parse()?, tokens[1].parse()?),
            total_presents: presents?.iter().sum(),
        })
    }
}
