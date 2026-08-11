use anyhow::{Error, Result, bail};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let signals_digits = parse_input()?;

    println!("Part 1: {}", part1(&signals_digits));
    println!("Part 2: {}", part2(&signals_digits));

    Ok(())
}

type Input = Vec<(Vec<u8>, Vec<u8>)>;

fn parse_input() -> Result<Input> {
    stdin()
        .lock()
        .lines()
        .map(|l| {
            let l = l.map_err(Error::from)?;
            let Some((signals, digits)) = l.trim().split_once(" | ") else {
                bail!("Missing delimiter ' | '")
            };
            let patterns = signals
                .split_ascii_whitespace()
                .map(to_bitmask)
                .collect::<Result<Vec<_>>>()?;
            let digits = digits
                .split_ascii_whitespace()
                .map(to_bitmask)
                .collect::<Result<Vec<_>>>()?;
            Ok((patterns, digits))
        })
        .collect()
}

fn to_bitmask(s: &str) -> Result<u8> {
    let mut mask = 0u8;
    for c in s.chars() {
        let v = c as usize;
        if !(97..=103).contains(&v) {
            bail!("Invalid character in digit: {c}");
        }
        mask |= 1 << (v - 97);
    }
    Ok(mask)
}

fn part1(signals_digits: &[(Vec<u8>, Vec<u8>)]) -> usize {
    signals_digits
        .iter()
        .flat_map(|(_s, d)| d)
        .filter(|d| matches!(d.count_ones(), 2 | 3 | 4 | 7))
        .count()
}

fn part2(signals_digits: &[(Vec<u8>, Vec<u8>)]) -> usize {
    signals_digits
        .iter()
        .map(|(signals, digits)| (decode(signals), digits))
        .map(|(decoded, digits)| {
            digits
                .iter()
                .fold(0, |acc, &d| acc * 10 + eval(d, &decoded))
        })
        .sum()
}

fn decode(patterns: &[u8]) -> [u8; 7] {
    let one = patterns.iter().find(|p| p.count_ones() == 2).unwrap();
    let seven = patterns.iter().find(|p| p.count_ones() == 3).unwrap();
    let four = patterns.iter().find(|p| p.count_ones() == 4).unwrap();
    let eight = patterns.iter().find(|p| p.count_ones() == 7).unwrap();
    let a = one ^ seven;
    let bd = one ^ four;
    let eg = eight ^ (one | four | seven);
    let cf = seven & one;
    let adg = patterns
        .iter()
        .filter(|p| p.count_ones() == 5)
        .fold(u8::MAX, |acc, &p| acc & p);
    let d = adg & bd;
    let g = adg & eg;
    let b = bd & !d;
    let e = eg & !g;
    let abfg = patterns
        .iter()
        .filter(|p| p.count_ones() == 6)
        .fold(u8::MAX, |acc, &p| acc & p);
    let f = abfg & cf;
    let c = cf & !f;
    [a, b, c, d, e, f, g]
}

fn eval(scrambled_mask: u8, decoded: &[u8; 7]) -> usize {
    let mut mask: u8 = 0;
    for (segment, &wire) in decoded.iter().enumerate() {
        if (scrambled_mask & wire) != 0 {
            mask |= 1 << segment;
        }
    }

    match mask {
        0b01110111 => 0,
        0b00100100 => 1,
        0b01011101 => 2,
        0b01101101 => 3,
        0b00101110 => 4,
        0b01101011 => 5,
        0b01111011 => 6,
        0b00100101 => 7,
        0b01111111 => 8,
        0b01101111 => 9,
        _ => unreachable!(),
    }
}
