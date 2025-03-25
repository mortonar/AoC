use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut lines = std::io::stdin().lock().lines();

    let mut buffer: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim().is_empty() {
            match Type::from_grid(&buffer) {
                Type::Lock(lock) => locks.push(lock),
                Type::Key(key) => keys.push(key),
            }
            buffer.clear();
            continue;
        }
        buffer.push(line.chars().collect())
    }

    let mut fits = 0;
    for lock in &locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(lh, kh)| lh + kh <= lock.len()) {
                fits += 1;
            }
        }
    }
    println!("{fits}");

    Ok(())
}

enum Type {
    Lock(Vec<usize>),
    Key(Vec<usize>),
}

impl Type {
    fn from_grid(grid: &Vec<Vec<char>>) -> Self {
        let mut heights = vec![0; grid[0].len()];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '#' {
                    heights[j] += 1;
                }
            }
        }
        heights.iter_mut().for_each(|h| *h -= 1);
        if grid[0].iter().all(|&c| c == '#') {
            Type::Lock(heights)
        } else {
            Type::Key(heights)
        }
    }
}