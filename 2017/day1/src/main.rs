use anyhow::{Result, anyhow};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let digits = parse_digits()?;
    let len = digits.len();

    let mut part1 = 0;
    let mut part2 = 0;
    digits.iter().enumerate().for_each(|(i, digit)| {
        if *digit == digits[(i + 1) % len] {
            part1 += digit;
        }

        if *digit == digits[(i + len / 2) % len] {
            part2 += digit;
        }
    });
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

fn parse_digits() -> Result<Vec<u32>> {
    stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No digit line provided"))??
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| anyhow!("All digits must be [0-9]"))
        })
        .collect()
}
