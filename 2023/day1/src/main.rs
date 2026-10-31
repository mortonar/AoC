use std::io::{BufRead, stdin};

use anyhow::Result;

fn main() -> Result<()> {
    let lines = parse_input()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn parse_input() -> Result<Vec<String>> {
    stdin().lock().lines().map(|l| Ok(l?)).collect()
}

fn part1(lines: &[String]) -> usize {
    lines.iter().map(|l| l.calibrate(false)).sum()
}

fn part2(lines: &[String]) -> usize {
    lines.iter().map(|l| l.calibrate(true)).sum()
}

trait CalibrationValue {
    fn calibrate(&self, words: bool) -> usize;
}

impl CalibrationValue for String {
    fn calibrate(&self, words: bool) -> usize {
        let mut digits = (0..self.len()).filter_map(|i| digit_at(&self[i..], words));
        let first = digits.next().unwrap_or(0);
        let last = digits.next_back().unwrap_or(first);
        first * 10 + last
    }
}

fn digit_at(s: &str, words: bool) -> Option<usize> {
    if let Some(d) = s.chars().next()?.to_digit(10) {
        return Some(d as usize);
    }
    if words {
        return WORDS.iter().position(|w| s.starts_with(w));
    }
    None
}

const WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
