use anyhow::{Error, Result};
use std::io::stdin;

fn main() -> Result<()> {
    let rows = parse_rows()?;
    let mut part1 = 0;
    let mut part2 = 0;
    rows.iter().for_each(|row| {
        part1 += diff_checksum(row);
        part2 += div_checksum(row);
    });
    println!("Part 1 {part1}");
    println!("Part 2 {part2}");

    Ok(())
}

fn diff_checksum(row: &[usize]) -> usize {
    let (min, max) = row
        .iter()
        .fold((usize::MAX, usize::MIN), |(min, max), &val| {
            (min.min(val), max.max(val))
        });
    min.abs_diff(max)
}

fn div_checksum(row: &[usize]) -> usize {
    for (i, &val1) in row.iter().enumerate() {
        for &val2 in row.iter().skip(i + 1) {
            let (small, large) = (val1.min(val2), val1.max(val2));
            if large % small == 0 {
                return large / small;
            }
        }
    }
    0
}

fn parse_rows() -> Result<Vec<Vec<usize>>> {
    stdin()
        .lines()
        .map(|line| {
            line?
                .split_ascii_whitespace()
                .map(|val| val.parse::<usize>().map_err(Error::from))
                .collect()
        })
        .collect()
}
