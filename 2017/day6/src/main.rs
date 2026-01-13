use anyhow::{Error, Result, anyhow};
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let banks = parse_memory_banks()?;
    let (cycles, loop_size) = redistribute(banks);
    println!("Part 1: {cycles}");
    println!("Part 2: {loop_size}");
    Ok(())
}

fn redistribute(mut banks: Vec<usize>) -> (usize, usize) {
    let mut seen = HashMap::new();
    for step in 0usize.. {
        let (mut idx, blocks) = max_bank(&banks);
        banks[idx] = 0;
        idx = (idx + 1) % banks.len();
        for _ in 0..blocks {
            banks[idx] += 1;
            idx = (idx + 1) % banks.len();
        }

        if let Some(&s) = seen.get(&banks) {
            return (step + 1, step - s);
        }
        seen.insert(banks.clone(), step);
    }
    unreachable!("Cycle not detected");
}

fn max_bank(banks: &[usize]) -> (usize, usize) {
    let mut max = (0, banks[0]);
    for (i, &blocks) in banks.iter().enumerate().skip(1) {
        if blocks > max.1 {
            max = (i, blocks);
        }
    }
    max
}

fn parse_memory_banks() -> Result<Vec<usize>> {
    let line = stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No memory banks given"))??;
    line.split_ascii_whitespace()
        .map(|bank| bank.parse().map_err(Error::from))
        .collect()
}
