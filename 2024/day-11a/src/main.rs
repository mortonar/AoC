use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut stones = String::new();
    stdin().read_line(&mut stones)?;
    let mut stones: Vec<u64> = stones
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse())
        .flatten()
        .collect();

    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
            } else if stone.to_string().len() % 2 == 0 {
                let stone = stone.to_string();
                stones[i] = stone[..stone.len() / 2].parse()?;
                i += 1;
                stones.insert(i, stone[(stone.len() / 2)..].parse()?);
            } else {
                stones[i] = stone * 2024;
            }
            i += 1;
        }
    }

    println!("{}", stones.len());

    Ok(())
}
