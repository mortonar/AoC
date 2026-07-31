use anyhow::{Error, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let sonar_report = parse_input()?;

    println!("Part 1: {}", part1(&sonar_report));
    println!("Part 2: {}", part2(&sonar_report));

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l?.parse().map_err(Error::from))
        .collect()
}

fn part1(sonar_report: &[usize]) -> usize {
    sonar_report.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(sonar_report: &[usize]) -> usize {
    let windows: Vec<_> = sonar_report
        .windows(3)
        .map(|w| w.iter().sum::<usize>())
        .collect();
    part1(&windows)
}
