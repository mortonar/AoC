use anyhow::{Result, anyhow};
use std::io::stdin;

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for range in line.trim().split(',') {
        let (start, end) = parse_range(range)?;
        (start..=end).for_each(|id| {
            if repeat_twice(id) {
                sum_p1 += id;
            }
            if repeat_n_times(id) {
                sum_p2 += id;
            }
        });
    }
    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);

    Ok(())
}

fn parse_range(s: &str) -> Result<(usize, usize)> {
    let tokens: Vec<_> = s.split('-').collect();
    let error = || anyhow!("Range must include start and end: {}", s);
    let start = tokens.first().ok_or_else(error)?.parse()?;
    let end = tokens.get(1).ok_or_else(error)?.parse()?;
    Ok((start, end))
}

// The ID is ONLY composed of the same sequence twice. So split the thing in half and check it.
fn repeat_twice(id: usize) -> bool {
    let id = id.to_string();
    let mid = id.len() / 2;
    id[0..mid].eq(&id[mid..])
}

// Now the sequence can be of any length but repeated enough times to compose the ID.
// Check all possible sequences from 0 to half the ID's length (it can't be any larger).
fn repeat_n_times(id: usize) -> bool {
    let id: Vec<_> = id.to_string().chars().collect();
    let mid = id.len() / 2;
    (1..=mid).any(|chunk_size| {
        let seq = &id[0..chunk_size];
        id.chunks(chunk_size).skip(1).all(|chunk| chunk == seq)
    })
}
