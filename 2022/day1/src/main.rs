use anyhow::Result;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let elves = parse_input()?;

    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));

    Ok(())
}

fn parse_input() -> Result<Vec<Elf>> {
    let mut elves = Vec::new();
    let mut calories = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(Elf { calories });
            calories = Vec::new();
            continue;
        }
        calories.push(line.parse()?);
    }
    elves.push(Elf { calories });
    Ok(elves)
}

struct Elf {
    calories: Vec<usize>,
}

impl Elf {
    fn total(&self) -> usize {
        self.calories.iter().sum()
    }
}

fn part1(food: &[Elf]) -> usize {
    food.iter().map(Elf::total).max().unwrap()
}

fn part2(food: &[Elf]) -> usize {
    let mut totals: Vec<usize> = food.iter().map(Elf::total).collect();
    totals.sort_unstable();
    totals.iter().rev().take(3).sum()
}
