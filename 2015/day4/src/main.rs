use anyhow::{Context, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;
    let input = line.trim();

    // In the samples given, we appear to need at least n+1 digits where n is # of 0s in prefix.
    // This is a bit slow for part 2...
    let num = 100000;
    let p1 = find_secret(&input, num, 5).context("Part 1 not found")?;
    println!("Part 1: {p1}");
    let p2 = find_secret(&input, num << 1, 6).context("Part 2 not found")?;
    println!("Part 2: {p2}");

    Ok(())
}

fn find_secret(input: &str, start: usize, prefix_size: usize) -> Option<usize> {
    let prefix = "0".repeat(prefix_size);
    (start..).into_iter().find(|i| {
        let key = format!("{input}{i}");
        let digest = md5::compute(key.as_bytes());
        let hex = format!("{:x}", &digest);
        hex.starts_with(&prefix)
    })
}
