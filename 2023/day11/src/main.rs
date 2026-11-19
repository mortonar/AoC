use std::{
    collections::HashSet,
    io::{BufRead, stdin},
};

use anyhow::Result;

fn main() -> Result<()> {
    let galaxies = parse_input()?;

    println!("Part 1: {}", part1(&galaxies));
    println!("Part 2: {}", part2(&galaxies));

    Ok(())
}

fn parse_input() -> Result<Vec<(usize, usize)>> {
    let mut galaxies = Vec::new();
    for (x, line) in stdin().lock().lines().enumerate() {
        for (y, c) in line?.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
            }
        }
    }
    Ok(galaxies)
}

fn part1(galaxies: &[(usize, usize)]) -> usize {
    shortest_path_sum(&expand(galaxies, 1))
}

fn part2(galaxies: &[(usize, usize)]) -> usize {
    shortest_path_sum(&expand(galaxies, 999_999))
}

fn expand(galaxies: &[(usize, usize)], factor: usize) -> Vec<(usize, usize)> {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();
    let mut row_max = 0;
    let mut col_max = 0;
    for &(x, y) in galaxies.iter() {
        rows.insert(x);
        cols.insert(y);
        row_max = row_max.max(x);
        col_max = col_max.max(y);
    }

    let expand_rows: Vec<_> = (0..=row_max).filter(|r| !rows.contains(r)).collect();
    let expand_cols: Vec<_> = (0..=col_max).filter(|r| !cols.contains(r)).collect();

    let mut expanded_galaxies = galaxies.to_vec();
    for (x, y) in expanded_galaxies.iter_mut() {
        for (expanded, &r) in expand_rows.iter().enumerate() {
            if *x > r + expanded * factor {
                *x += factor;
            }
        }
        for (expanded, &c) in expand_cols.iter().enumerate() {
            if *y > c + expanded * factor {
                *y += factor;
            }
        }
    }
    expanded_galaxies
}

fn shortest_path_sum(galaxies: &[(usize, usize)]) -> usize {
    let mut path_sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in (i + 1)..galaxies.len() {
            let &(x1, y1) = &galaxies[i];
            let &(x2, y2) = &galaxies[j];
            path_sum += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }
    path_sum
}
