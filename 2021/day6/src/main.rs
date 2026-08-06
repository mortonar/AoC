use anyhow::{Error, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let fish = parse_input()?;

    println!("Part 1: {}", part1(&fish));
    println!("Part 2: {}", part2(&fish));

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

fn part1(fish: &[usize]) -> usize {
    simulate(fish, 80)
}

fn part2(fish: &[usize]) -> usize {
    simulate(fish, 256)
}

fn simulate(fish: &[usize], days: usize) -> usize {
    let mut fish_counts = [0; 9];
    fish.iter().for_each(|&f| fish_counts[f] += 1);

    (0..days).for_each(|_| {
        let mut new_counts = [0; 9];
        fish_counts.iter().enumerate().for_each(|(t, &count)| {
            if t == 0 {
                new_counts[6] += count;
                new_counts[8] += count;
            } else {
                new_counts[t - 1] += count;
            }
        });
        fish_counts = new_counts;
    });

    fish_counts.iter().sum()
}
