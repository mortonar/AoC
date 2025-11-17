use anyhow::{Result, anyhow};
use std::io::BufRead;
use std::{env, io};

fn main() -> Result<()> {
    let mut row: Vec<_> = io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No row provided"))??
        .trim()
        .chars()
        .map(|c| c == '.')
        .collect();

    let count = env::args()
        .nth(1)
        .unwrap_or("40".to_string())
        .parse::<usize>()?;

    let mut safe_count = row.safe_count();
    let right_bound = row.len() - 1;
    for _ in 0..count - 1 {
        let mut left = true;
        for i in 0..row.len() {
            let right = if i < right_bound { row[i + 1] } else { true };
            let center = row[i];
            #[allow(clippy::nonminimal_bool)]
            let is_trap = (!left && !center && right)
                || (!center && !right && left)
                || (center && right && !left)
                || (center && left && !right);
            left = row[i];
            row[i] = !is_trap;
        }
        safe_count += row.safe_count();
    }

    println!("Safe count: {safe_count}");

    Ok(())
}

trait SafeCount {
    fn safe_count(&self) -> usize;
}

impl SafeCount for Vec<bool> {
    fn safe_count(&self) -> usize {
        self.iter().filter(|&&tile| tile).count()
    }
}
