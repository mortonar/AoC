use anyhow::{Error, Result, anyhow, bail};
use std::env::args;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let end = args().nth(1).and_then(|s| s.chars().next()).unwrap_or('p');
    let mut dancers: Vec<_> = ('a'..=end).collect();
    let moves = parse_moves()?;

    for m in &moves {
        m.apply(&mut dancers);
    }
    println!("Part 1: {}", dancers.iter().collect::<String>());

    // Positions repeat after every 60 dances, 1_000_000_000 % 60 == 40
    for _ in 0..39 {
        for m in &moves {
            m.apply(&mut dancers);
        }
    }
    println!("Part 2: {}", dancers.iter().collect::<String>());

    Ok(())
}

#[derive(Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    fn apply(&self, dancers: &mut [char]) {
        match self {
            Move::Spin(n) => dancers.rotate_right(*n),
            Move::Exchange(p1, p2) => dancers.swap(*p1, *p2),
            Move::Partner(p1, p2) => {
                let pos1 = dancers.iter().position(|d| d == p1).unwrap();
                let pos2 = dancers.iter().position(|d| d == p2).unwrap();
                dancers.swap(pos1, pos2);
            }
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let to_pos = |s: &str| -> Result<usize, Error> {
            let d = s.parse::<usize>()?;
            if d > 15 {
                bail!("Invalid position: {s}")
            } else {
                Ok(d)
            }
        };
        match s.chars().next().ok_or(anyhow!("Invalid move: {s}"))? {
            's' => Ok(Move::Spin(to_pos(&s[1..])?)),
            'x' => {
                let parts: Vec<_> = s[1..].split("/").collect();
                Ok(Move::Exchange(to_pos(parts[0])?, to_pos(parts[1])?))
            }
            'p' => {
                let chars: Vec<_> = s.chars().collect();
                Ok(Move::Partner(chars[1], chars[3]))
            }
            _ => bail!("Invalid move"),
        }
    }
}

fn parse_moves() -> Result<Vec<Move>> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    line.trim().split(",").map(|m| m.parse()).collect()
}
