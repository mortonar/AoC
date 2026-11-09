use std::io::{BufRead, stdin};

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let races = parse_input()?;

    println!("Part 1: {}", part1(&races));
    println!("part 2: {}", part2(&races));

    Ok(())
}

fn parse_input() -> Result<Races> {
    let mut lines = stdin().lock().lines();
    let times = parse_nums(&lines.next().ok_or_else(|| anyhow!("missing times"))??)?;
    let distances = parse_nums(&lines.next().ok_or_else(|| anyhow!("missing times"))??)?;
    Ok(Races { times, distances })
}

fn parse_nums(s: &str) -> Result<Vec<usize>> {
    s.split_ascii_whitespace()
        // Skip label
        .skip(1)
        .map(|n| Ok(n.parse()?))
        .collect()
}

#[derive(Debug)]
struct Races {
    times: Vec<usize>,
    distances: Vec<usize>,
}

fn part1(races: &Races) -> usize {
    races
        .times
        .iter()
        .zip(&races.distances)
        .map(|(&time, &record_dist)| {
            (1..=time)
                .filter(|hold| hold * (time - hold) > record_dist)
                .count()
        })
        .product()
}

fn part2(races: &Races) -> usize {
    let races = Races {
        times: fix_kerning(&races.times),
        distances: fix_kerning(&races.distances),
    };
    part1(&races)
}

fn fix_kerning(nums: &[usize]) -> Vec<usize> {
    vec![
        nums.iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse()
            .unwrap(),
    ]
}
