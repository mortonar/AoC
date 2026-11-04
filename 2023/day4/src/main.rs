use std::{
    collections::HashSet,
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let cards = parse_input()?;

    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));

    Ok(())
}

fn parse_input() -> Result<Vec<Card>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(cards: &[Card]) -> usize {
    cards.iter().map(|c| c.value).sum()
}

fn part2(cards: &[Card]) -> usize {
    let mut copies = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        for j in 0..card.matches {
            copies[i + j + 1] += copies[i];
        }
    }

    copies.iter().sum()
}

#[derive(Debug)]
struct Card {
    matches: usize,
    value: usize,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let Some((_num, numbers)) = s.split_once(": ") else {
            bail!("ill-formatted card - card number: {s}");
        };

        let Some((nums, winning)) = numbers.split_once(" | ") else {
            bail!("ill-formatted card - number format: {s}");
        };
        let (nums, winning) = (parse_nums(nums)?, parse_nums(winning)?);
        let matches = nums.intersection(&winning).count();
        let value = match matches {
            0 => 0,
            m => 1 << (m - 1),
        };

        Ok(Card { matches, value })
    }
}

fn parse_nums(s: &str) -> Result<HashSet<usize>> {
    s.split_ascii_whitespace().map(|n| Ok(n.parse()?)).collect()
}
