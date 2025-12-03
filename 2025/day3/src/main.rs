use anyhow::Result;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let banks = parse_banks()?;
    println!("Part 1: {}", banks.max_joltage(2));
    println!("Part 2: {}", banks.max_joltage(12));
    Ok(())
}

fn parse_banks() -> Result<Vec<Vec<u8>>> {
    stdin()
        .lock()
        .lines()
        .map(|line| {
            line?
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or_else(|| anyhow::anyhow!("Invalid digit: {}", c))
                })
                .collect()
        })
        .collect()
}

trait MaxJoltage {
    fn max_joltage(&self, to_select: usize) -> usize;
}

impl MaxJoltage for Vec<Vec<u8>> {
    fn max_joltage(&self, to_select: usize) -> usize {
        self.iter()
            .map(|bank| max_joltage_rec(bank, to_select, String::new()))
            .sum::<usize>()
    }
}

// A greedy recursive algorithm for maximizing the "joltage".
fn max_joltage_rec(bank: &[u8], to_select: usize, mut digits: String) -> usize {
    if to_select == 0 {
        return digits.parse().unwrap();
    }

    // Pick the max from the left of the bank while leaving enough room to select remaining batteries
    let (i, selected) = bank[0..=(bank.len() - to_select)]
        .iter()
        .enumerate()
        .max_by(|b1, b2| b1.1.cmp(b2.1).then_with(|| b2.0.cmp(&b1.0)))
        .unwrap();
    digits.push(char::from_digit(*selected as u32, 10).unwrap());
    max_joltage_rec(&bank[i + 1..], to_select - 1, digits)
}
