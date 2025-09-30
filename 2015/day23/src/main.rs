use anyhow::{Error, Result};
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut program: Vec<Instruction> = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let instr = line.parse()?;
        program.push(instr);
    }

    let mut registers = Registers::default();
    run_program(&program, &mut registers);
    println!("Part 1: {}", registers.b);

    let mut registers = Registers::default();
    registers.a = 1;
    run_program(&program, &mut registers);
    println!("Part 2: {}", registers.b);

    Ok(())
}

fn run_program(program: &[Instruction], registers: &mut Registers) {
    while registers.ip < program.len() as isize && registers.ip >= 0 {
        let instruction = &program[registers.ip as usize];
        instruction.execute(registers);
    }
}

#[derive(Debug)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.trim().split_whitespace().collect();
        let no_reg_err = Error::msg("No register given");
        let reg = tokens[1].chars().next();
        match tokens[0] {
            "hlf" => Ok(Instruction::Hlf(reg.ok_or(no_reg_err)?)),
            "tpl" => Ok(Instruction::Tpl(reg.ok_or(no_reg_err)?)),
            "inc" => Ok(Instruction::Inc(reg.ok_or(no_reg_err)?)),
            "jmp" => Ok(Instruction::Jmp(tokens[1].parse()?)),
            "jie" => {
                let reg = reg.ok_or(no_reg_err)?;
                let offset = tokens[2].parse()?;
                Ok(Instruction::Jie(reg, offset))
            }
            "jio" => {
                let reg = reg.ok_or(no_reg_err)?;
                let offset = tokens[2].parse()?;
                Ok(Instruction::Jio(reg, offset))
            }
            _ => Err(Error::msg("Unknown instruction")),
        }
    }
}

impl Instruction {
    fn execute(&self, registers: &mut Registers) {
        match self {
            Instruction::Hlf(r) => {
                if *r == 'a' {
                    registers.a /= 2
                } else {
                    registers.b /= 2
                };
                registers.ip += 1;
            }
            Instruction::Tpl(r) => {
                if *r == 'a' {
                    registers.a *= 3
                } else {
                    registers.b *= 3
                };
                registers.ip += 1;
            }
            Instruction::Inc(r) => {
                if *r == 'a' {
                    registers.a += 1
                } else {
                    registers.b += 1
                };
                registers.ip += 1;
            }
            Instruction::Jmp(offset) => {
                registers.ip += *offset;
            }
            Instruction::Jie(r, offset) => {
                let reg = if *r == 'a' { registers.a } else { registers.b };
                if reg % 2 == 0 {
                    registers.ip += *offset;
                } else {
                    registers.ip += 1;
                }
            }
            Instruction::Jio(r, offset) => {
                let reg = if *r == 'a' { registers.a } else { registers.b };
                if reg == 1 {
                    registers.ip += *offset;
                } else {
                    registers.ip += 1;
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Registers {
    a: usize,
    b: usize,
    ip: isize,
}
