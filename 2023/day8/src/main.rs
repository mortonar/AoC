use std::{
    collections::HashMap,
    io::{BufRead, stdin},
};

use anyhow::{Error, Result, anyhow, bail};

fn main() -> Result<()> {
    let map = parse_input()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part1(map: &Map) -> usize {
    map.min_steps("AAA", |n: &str| n.ends_with("ZZZ"))
}

// Inputs conveniently contain discrete closed loops of length T cycle length T for each ("XXA", "XXZ") pair
fn part2(map: &Map) -> usize {
    map.nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| map.min_steps(n, |n: &str| n.ends_with("Z")))
        .reduce(num_integer::lcm)
        .unwrap()
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<String, [String; 2]>,
}

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Map {
    fn min_steps<F>(&self, start: &str, end: F) -> usize
    where
        F: Fn(&str) -> bool,
    {
        let mut ins_idx = 0;
        let mut steps = 0;
        let mut current = start;
        while !end(current) {
            let to = &self.nodes[current];
            current = &to[self.instructions[ins_idx] as usize];
            steps += 1;
            ins_idx = (ins_idx + 1) % self.instructions.len();
        }
        steps
    }
}

fn parse_input() -> Result<Map> {
    let mut lines = stdin().lock().lines();

    let instructions = lines
        .next()
        .ok_or_else(|| anyhow!("missing instructions"))??
        .chars()
        .map(|c| c.try_into())
        .collect::<Result<Vec<_>>>()?;

    lines.next().ok_or_else(|| anyhow!("missing nodes"))??;

    let mut nodes = HashMap::new();
    for line in lines.by_ref() {
        let line = line?;
        let Some((from, to)) = line.split_once(" = ") else {
            bail!("illformatted node line: {line}");
        };
        let Some((left, right)) = to.split_once(", ") else {
            bail!("illformatted node line: {line}");
        };
        nodes.insert(
            from.to_string(),
            [left[1..].to_string(), right[0..right.len() - 1].to_string()],
        );
    }

    Ok(Map {
        instructions,
        nodes,
    })
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(anyhow!("invalid instruction: {value}")),
        }
    }
}
