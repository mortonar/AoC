use anyhow::{Error, Result, bail};
use std::env::args;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let numbers = parse_input()?;
    let preamble = args().nth(1).unwrap_or("25".to_owned()).parse()?;

    let part1 = part1(&numbers, preamble)?;
    println!("Part 1: {part1}");
    let part2 = part2(&numbers, part1)?;
    println!("Part 2: {part2}");

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l?.parse().map_err(Error::from))
        .collect()
}

fn part1(numbers: &[usize], preamble: usize) -> Result<usize> {
    for (i, &n) in numbers.iter().enumerate().skip(preamble) {
        if !two_sum(&numbers[i - preamble..i], n) {
            return Ok(n);
        }
    }
    bail!("Part 1 not found")
}

fn two_sum(numbers: &[usize], target: usize) -> bool {
    for i in 0..numbers.len() - 1 {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }
    false
}

fn part2(numbers: &[usize], target: usize) -> Result<usize> {
    for w_size in 2..numbers.len() {
        for window in numbers.windows(w_size) {
            if window.iter().sum::<usize>() == target {
                return Ok(window.iter().min().unwrap() + window.iter().max().unwrap());
            }
        }
    }
    bail!("Part 2 not found")
}
