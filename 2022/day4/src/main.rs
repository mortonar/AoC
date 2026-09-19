use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let pairs = parse_input()?;

    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));

    Ok(())
}

fn parse_input() -> Result<Vec<Pair>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|p| p.encapsulated()).count()
}

fn part2(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|p| p.overlap()).count()
}

#[derive(Debug)]
struct Pair {
    r1: Range,
    r2: Range,
}

#[derive(Debug)]
struct Range {
    left: usize,
    right: usize,
}

impl Pair {
    fn encapsulated(&self) -> bool {
        self.r1.contains(&self.r2) || self.r2.contains(&self.r1)
    }

    fn overlap(&self) -> bool {
        self.r1.overlap(&self.r2) || self.r2.overlap(&self.r1)
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        other.left >= self.left && other.right <= self.right
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.right >= other.left && self.right <= other.right)
            || (self.left <= other.right && self.left >= other.left)
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let Some((r1, r2)) = s.split_once(',') else {
            bail!("missing pair delimiter");
        };
        let (r1, r2) = (r1.parse()?, r2.parse()?);
        Ok(Pair { r1, r2 })
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let Some((left, right)) = s.split_once('-') else {
            bail!("missing range delimiter");
        };
        let (left, right) = (left.parse()?, right.parse()?);
        Ok(Range { left, right })
    }
}
