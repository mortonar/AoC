use anyhow::{Error, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let positions = parse_input()?;

    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;
    line.trim()
        .split(',')
        .map(|n| n.parse().map_err(Error::from))
        .collect()
}

fn part1(positions: &[usize]) -> usize {
    min_fuel(positions, |a, p| a.abs_diff(p))
}

fn part2(positions: &[usize]) -> usize {
    min_fuel(positions, |a, p| {
        // 1 + 2 + 3 + ... n
        let n = a.abs_diff(p);
        n * (n + 1) / 2
    })
}

fn min_fuel<F>(positions: &[usize], align_cost: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    let (min, max) = positions
        .iter()
        .fold((usize::MAX, usize::MIN), |(min, max), &p| {
            (min.min(p), max.max(p))
        });

    (min..=max)
        .map(|align_to| positions.iter().map(|&p| align_cost(align_to, p)).sum())
        .min()
        .unwrap()
}
