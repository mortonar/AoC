use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let snafus = parse_input()?;

    println!("Part 1: {}", part1(&snafus).0);

    Ok(())
}

fn parse_input() -> Result<Vec<Snafu>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(snafus: &[Snafu]) -> Snafu {
    snafus.iter().map(isize::from).sum::<isize>().into()
}

#[derive(Debug)]
struct Snafu(String);

impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<&Snafu> for isize {
    fn from(value: &Snafu) -> Self {
        value
            .0
            .chars()
            .zip((0..value.0.len()).rev())
            .fold(0isize, |acc, (c, pos)| {
                acc + 5isize.pow(pos as u32)
                    * match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        d => panic!("invalid digit: {d}"),
                    }
            })
    }
}

impl From<isize> for Snafu {
    fn from(mut value: isize) -> Self {
        let mut digits = Vec::new();
        while value > 0 {
            let rem = value.rem_euclid(5);
            value /= 5;
            match rem {
                0 => digits.push('0'),
                1 => digits.push('1'),
                2 => digits.push('2'),
                3 => digits.push('='),
                4 => digits.push('-'),
                d => panic!("invalid digit: {d}"),
            }
            if rem >= 3 {
                value += 1;
            }
        }
        Snafu(digits.iter().rev().collect())
    }
}
