use anyhow::{Error, Result};
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let passwords = parse_input()?;

    let (mut p1, mut p2) = (0, 0);
    for password in passwords {
        if password.is_valid() {
            p1 += 1;
        }
        if password.is_valid_xor() {
            p2 += 1;
        }
    }
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}

fn parse_input() -> Result<Vec<PasswordRecord>> {
    stdin().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug)]
struct PasswordRecord {
    password: Vec<char>,
    constraint: (char, (usize, usize)),
}

impl PasswordRecord {
    fn is_valid(&self) -> bool {
        let (ch, range) = self.constraint;
        let count = self.password.iter().filter(|&&c| c == ch).count();
        count >= range.0 && count <= range.1
    }

    fn is_valid_xor(&self) -> bool {
        let (ch, range) = self.constraint;
        (self.password[range.0 - 1] == ch) ^ (self.password[range.1 - 1] == ch)
    }
}

impl FromStr for PasswordRecord {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.trim().split_ascii_whitespace().collect();

        let password = tokens[2].chars().collect();

        let range = tokens[0].split('-').collect::<Vec<&str>>();
        let range = (range[0].parse()?, range[1].parse()?);
        let ch = tokens[1].chars().next().unwrap();
        let constraint = (ch, range);

        Ok(PasswordRecord {
            password,
            constraint,
        })
    }
}
