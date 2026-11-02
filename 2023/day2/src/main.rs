use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, anyhow};

fn main() -> Result<()> {
    let games = parse_input()?;

    println!("Part 1: {}", part1(&games));
    println!("Part 2: {}", part2(&games));

    Ok(())
}

fn parse_input() -> Result<Vec<Game>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(games: &[Game]) -> usize {
    let bag = [12, 13, 14];
    games
        .iter()
        .filter(|g| g.is_possible(&bag))
        .map(|g| g.id)
        .sum()
}

fn part2(games: &[Game]) -> usize {
    games.iter().map(Game::power).sum()
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    // Red, Green, Blue
    cubes: [usize; 3],
}

impl Game {
    fn is_possible(&self, bag: &[usize; 3]) -> bool {
        self.sets.iter().all(|s| s.is_possible(bag))
    }

    fn power(&self) -> usize {
        self.sets
            .iter()
            .fold([0; 3], |max, s| {
                std::array::from_fn(|i| max[i].max(s.cubes[i]))
            })
            .iter()
            .product()
    }
}

impl Set {
    fn is_possible(&self, bag: &[usize; 3]) -> bool {
        self.cubes.iter().zip(bag).all(|(&s, &b)| s <= b)
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let game_prefix = s
            .split(':')
            .next()
            .ok_or_else(|| anyhow!("invalid game format"))?;
        let id = game_prefix[5..].parse()?;

        let sets = s.split(';').map(str::parse).collect::<Result<_>>()?;

        Ok(Self { id, sets })
    }
}

impl FromStr for Set {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut cubes = [0; 3];
        let mut num = "";
        for token in s.split_ascii_whitespace() {
            if token.starts_with("red") {
                cubes[0] = num.parse()?;
            } else if token.starts_with("green") {
                cubes[1] = num.parse()?;
            } else if token.starts_with("blue") {
                cubes[2] = num.parse()?;
            }

            num = token;
        }

        Ok(Self { cubes })
    }
}
