use anyhow::{Error, Result, bail};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let instructions = parse_input()?;

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));

    Ok(())
}

fn parse_input() -> Result<Vec<Instruction>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    value: isize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction, value) = s.split_at(1);

        let direction = match direction {
            "N" => Direction::North,
            "E" => Direction::East,
            "S" => Direction::South,
            "W" => Direction::West,
            "L" => Direction::Left,
            "R" => Direction::Right,
            "F" => Direction::Forward,
            _ => bail!("Unknown direction: {direction}"),
        };

        let value = value.parse()?;

        Ok(Instruction { direction, value })
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut ship = Ship::default();

    for Instruction { direction, value } in instructions {
        match direction {
            Direction::North => ship.coords.x -= value,
            Direction::East => ship.coords.y += value,
            Direction::South => ship.coords.x += value,
            Direction::West => ship.coords.y -= value,
            Direction::Left => {
                ship.angle -= value / 90;
                ship.angle = ship.angle.rem_euclid(LEN);
            }
            Direction::Right => {
                ship.angle += value / 90;
                ship.angle = ship.angle.rem_euclid(LEN);
            }
            Direction::Forward => {
                let (dx, dy) = DIRS[ship.angle as usize];
                ship.coords.x += dx * value;
                ship.coords.y += dy * value;
            }
        }
    }

    ship.coords.manhattan()
}

#[derive(Debug, Clone, Copy)]
struct Ship {
    /// Index into DIRS
    angle: isize,
    coords: Coords,
}

/// North, East, South, West
const DIRS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
const LEN: isize = DIRS.len() as isize;

#[derive(Debug, Default, Clone, Copy)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn manhattan(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            // East
            angle: 1,
            coords: Coords::default(),
        }
    }
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut ship = Coords::default();
    let mut waypoint = Coords {
        x: ship.x - 1,
        y: ship.y + 10,
    };

    for &Instruction {
        direction,
        mut value,
    } in instructions
    {
        match direction {
            Direction::North => waypoint.x -= value,
            Direction::East => waypoint.y += value,
            Direction::South => waypoint.x += value,
            Direction::West => waypoint.y -= value,
            Direction::Left | Direction::Right => {
                let (mut dx, mut dy) = (waypoint.x - ship.x, waypoint.y - ship.y);
                while value > 0 {
                    value -= 90;
                    if direction == Direction::Left {
                        (dx, dy) = (-dy, dx);
                    } else {
                        (dx, dy) = (dy, -dx);
                    }
                }
                (waypoint.x, waypoint.y) = (ship.x + dx, ship.y + dy);
            }
            Direction::Forward => {
                let (dx, dy) = (waypoint.x - ship.x, waypoint.y - ship.y);
                ship.x += dx * value;
                ship.y += dy * value;
                (waypoint.x, waypoint.y) = (ship.x + dx, ship.y + dy);
            }
        }
    }

    ship.manhattan()
}
