use anyhow::{Error, Result, anyhow};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let mut dial: isize = 50;
    let mut p1_password: usize = 0;
    let mut p2_password: usize = 0;

    for line in stdin().lock().lines() {
        let Rotation { amount, direction } = parse_rotation(line)?;

        for _ in 0..amount {
            match direction {
                Dir::Right => {
                    dial += 1;
                    if dial == 100 {
                        dial = 0;
                    }
                }
                Dir::Left => {
                    dial -= 1;
                    if dial == -1 {
                        dial = 99;
                    }
                }
            }

            if dial == 0 {
                p2_password += 1;
            }
        }

        if dial == 0 {
            p1_password += 1;
        }
    }

    println!("Part 1: {p1_password}");
    println!("Part 2: {p2_password}");

    Ok(())
}

fn parse_rotation(line: std::io::Result<String>) -> Result<Rotation> {
    let line = line?;
    let tokens: Vec<_> = line.trim().split(&['L', 'R']).collect();
    Ok(Rotation {
        amount: tokens[1].parse()?,
        direction: line[..=0].parse()?,
    })
}

struct Rotation {
    amount: usize,
    direction: Dir,
}

enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => Err(anyhow!("Invalid direction: {}", &s))?,
        })
    }
}
