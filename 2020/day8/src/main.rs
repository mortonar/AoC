use anyhow::{Error, Result, bail};
use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let program = parse_input()?;

    let mut computer = Computer::new(program.clone());
    computer.execute_until_repeat();
    println!("Part 1: {}", computer.acc);

    for (i, ins) in program.iter().enumerate() {
        let mut program = program.clone();
        if let Instruction::Jmp(j) = ins {
            program[i] = Instruction::Nop(*j);
        } else if let Instruction::Nop(n) = ins {
            program[i] = Instruction::Jmp(*n);
        } else {
            continue;
        }

        let mut computer = Computer::new(program);
        if computer.execute_until_term() {
            println!("Part 2: {}", computer.acc);
            break;
        }
    }

    Ok(())
}

fn parse_input() -> Result<Program> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

type Program = Vec<Instruction>;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.trim().split(" ").collect();
        let val = tokens[1].parse()?;
        match tokens[0] {
            "acc" => Ok(Instruction::Acc(val)),
            "jmp" => Ok(Instruction::Jmp(val)),
            "nop" => Ok(Instruction::Nop(val)),
            _ => bail!("Invalid instruction: {s}"),
        }
    }
}

struct Computer {
    ip: usize,
    acc: isize,
    program: Program,
}

impl Computer {
    fn new(program: Program) -> Self {
        Self {
            ip: 0,
            acc: 0,
            program,
        }
    }

    fn execute_until_repeat(&mut self) {
        let mut executed = HashSet::new();
        while executed.insert(self.ip) {
            self.execute_ins();
        }
    }

    fn execute_until_term(&mut self) -> bool {
        let mut ins_executed = 0;
        while self.ip != self.program.len() {
            self.execute_ins();

            // If we toil away for too long, bail and try changing another instruction
            ins_executed += 1;
            if ins_executed == 1000 {
                return false;
            }
        }
        true
    }

    fn execute_ins(&mut self) {
        match self.program[self.ip] {
            Instruction::Acc(a) => {
                self.acc += a;
                self.ip += 1;
            }
            Instruction::Jmp(j) => {
                self.ip = ((self.ip as isize) + j) as usize;
            }
            Instruction::Nop(_n) => self.ip += 1,
        }
    }
}
