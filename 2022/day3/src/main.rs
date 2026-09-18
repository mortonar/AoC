use std::{
    collections::HashSet,
    io::{BufRead, stdin},
};

use anyhow::Result;

fn main() -> Result<()> {
    let ruck_sacks = parse_input()?;

    println!("Part 1: {}", part1(&ruck_sacks));
    println!("Part 2: {}", part2(&ruck_sacks));

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<char>>> {
    stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.chars().collect()))
        .collect()
}

fn part1(ruck_sacks: &[Vec<char>]) -> usize {
    ruck_sacks
        .iter()
        .map(|rs| {
            // Treat different compartments as different rucksacks
            let half = rs.len() / 2;
            let h1 = &rs[0..half];
            let h2 = &rs[half..];
            priority(&[h1, h2])
        })
        .sum()
}

fn part2(ruck_sacks: &[Vec<char>]) -> usize {
    ruck_sacks
        .to_vec()
        .chunks(3)
        .map(|group| {
            priority(&[
                group[0].as_slice(),
                group[1].as_slice(),
                group[2].as_slice(),
            ])
        })
        .sum()
}

fn priority(sacks: &[&[char]]) -> usize {
    let common = sacks
        .iter()
        .map(|s| s.iter().cloned().collect::<HashSet<_>>())
        .reduce(|common, set| common.intersection(&set).cloned().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    if common.is_ascii_lowercase() {
        common as usize - 'a' as usize + 1
    } else {
        common as usize - 'A' as usize + 27
    }
}
