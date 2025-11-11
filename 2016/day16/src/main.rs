use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::{env, io};

fn main() -> Result<()> {
    let mut initial = String::new();
    io::stdin().read_line(&mut initial)?;
    let initial = initial.trim();

    let length = env::args()
        .nth(1)
        .ok_or(anyhow!("No length provided"))?
        .parse::<usize>()?;

    let data = dragon_to_length(initial, length)?;
    println!("Checksum: {}", checksum(&data));

    Ok(())
}

// b = !(rev(a)); new a = a0b
fn dragon_to_length(data: &str, length: usize) -> Result<String> {
    let mut data = data.to_string();
    while data.chars().count() < length {
        let b = data.chars().rev().map(|c| c.negate()).collect::<String>();
        data.push('0');
        data.push_str(&b);
    }
    Ok(data.chars().take(length).collect())
}

fn checksum(data: &str) -> String {
    let mut data = data.to_string();
    loop {
        let checksum: String = data
            .chars()
            .tuples()
            .map(|pair: (char, char)| pair.pair())
            .collect();

        if checksum.chars().count() % 2 == 1 {
            return checksum;
        }

        data = checksum;
    }
}

trait Negate {
    fn negate(&self) -> char;
}

impl Negate for char {
    fn negate(&self) -> char {
        if *self == '0' { '1' } else { '0' }
    }
}

trait Pair {
    fn pair(&self) -> char;
}
impl Pair for (char, char) {
    fn pair(&self) -> char {
        if self.0 == self.1 { '1' } else { '0' }
    }
}
