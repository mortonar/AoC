use anyhow::{Error, Result, bail};
use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", run(&program, Version::V1));
    println!("Part 2: {}", run(&program, Version::V2));

    Ok(())
}

fn parse_input() -> Result<Vec<Instruction>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn run(program: &[Instruction], version: Version) -> usize {
    let mut memory = HashMap::new();
    let mut mask = Mask::default();
    let apply = match version {
        Version::V1 => apply_v1,
        Version::V2 => apply_v2,
    };

    for instruction in program.iter() {
        match instruction {
            Instruction::Mask(m) => {
                mask = m.clone();
            }
            Instruction::Mem(addr, val) => apply(&mut memory, &mask, *addr, *val),
        }
    }

    memory.values().sum()
}

fn apply_v1(memory: &mut HashMap<usize, usize>, mask: &Mask, addr: usize, value: usize) {
    memory.insert(addr, (mask.ones | value) & mask.zeros);
}

fn apply_v2(memory: &mut HashMap<usize, usize>, mask: &Mask, addr: usize, value: usize) {
    let addr = addr | mask.ones;
    let mut addresses = vec![addr];
    for &bit in &mask.floating {
        let one_mask = 1 << bit;
        let zero_mask = !one_mask;
        addresses = addresses
            .iter()
            .flat_map(|&a| [a | one_mask, a & zero_mask])
            .collect();
    }

    for &addr in addresses.iter() {
        memory.insert(addr, value);
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem(usize, usize),
}

#[derive(Debug, Clone)]
struct Mask {
    ones: usize,
    zeros: usize,
    floating: Vec<usize>,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(&[' ', '[', ']']).collect();
        match tokens[0] {
            "mask" => Ok(Instruction::Mask(tokens[2].parse()?)),
            "mem" => Ok(Instruction::Mem(tokens[1].parse()?, tokens[4].parse()?)),
            _ => bail!("Unrecognized instruction: {s}"),
        }
    }
}

impl FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (mut zeros, mut ones) = (usize::MAX, 0);
        let mut floating = Vec::new();
        for (i, bit) in s.trim().chars().rev().enumerate() {
            match bit {
                'X' => floating.push(i),
                '1' => ones |= 1 << i,
                '0' => zeros &= !(1 << i),
                _ => bail!("Unrecognized bit: {bit}"),
            }
        }
        Ok(Self {
            ones,
            zeros,
            floating,
        })
    }
}

impl Default for Mask {
    fn default() -> Self {
        let (ones, zeros) = (usize::MAX, usize::MAX);
        let floating = Vec::new();
        Self {
            ones,
            zeros,
            floating,
        }
    }
}

enum Version {
    V1,
    V2,
}
