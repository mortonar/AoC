use anyhow::{Error, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let offsets = parse_offsets()?;
    println!("Part 1: {}", find_exit(&offsets, false));
    println!("Part 2: {}", find_exit(&offsets, true));
    Ok(())
}

fn find_exit(offsets: &[isize], part2: bool) -> usize {
    let mut offsets = offsets.to_vec();
    let len = offsets.len() as isize;
    let mut steps = 0;
    let mut idx: isize = 0;
    while idx >= 0 && idx < len {
        let offset = &mut offsets[idx as usize];
        idx += *offset;
        if part2 && *offset >= 3 {
            *offset -= 1;
        } else {
            *offset += 1;
        }
        steps += 1;
    }
    steps
}

fn parse_offsets() -> Result<Vec<isize>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l?.parse().map_err(Error::from))
        .collect()
}
