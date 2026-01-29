use anyhow::{Result, anyhow};
use std::io::{BufRead, stdin};
use std::ops::Add;
use std::str::FromStr;

// https://www.redblobgames.com/grids/hexagons/#distances
fn main() -> Result<()> {
    let path = parse_path()?;
    let (end, max) = path
        .iter()
        .fold((Coords::default(), 0), |(pos, max), &dir| {
            let new_pos = pos + dir;
            let new_max = max.max(new_pos.distance());
            (new_pos, new_max)
        });
    println!("Part 1: {}", end.distance());
    println!("Part 2: {max}");
    Ok(())
}

fn parse_path() -> Result<Vec<Coords>> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;
    line.trim().split(',').map(str::parse).collect()
}

#[derive(Debug, Default, Copy, Clone)]
struct Coords {
    q: isize,
    r: isize,
    s: isize,
}

impl Coords {
    const fn new(q: isize, r: isize, s: isize) -> Self {
        Self { q, r, s }
    }

    fn distance(&self) -> usize {
        ((self.q.abs() + self.r.abs() + self.s.abs()) / 2) as usize
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
            s: self.s + other.s,
        }
    }
}

impl FromStr for Coords {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "n" => Ok(Coords::new(0, -1, 1)),
            "ne" => Ok(Coords::new(1, -1, 0)),
            "se" => Ok(Coords::new(1, 0, -1)),
            "s" => Ok(Coords::new(0, 1, -1)),
            "sw" => Ok(Coords::new(-1, 1, 0)),
            "nw" => Ok(Coords::new(-1, 0, 1)),
            _ => Err(anyhow!("Unrecognized direction: {s}")),
        }
    }
}
