use anyhow::{Error, Result, anyhow};
use std::io::BufRead;
use std::str::FromStr;

fn main() -> Result<()> {
    let instructions = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line?.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, Error>>()?;

    let a_reg = Register { label: 'a' };
    let mut cpu = Cpu::default();
    cpu.assign_register(&a_reg, 7);
    cpu.run(instructions.clone().as_mut_slice());
    println!("Part 1: {}", cpu.eval_reg(&a_reg));

    // TODO Optimize
    cpu.reset();
    cpu.assign_register(&a_reg, 12);
    cpu.run(instructions.clone().as_mut_slice());
    println!("Part 1: {}", cpu.eval_reg(&a_reg));

    Ok(())
}

#[derive(Clone, Debug)]
enum Instruction {
    Cpy(Either, Either),
    Inc(Either),
    Dec(Either),
    Jnz(Either, Either),
    Tgl(Either),
}

impl Instruction {
    fn toggle(&self) -> Self {
        match self {
            Instruction::Cpy(either1, either2) => {
                Instruction::Jnz(either1.clone(), either2.clone())
            }
            Instruction::Inc(either) => Instruction::Dec(either.clone()),
            Instruction::Dec(either) => Instruction::Inc(either.clone()),
            Instruction::Jnz(either1, either2) => {
                Instruction::Cpy(either1.clone(), either2.clone())
            }
            Instruction::Tgl(either) => Instruction::Inc(either.clone()),
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        match tokens[0] {
            "cpy" => Ok(Instruction::Cpy(tokens[1].parse()?, tokens[2].parse()?)),
            "inc" => Ok(Instruction::Inc(tokens[1].parse()?)),
            "dec" => Ok(Instruction::Dec(tokens[1].parse()?)),
            "jnz" => Ok(Instruction::Jnz(tokens[1].parse()?, tokens[2].parse()?)),
            "tgl" => Ok(Instruction::Tgl(tokens[1].parse()?)),
            _ => Err(anyhow!("Invalid instruction: {}", s)),
        }
    }
}

#[derive(Clone, Debug)]
enum Either {
    Integer(isize),
    Register(Register),
}

impl FromStr for Either {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim().chars().count() < 1 {
            return Err(anyhow!("Invalid string: {}", s));
        }
        let c = s.trim().chars().next().unwrap();
        if c.is_ascii_digit() || c == '-' {
            Ok(Either::Integer(s.parse::<isize>()?))
        } else {
            Ok(Either::Register(s.parse::<Register>()?))
        }
    }
}

#[derive(Clone, Debug)]
struct Register {
    label: char,
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim().chars().count() != 1 {
            return Err(anyhow!("Invalid register: {}", s));
        }
        let c = s.trim().chars().next().unwrap();
        match c {
            'a' | 'b' | 'c' | 'd' => Ok(Self { label: c }),
            _ => Err(anyhow!("Invalid register: {}", c)),
        }
    }
}

impl Register {
    fn offset(&self) -> usize {
        self.label as usize - 'a' as usize
    }
}

#[derive(Debug, Default)]
struct Cpu {
    ip: isize,
    named_regs: [isize; 4],
}

impl Cpu {
    fn run(&mut self, instruction: &mut [Instruction]) {
        while self.ip > -1 && (self.ip as usize) < instruction.len() {
            match &instruction[self.ip as usize] {
                Instruction::Cpy(either, Either::Register(register)) => {
                    self.assign_register(register, self.eval_either(either));
                    self.ip += 1;
                }
                Instruction::Inc(Either::Register(register)) => {
                    self.assign_register(register, self.eval_reg(register) + 1);
                    self.ip += 1;
                }
                Instruction::Dec(Either::Register(register)) => {
                    self.assign_register(register, self.eval_reg(register) - 1);
                    self.ip += 1;
                }
                Instruction::Jnz(either1, either2) => {
                    let x = self.eval_either(either1);
                    if x != 0 {
                        let y = self.eval_either(either2);
                        self.ip += y;
                    } else {
                        self.ip += 1;
                    }
                }
                Instruction::Tgl(either) => {
                    let ins = self.eval_either(either) + self.ip;
                    self.ip += 1;
                    if ins < 0 || (ins as usize) >= instruction.len() {
                        continue;
                    }
                    let ins = &mut instruction[ins as usize];
                    *ins = ins.toggle();
                }
                // If an instruction becomes invalid, just ignore it: no-op.
                _ => {}
            }
        }
    }

    fn eval_either(&self, either: &Either) -> isize {
        match either {
            Either::Integer(i) => *i,
            Either::Register(r) => self.eval_reg(r),
        }
    }

    fn eval_reg(&self, register: &Register) -> isize {
        self.named_regs[register.offset()]
    }

    fn assign_register(&mut self, register: &Register, value: isize) {
        self.named_regs[register.offset()] = value;
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.named_regs.fill(0);
    }
}
