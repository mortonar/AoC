use anyhow::{Context, Result};
use std::env::args;

fn main() -> Result<()> {
    let step = parse_input()?;
    let mut pos = 0;
    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0);
    for i in 1u32..=2017 {
        pos = (pos + step) % buffer.len() + 1;
        buffer.insert(pos, i);
    }
    let after = (pos + 1) % buffer.len();
    println!("Part 1: {}", buffer[after]);

    // At this point, we only need to track what's inserted into the second position (index 1)
    // rather than the whole ~1.5GB of buffer contents for 50 million numbers.
    let mut second = buffer[1] as usize;
    let mut buffer_size = buffer.len();
    for i in 2018..=50_000_000 {
        pos = (pos + step) % buffer_size + 1;
        if pos == 1 {
            second = i;
        }
        buffer_size += 1;
    }
    println!("Part 2: {}", second);

    Ok(())
}

fn parse_input() -> Result<usize> {
    args()
        .nth(1)
        .unwrap_or_else(|| "3".to_string())
        .parse()
        .context("Failed to parse step")
}
