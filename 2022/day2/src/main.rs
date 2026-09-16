use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let guide = parse_input()?;

    println!("Part 1: {}", part1(&guide));
    println!("Part 2: {}", part2(&guide));

    Ok(())
}

fn parse_input() -> Result<Guide> {
    let mut rounds = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let Some((opponent, you)) = line.split_once(" ") else {
            bail!("invalid guide line: {line}");
        };
        let outcome = you;
        let (opponent, you, outcome) = (opponent.parse()?, you.parse()?, outcome.parse()?);
        rounds.push((opponent, you, outcome))
    }
    Ok(Guide { rounds })
}

fn part1(guide: &Guide) -> usize {
    guide
        .rounds
        .iter()
        .map(|(opponent, you, _)| you.score(opponent))
        .sum()
}

fn part2(guide: &Guide) -> usize {
    guide
        .rounds
        .iter()
        .map(|(opponent, _, outcome)| outcome.pick(opponent))
        .sum()
}

#[derive(Debug)]
struct Guide {
    rounds: Vec<(Shape, Shape, Outcome)>,
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self, other: &Shape) -> usize {
        match self {
            Shape::Rock => {
                1 + match other {
                    Shape::Rock => 3,
                    Shape::Paper => 0,
                    Shape::Scissors => 6,
                }
            }
            Shape::Paper => {
                2 + match other {
                    Shape::Rock => 6,
                    Shape::Paper => 3,
                    Shape::Scissors => 0,
                }
            }
            Shape::Scissors => {
                3 + match other {
                    Shape::Rock => 0,
                    Shape::Paper => 6,
                    Shape::Scissors => 3,
                }
            }
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    #[allow(clippy::identity_op)]
    fn pick(&self, opponent: &Shape) -> usize {
        match (opponent, self) {
            (Shape::Rock, Outcome::Lose) => 3 + 0,
            (Shape::Rock, Outcome::Draw) => 1 + 3,
            (Shape::Rock, Outcome::Win) => 2 + 6,
            (Shape::Paper, Outcome::Lose) => 1 + 0,
            (Shape::Paper, Outcome::Draw) => 2 + 3,
            (Shape::Paper, Outcome::Win) => 3 + 6,
            (Shape::Scissors, Outcome::Lose) => 2 + 0,
            (Shape::Scissors, Outcome::Draw) => 3 + 3,
            (Shape::Scissors, Outcome::Win) => 1 + 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => bail!("unreocognized outcome: {s}"),
        }
    }
}

impl FromStr for Shape {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => bail!("unreocognized shape: {s}"),
        }
    }
}
