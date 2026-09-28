use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", part1(&program));

    println!("Part 2:");
    part2(&program);

    Ok(())
}

fn parse_input() -> Result<Program> {
    let instructions = stdin()
        .lock()
        .lines()
        .map(|l| l?.parse())
        .collect::<Result<Vec<_>>>()?;

    Ok(Program { instructions })
}

fn part1(program: &Program) -> isize {
    let mut sum = 0;
    let mut special = vec![220, 180, 140, 100, 60, 20];
    let mut x = 1;
    for (cycle, delta) in program.with_cycles() {
        if *special.last().unwrap() == cycle {
            sum += cycle * x;

            special.pop();
            if special.is_empty() {
                break;
            }
        }

        x += delta;
    }
    sum
}

fn part2(program: &Program) {
    let mut x = 1;
    for (cycle, delta) in program.with_cycles() {
        let draw_pos = (cycle - 1) % 40;
        if [x - 1, x, x + 1].contains(&draw_pos) {
            print!("#")
        } else {
            print!(".")
        }

        if cycle % 40 == 0 {
            println!();
        }

        x += delta;
    }
}

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    Noop,
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        match tokens[0] {
            "addx" => Ok(Instruction::AddX(tokens[1].parse()?)),
            "noop" => Ok(Instruction::Noop),
            _ => bail!("unrecognized instruction: {s}"),
        }
    }
}

impl Program {
    fn with_cycles(&self) -> impl Iterator<Item = (isize, isize)> {
        (1..).zip(self.instructions.iter().flat_map(|i| {
            let (delta, cycles) = match i {
                Instruction::AddX(v) => (*v, 2),
                Instruction::Noop => (0, 1),
            };

            [0, delta].into_iter().take(cycles)
        }))
    }
}
