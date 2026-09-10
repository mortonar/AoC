use anyhow::{Error, Result, anyhow};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let instructions = parse_input()?;

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));

    Ok(())
}

fn parse_input() -> Result<Vec<Instruction>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(instructions: &[Instruction]) -> isize {
    // An instruction can only affect cells inside its own cuboid, so clipping each
    // one to the region leaves the count within that region unchanged.
    let clipped: Vec<_> = instructions
        .iter()
        .filter_map(|i| {
            let cuboid = REGION.intersect(&i.cuboid)?;
            Some(Instruction { on: i.on, cuboid })
        })
        .collect();

    count(&clipped)
}

const REGION: Cuboid = Cuboid {
    x: (-50, 50),
    y: (-50, 50),
    z: (-50, 50),
};

fn part2(instructions: &[Instruction]) -> isize {
    count(instructions)
}

/// Counts lit cells as a list of signed cuboids whose volumes sum to the true total.
///
/// Before a new cuboid is added, its overlap with every existing term is pushed back
/// with the opposite sign, cancelling whatever that region already contributed. So an
/// `on` cuboid adds only the cells that weren't already lit, and an `off` cuboid just
/// cancels and adds nothing of its own.
fn count(instructions: &[Instruction]) -> isize {
    let mut terms: Vec<(Cuboid, isize)> = Vec::new();

    for Instruction { on, cuboid } in instructions {
        let corrections: Vec<_> = terms
            .iter()
            .filter_map(|(term, sign)| Some((term.intersect(cuboid)?, -sign)))
            .collect();

        terms.extend(corrections);

        if *on {
            terms.push((*cuboid, 1));
        }
    }

    terms
        .iter()
        .map(|(cuboid, sign)| cuboid.volume() * sign)
        .sum()
}

#[derive(Debug)]
struct Instruction {
    on: bool,
    cuboid: Cuboid,
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    /// Bounds are inclusive, hence the +1 per axis.
    fn volume(&self) -> isize {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let overlap = |a: (isize, isize), b: (isize, isize)| {
            let range = (a.0.max(b.0), a.1.min(b.1));
            (range.0 <= range.1).then_some(range)
        };

        Some(Cuboid {
            x: overlap(self.x, other.x)?,
            y: overlap(self.y, other.y)?,
            z: overlap(self.z, other.z)?,
        })
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (state, axes) = s
            .trim()
            .split_once(' ')
            .ok_or_else(|| anyhow!("Malformed instruction: {s}"))?;

        let on = match state {
            "on" => true,
            "off" => false,
            _ => return Err(anyhow!("Unknown state: {state}")),
        };

        let ranges = axes
            .split(',')
            .map(parse_range)
            .collect::<Result<Vec<_>>>()?;

        match ranges[..] {
            [x, y, z] => Ok(Instruction {
                on,
                cuboid: Cuboid { x, y, z },
            }),
            _ => Err(anyhow!("Expected 3 axis ranges, got {}", ranges.len())),
        }
    }
}

fn parse_range(axis: &str) -> Result<(isize, isize)> {
    let (_, range) = axis
        .split_once('=')
        .ok_or_else(|| anyhow!("Malformed axis: {axis}"))?;
    let (low, high) = range
        .split_once("..")
        .ok_or_else(|| anyhow!("Malformed range: {range}"))?;

    Ok((low.parse()?, high.parse()?))
}
