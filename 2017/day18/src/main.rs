use anyhow::{Error, Result, anyhow, bail};
use std::collections::VecDeque;
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let program = parse_program()?;
    let mut process = Process::default();
    process.execute(&program);
    println!("Part 1: {}", process.last_played.unwrap());

    let mut c1 = Process::new(0, Mode::Send);
    let mut c2 = Process::new(1, Mode::Send);
    loop {
        c1.execute(&program);
        c2.execute(&program);

        c1.out_buff.drain(0..).for_each(|v| c2.in_buff.push_back(v));
        c2.out_buff.drain(0..).for_each(|v| c1.in_buff.push_back(v));

        if c1.in_buff.is_empty() && c2.in_buff.is_empty() {
            break;
        }
    }
    println!("Part 2: {}", c2.sent);

    Ok(())
}

fn parse_program() -> Result<Vec<Instruction>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

#[derive(Debug, Default)]
struct Process {
    mode: Mode,
    registers: [isize; 26],
    ip: isize,
    last_played: Option<isize>,
    in_buff: VecDeque<isize>,
    out_buff: VecDeque<isize>,
    sent: usize,
}

#[derive(Debug, Default, PartialEq)]
enum Mode {
    #[default]
    Play,
    Send,
}

impl Process {
    fn new(pid: isize, mode: Mode) -> Self {
        let mut c = Self::default();
        *c.reg_mut('p') = pid;
        c.mode = mode;
        c
    }

    fn execute(&mut self, program: &[Instruction]) {
        while self.ip >= 0 && self.ip < program.len() as isize {
            match program[self.ip as usize] {
                Instruction::Snd(x) if self.mode == Mode::Play => {
                    self.last_played = Some(self.val(x))
                }
                Instruction::Snd(x) => {
                    self.out_buff.push_back(self.val(x));
                    self.sent += 1;
                }
                Instruction::Set(x, y) => *self.reg_mut(x) = self.val(y),
                Instruction::Add(x, y) => *self.reg_mut(x) += self.val(y),
                Instruction::Mul(x, y) => *self.reg_mut(x) *= self.val(y),
                Instruction::Mod(x, y) => *self.reg_mut(x) %= self.val(y),
                Instruction::Rcv(x) if self.mode == Mode::Play => {
                    if self.reg(x) != 0 && self.last_played.is_some() {
                        return;
                    }
                }
                Instruction::Rcv(x) => {
                    if let Some(value) = self.in_buff.pop_front() {
                        *self.reg_mut(x) = value;
                    } else {
                        return;
                    }
                }
                Instruction::Jgz(x, y) => {
                    if self.val(x) > 0 {
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
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
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
            "snd" => Ok(Instruction::Snd(tokens.parse_val(1)?)),
            "set" => Ok(Instruction::Set(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "add" => Ok(Instruction::Add(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "mul" => Ok(Instruction::Mul(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "mod" => Ok(Instruction::Mod(tokens.parse_reg(1)?, tokens.parse_val(2)?)),
            "rcv" => Ok(Instruction::Rcv(tokens.parse_reg(1)?)),
            "jgz" => Ok(Instruction::Jgz(tokens.parse_val(1)?, tokens.parse_val(2)?)),
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
        Err(anyhow!("Cannot parse ins[{i}] as register or number"))
    }
}
