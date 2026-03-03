use anyhow::{Error, Result, anyhow, bail};
use primes::is_prime;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let program = parse_program()?;

    let mut process = Process::default();
    process.execute(&program);
    println!("Part 1: {}", process.mul_invoked);

    // See https://www.reddit.com/r/adventofcode/comments/7lms6p/comment/drnmlbk/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    // The assembly is doing an inefficient check for composite numbers in a custom range.
    // I'm still learning how to reverse engineer assembly, but someone had identical input :)
    let p2 = (109900..=126900)
        .step_by(17)
        .filter(|n| !is_prime(*n))
        .count();
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_program() -> Result<Vec<Instruction>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug, Default)]
struct Process {
    /// 'a' - 'h'
    registers: [isize; 8],
    ip: isize,
    mul_invoked: usize,
}

impl Process {
    fn execute(&mut self, program: &[Instruction]) {
        while self.ip >= 0 && self.ip < program.len() as isize {
            match program[self.ip as usize] {
                Instruction::Set(x, y) => *self.reg_mut(x) = self.val(y),
                Instruction::Sub(x, y) => *self.reg_mut(x) -= self.val(y),
                Instruction::Mul(x, y) => {
                    *self.reg_mut(x) *= self.val(y);
                    self.mul_invoked += 1;
                }
                Instruction::Jnz(x, y) => {
                    if self.val(x) != 0 {
                        self.ip += self.val(y);
                        // Skip normal IP incrementing
                        continue;
                    }
                }
            }

            // Normal IP incrementing
            self.ip += 1;
        }
    }

    fn val(&self, r: Value) -> isize {
        match r {
            Value::Reg(r) => self.reg(r),
            Value::Number(n) => n,
        }
    }

    fn reg(&self, r: char) -> isize {
        self.registers[r as usize - 'a' as usize]
    }

    fn reg_mut(&mut self, r: char) -> &mut isize {
        &mut self.registers[r as usize - 'a' as usize]
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Reg(char),
    Number(isize),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        match tokens[0] {
            "set" => Ok(Instruction::Set(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "sub" => Ok(Instruction::Sub(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "mul" => Ok(Instruction::Mul(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "jnz" => Ok(Instruction::Jnz(tokens.parse_val(1)?, tokens.parse_val(2)?)),
            _ => bail!("Unrecognized instruction: {}", tokens[0]),
        }
    }
}

trait ParseReg {
    fn parse_reg(&self, i: usize) -> Result<char>;
    fn parse_val(&self, i: usize) -> Result<Value>;
}

impl ParseReg for Vec<&str> {
    fn parse_reg(&self, i: usize) -> Result<char> {
        self.get(i)
            .filter(|t| t.len() == 1)
            .and_then(|t| t.chars().next())
            .filter(|&c| c.is_ascii_lowercase())
            .ok_or_else(|| anyhow!("Cannot parse ins[{i}] as register (must be 'a'..='z')"))
    }

    fn parse_val(&self, i: usize) -> Result<Value> {
        let token = self
            .get(i)
            .ok_or_else(|| anyhow!("Missing token at index {i}"))?;
        if let Ok(r) = self.parse_reg(i) {
            return Ok(Value::Reg(r));
        }
        if let Ok(n) = token.parse::<isize>() {
            return Ok(Value::Number(n));
        }
        bail!("Cannot parse ins[{i}] as register or number")
    }
}
