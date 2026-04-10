use anyhow::{Error, Result};
use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let computer = parse_input()?;

    let mut p1 = computer.clone();
    p1[1] = 12;
    p1[2] = 2;
    p1.run();
    println!("Part 1: {}", p1[0]);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut p2 = computer.clone();
            p2[1] = noun;
            p2[2] = verb;

            p2.run();

            if p2[0] == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

fn parse_input() -> Result<IntcodeComputer> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let memory: Vec<_> = line
        .trim()
        .split(",")
        .map(|n| n.parse::<usize>().map_err(Error::from))
        .collect::<Result<_, _>>()?;

    Ok(IntcodeComputer { memory })
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    memory: Vec<usize>,
}

impl IntcodeComputer {
    fn run(&mut self) {
        let mut ip = 0;

        loop {
            let opcode = self.memory[ip];
            match opcode {
                1 => self.bin_op(ip, |p1, p2| p1 + p2),
                2 => self.bin_op(ip, |p1, p2| p1 * p2),
                99 => break,
                _ => panic!("Unknown opcode {opcode}"),
            }

            ip += 4;
        }
    }

    fn bin_op<F>(&mut self, ip: usize, op: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        let result = op(self[self[ip + 1]], self[self[ip + 2]]);
        let assign_to = self[ip + 3];
        self[assign_to] = result;
    }
}

impl Index<usize> for IntcodeComputer {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for IntcodeComputer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}
