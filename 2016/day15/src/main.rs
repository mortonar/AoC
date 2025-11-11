use anyhow::{Error, Result};
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut discs = io::stdin()
        .lock()
        .lines()
        .map(|l| Disc::from_str(&l?))
        .collect::<Result<Vec<_>>>()?;

    println!("Part 1: {}", find_time(&mut discs));
    discs.iter_mut().for_each(|disc| disc.reset());
    discs.push(Disc::new(11, 0));
    println!("Part 2: {}", find_time(&mut discs));

    Ok(())
}

// Disc 1 should be 1s / rotation away from 0. Disc 2 should be 2. And so on.
// Basically keep rotating until all the discs line up (runs in ~ 1/4 of a second on my machine).
fn find_time(discs: &mut [Disc]) -> usize {
    for t in 0.. {
        if discs.iter().enumerate().all(|(i, d)| d.hit_0_in(i + 1)) {
            return t;
        }

        discs.iter_mut().for_each(|d| d.rotate())
    }

    0
}

#[derive(Debug)]
struct Disc {
    positions: usize,
    position: usize,
    original: usize,
}

impl Disc {
    fn new(positions: usize, position: usize) -> Self {
        Self {
            positions,
            position,
            original: position,
        }
    }

    // If we rotated n times, would we hit 0?
    fn hit_0_in(&self, n: usize) -> bool {
        (self.position + n) % self.positions == 0
    }

    fn rotate(&mut self) {
        self.position = (self.position + 1) % self.positions
    }

    fn reset(&mut self) {
        self.position = self.original;
    }
}

impl FromStr for Disc {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        let positions = tokens[3].parse()?;
        let pos_tokens = tokens[11].split('.').collect::<Vec<_>>();
        let position = pos_tokens[0].parse()?;
        Ok(Self::new(positions, position))
    }
}
