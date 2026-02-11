use anyhow::{Result, anyhow};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let [gen_a, gen_b] = parse_input()?;

    let matches = count_matches(gen_a.clone(), gen_b.clone(), 40_000_000);
    println!("Part 1 {matches}");

    let matches = count_matches(
        gen_a.filter(|v| v % 4 == 0),
        gen_b.filter(|v| v % 8 == 0),
        5_000_000,
    );
    println!("Part 2 {matches}");

    Ok(())
}

fn count_matches(
    a: impl Iterator<Item = usize>,
    b: impl Iterator<Item = usize>,
    take: usize,
) -> usize {
    const MASK: usize = 0xFFFF;
    a.zip(b)
        .take(take)
        .filter(|&(a, b)| a & MASK == b & MASK)
        .count()
}

fn parse_input() -> Result<[Generator; 2]> {
    fn parse_line(line: io::Result<String>) -> Result<usize> {
        Ok(line?
            .split_ascii_whitespace()
            .last()
            .ok_or_else(|| anyhow!("No starting value provided"))?
            .parse()?)
    }

    let mut lines = io::stdin().lock().lines();
    let start_a = parse_line(lines.next().ok_or_else(|| anyhow!("Missing first line"))?)?;
    let start_b = parse_line(lines.next().ok_or_else(|| anyhow!("Missing second line"))?)?;

    Ok([
        Generator::new(start_a, 16807),
        Generator::new(start_b, 48271),
    ])
}

#[derive(Clone, Debug)]
struct Generator {
    value: usize,
    factor: usize,
}

impl Generator {
    fn new(value: usize, factor: usize) -> Self {
        Self { value, factor }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % 2147483647;
        Some(self.value)
    }
}
