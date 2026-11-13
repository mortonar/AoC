use std::io::{BufRead, stdin};

use anyhow::Result;

fn main() -> Result<()> {
    let histories = parse_input()?;

    println!("Part 1: {}", part1(&histories));
    println!("Part 2: {}", part2(&histories));

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<isize>>> {
    stdin()
        .lock()
        .lines()
        .map(|l| {
            l?.split_ascii_whitespace()
                .map(|v| Ok(v.parse()?))
                .collect()
        })
        .collect()
}

fn part1(histories: &[Vec<isize>]) -> isize {
    histories
        .iter()
        .map(|h| extrapolate(h, &|h, below| h.last().unwrap() + below))
        .sum()
}

fn part2(histories: &[Vec<isize>]) -> isize {
    histories
        .iter()
        .map(|h| extrapolate(h, &|h, below| h.first().unwrap() - below))
        .sum()
}

fn extrapolate<F>(history: &[isize], ext: &F) -> isize
where
    F: Fn(&[isize], isize) -> isize,
{
    if history.iter().all(|&v| v == 0) {
        0
    } else {
        let diffs = history
            .windows(2)
            .map(|win| win[1] - win[0])
            .collect::<Vec<_>>();
        ext(history, extrapolate(&diffs, ext))
    }
}
