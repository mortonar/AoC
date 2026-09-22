use std::io::{BufRead, stdin};

use anyhow::Result;

fn main() -> Result<()> {
    let buffer = parse_input()?;

    println!("Part 1: {}", part1(&buffer));
    println!("Part 2: {}", part2(&buffer));

    Ok(())
}

fn parse_input() -> Result<Vec<char>> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;
    Ok(line.trim().chars().collect())
}

fn part1(buffer: &[char]) -> usize {
    distinct(buffer, 4)
}

fn part2(buffer: &[char]) -> usize {
    distinct(buffer, 14)
}

fn distinct(buffer: &[char], n: usize) -> usize {
    let (start, _seq) = buffer
        .windows(n)
        .enumerate()
        .find(|(_, seq)| seq.diff())
        .unwrap();
    start + n
}

trait Different {
    fn diff(&self) -> bool;
}

impl Different for &[char] {
    fn diff(&self) -> bool {
        let mut used = [false; 26];
        let offset = 'a' as usize;
        for &c in self.iter() {
            let idx = c as usize - offset;
            if used[idx] {
                return false;
            } else {
                used[idx] = true;
            }
        }
        true
    }
}
