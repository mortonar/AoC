use anyhow::{Result, anyhow};
use std::io::stdin;

fn main() -> Result<()> {
    let mut paper: usize = 0;
    let mut ribbon: usize = 0;
    for line in stdin().lines() {
        let dimensions: Vec<_> = line?
            .split('x')
            .map(|d| d.parse::<usize>().unwrap())
            .collect();
        if let &[l, w, h] = dimensions.as_slice() {
            let sides = vec![vec![l, w], vec![w, h], vec![h, l]];
            let areas: Vec<_> = sides
                .iter()
                .map(|dims| dims.iter().product::<usize>())
                .collect();
            paper += 2 * areas.iter().sum::<usize>() + areas.iter().min().unwrap();

            let perims: Vec<_> = sides
                .iter()
                .map(|dims| 2 * dims.iter().sum::<usize>())
                .collect();
            ribbon += perims.iter().min().unwrap() + l * w * h;
        } else {
            return Err(anyhow!("Need 3 dimensions but given {}", dimensions.len()));
        }
    }
    println!("Part 1: {paper}");
    println!("Part 2: {ribbon}");

    Ok(())
}
