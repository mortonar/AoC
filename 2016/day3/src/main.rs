use anyhow::{Context, Result, anyhow};
use itertools::Itertools;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut valid_horiz = TriangleCounter::default();

    let mut valid_vert = TriangleCounter::default();

    for chunk in &io::stdin().lock().lines().chunks(3) {
        let mut rows = [[0; 3]; 3];
        for (i, line) in chunk.enumerate() {
            let sides: [usize; 3] = line?
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().context("Failed to parse side"))
                .collect::<Result<Vec<_>>>()?
                .try_into()
                .map_err(|_| anyhow!("Failed to parse 3 sides"))?;

            valid_horiz.count(&sides);

            rows[i] = sides;
        }

        for col in 0..3 {
            let column = [rows[0][col], rows[1][col], rows[2][col]];
            valid_vert.count(&column);
        }
    }

    println!("Part 1: {}", valid_horiz.count);
    println!("Part 2: {}", valid_vert.count);

    Ok(())
}

#[derive(Default)]
struct TriangleCounter {
    count: usize,
}

impl TriangleCounter {
    fn count(&mut self, [a, b, c]: &[usize; 3]) {
        if a + b > *c && a + c > *b && b + c > *a {
            self.count += 1;
        }
    }
}
