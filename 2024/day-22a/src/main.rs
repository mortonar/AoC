use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut secrets: Vec<u64> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        secrets.push(line?.trim().parse()?);
    }

    for _ in 0..2000 {
        for s in &mut secrets {
            *s = mix_prune(*s, *s * 64);
            *s = mix_prune(*s, *s / 32);
            *s = mix_prune(*s, *s * 2048)
        }
    }

    let sum = secrets.iter().sum::<u64>();
    println!("{sum}");

    Ok(())
}

fn mix_prune(secret: u64, mix_with: u64) -> u64 {
    (secret ^ mix_with) % 16777216
}