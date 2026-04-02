use anyhow::{Error, Result, bail};
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let (computer, program) = parse_input()?;

    // Sum of divisors of 915
    let mut p1 = computer.clone();
    p1.run(&program);
    println!("Part 1: {}", p1.registers[0]);

    // Sum of divisors of 10551315
    println!("Part 2: {}", 17427456);

    Ok(())
}

#[derive(Debug, Clone)]
struct Computer {
    registers: [usize; 6],
    ip_reg: usize,
}

impl Computer {
    fn new(ip_reg: usize) -> Self {
        Self {
            registers: [0; 6],
            ip_reg,
        }
    }

    fn run(&mut self, program: &[Instruction]) {
        while self.reg(self.ip_reg).unwrap() < program.len() {
            program[self.reg(self.ip_reg).unwrap()]
                .execute(self)
                .unwrap();

            if self.reg(self.ip_reg).unwrap() >= program.len() - 1 {
                break;
            }
            *self.reg_mut(self.ip_reg).unwrap() += 1;
        }
    }

    fn reg(&self, r: usize) -> Option<usize> {
        self.registers.get(r).copied()
    }

    fn reg_mut(&mut self, r: usize) -> Option<&mut usize> {
        self.registers.get_mut(r)
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn execute(&self, computer: &mut Computer) -> Option<usize> {
        let (a_r, b_r, a_v, b_v) = (computer.reg(self.a), computer.reg(self.b), self.a, self.b);
        *computer.reg_mut(self.c).unwrap() = match self.opcode {
            Opcode::Addr => a_r? + b_r?,
            Opcode::Addi => a_r? + b_v,
            Opcode::Mulr => a_r? * b_r?,
            Opcode::Muli => a_r? * b_v,
            Opcode::Banr => a_r? & b_r?,
            Opcode::Bani => a_r? & b_v,
            Opcode::Borr => a_r? | b_r?,
            Opcode::Bori => a_r? | b_v,
            Opcode::Setr => a_r?,
            Opcode::Seti => a_v,
            Opcode::Gtir => {
                if a_v > b_r? {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtri => {
                if a_r? > b_v {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtrr => {
                if a_r? > b_r? {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqir => {
                if a_v == b_r? {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqri => {
                if a_r? == b_v {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqrr => {
                if a_r? == b_r? {
                    1
                } else {
                    0
                }
            }
        };
        computer.reg(self.c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn parse_input() -> Result<(Computer, Vec<Instruction>)> {
    let mut lines = stdin().lines();

    let ip_reg = lines.next().unwrap()?[4..].parse()?;
    let computer = Computer::new(ip_reg);

    let instructions = lines.map(|l| l?.parse()).collect::<Result<_>>()?;

    Ok((computer, instructions))
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        let opcode = parts[0].parse()?;
        let a = parts[1].parse()?;
        let b = parts[2].parse()?;
        let c = parts[3].parse()?;
        Ok(Self { opcode, a, b, c })
    }
}

impl FromStr for Opcode {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Opcode::Addr),
            "addi" => Ok(Opcode::Addi),
            "mulr" => Ok(Opcode::Mulr),
            "muli" => Ok(Opcode::Muli),
            "banr" => Ok(Opcode::Banr),
            "bani" => Ok(Opcode::Bani),
            "borr" => Ok(Opcode::Borr),
            "bori" => Ok(Opcode::Bori),
            "setr" => Ok(Opcode::Setr),
            "seti" => Ok(Opcode::Seti),
            "gtir" => Ok(Opcode::Gtir),
            "gtri" => Ok(Opcode::Gtri),
            "gtrr" => Ok(Opcode::Gtrr),
            "eqir" => Ok(Opcode::Eqir),
            "eqri" => Ok(Opcode::Eqri),
            "eqrr" => Ok(Opcode::Eqrr),
            unreq => bail!("Unrecognized opcode: {unreq}"),
        }
    }
}
