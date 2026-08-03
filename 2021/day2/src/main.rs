use anyhow::{Error, Result};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let course = parse_input()?;

    println!("Part 1 : {}", part1(&course));
    println!("Part 2 : {}", part2(&course));

    Ok(())
}

fn parse_input() -> Result<Vec<Direction>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(course: &[Direction]) -> isize {
    let (h, d) = course.iter().fold((0, 0), |(h, d), dir| match dir {
        Direction::Forward(u) => (h + u, d),
        Direction::Down(u) => (h, d + u),
        Direction::Up(u) => (h, d - u),
    });
    h * d
}

fn part2(course: &[Direction]) -> isize {
    let (_a, h, d) = course.iter().fold((0, 0, 0), |(a, h, d), dir| match dir {
        Direction::Down(u) => (a + u, h, d),
        Direction::Up(u) => (a - u, h, d),
        Direction::Forward(u) => (a, h + u, d + a * u),
    });
    h * d
}

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.trim().split_ascii_whitespace().collect();
        let units = tokens[1].parse()?;
        match tokens[0] {
            "forward" => Ok(Direction::Forward(units)),
            "down" => Ok(Direction::Down(units)),
            "up" => Ok(Direction::Up(units)),
            _ => Err(Error::msg("Invalid direction")),
        }
    }
}
