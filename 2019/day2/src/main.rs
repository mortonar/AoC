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

    Ok(IntcodeComputer { ip: 0, memory })
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    ip: usize,
    memory: Vec<usize>,
}

impl IntcodeComputer {
    fn run(&mut self) {
        loop {
            match self.opcode() {
                1 => self.bin_op(|p1, p2| p1 + p2),
                2 => self.bin_op(|p1, p2| p1 * p2),
                99 => break,
                op => panic!("Unknown opcode {op}"),
            }

            self.next_ins();
        }
    }

    fn opcode(&self) -> usize {
        self[self.ip]
    }

    fn next_ins(&mut self) {
        self.ip += 4;
    }

    fn params(&self) -> [usize; 3] {
        [self[self[self.ip + 1]], self[self[self.ip + 2]], self[self.ip + 3]]
    }

    fn bin_op<F>(&mut self, op: F)
    where
        F: Fn(usize, usize) -> usize,
    {
        let [p1, p2, p3] = self.params();
        self[p3] = op(p1, p2);
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
