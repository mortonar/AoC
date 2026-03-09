use anyhow::{Error, Result, bail};
use std::io::stdin;

fn main() -> Result<()> {
    let box_ids = parse_input()?;

    println!("Part 1: {}", checksum(&box_ids));

    println!("Part 2: {}", common_correct_chars(&box_ids)?);

    Ok(())
}

fn parse_input() -> Result<Vec<String>> {
    stdin().lines().map(|l| l.map_err(Error::from)).collect()
}

fn checksum(box_ids: &[String]) -> usize {
    let (mut two, mut three) = (0, 0);
    for id in box_ids.iter() {
        let frequencies = frequencies(id);
        if frequencies.contains(&2) {
            two += 1;
        }
        if frequencies.contains(&3) {
            three += 1;
        }
    }
    two * three
}

fn frequencies(id: &str) -> [usize; 26] {
    let mut freq = [0; 26];
    id.trim()
        .chars()
        .for_each(|c| freq[c as usize - 'a' as usize] += 1);
    freq
}

fn common_correct_chars(box_ids: &[String]) -> Result<String> {
    for i in 0..box_ids.len() - 1 {
        for j in (i + 1)..box_ids.len() {
            if let Some(common) = matches(&box_ids[i], &box_ids[j]) {
                return Ok(common);
            }
        }
    }
    bail!("Correct box IDs not found")
}

/// If the box IDs differ by exactly 1 character, return an ID with the common chars
fn matches(id1: &str, id2: &str) -> Option<String> {
    let common: String = id1
        .trim()
        .chars()
        .zip(id2.trim().chars())
        .filter(|(c1, c2)| *c1 == *c2)
        .map(|(c1, _c2)| c1)
        .collect();
    if common.chars().count() == id1.len() - 1 {
        Some(common)
    } else {
        None
    }
}
