use anyhow::{Error, Result};
use std::env;
use std::iter::successors;
use std::ops::{Div, Index, IndexMut, Rem};

fn main() -> Result<()> {
    let computer = parse_input()?;

    let diagnostic = run(&computer, 1);
    println!("Part 1: {diagnostic}");

    let input = env::args().nth(1).unwrap_or(String::from("5")).parse()?;
    let diagnostic = run(&computer, input);
    println!("Part 2: {diagnostic}");

    Ok(())
}

fn parse_input() -> Result<IntcodeComputer> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let memory: Vec<_> = line
        .trim()
        .split(",")
        .map(|n| n.parse().map_err(Error::from))
        .collect::<Result<_, _>>()?;

    let (input, output) = (vec![], vec![]);

    Ok(IntcodeComputer {
        ip: 0,
        memory,
        input,
        output,
    })
}

fn run(computer: &IntcodeComputer, input: isize) -> isize {
    let mut computer = computer.clone();
    computer.input.push(input);
    computer.run();
    computer.output.pop().unwrap()
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    ip: usize,
    memory: Vec<isize>,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl IntcodeComputer {
    fn run(&mut self) {
        loop {
            match self.decode() {
                Instruction::Add(params) => {
                    self[params[2].value as usize] = params[0].value(self) + params[1].value(self);
                    self.ip += params.len() + 1;
                }
                Instruction::Multiply(params) => {
                    self[params[2].value as usize] = params[0].value(self) * params[1].value(self);
                    self.ip += params.len() + 1;
                }
                Instruction::Input(param) => {
                    self[param.value as usize] = self.input.pop().unwrap();
                    self.ip += 2;
                }
                Instruction::Output(param) => {
                    self.output.push(param.value(self));
                    self.ip += 2;
                }
                Instruction::JumpIfTrue(params) => {
                    if params[0].value(self) != 0 {
                        self.ip = params[1].value(self) as usize;
                    } else {
                        self.ip += params.len() + 1;
                    }
                }
                Instruction::JumpIfFalse(params) => {
                    if params[0].value(self) == 0 {
                        self.ip = params[1].value(self) as usize;
                    } else {
                        self.ip += params.len() + 1;
                    }
                }
                Instruction::LessThan(params) => {
                    self[params[2].value as usize] =
                        if params[0].value(self) < params[1].value(self) {
                            1
                        } else {
                            0
                        };
                    self.ip += params.len() + 1;
                }
                Instruction::Equals(params) => {
                    self[params[2].value as usize] =
                        if params[0].value(self) == params[1].value(self) {
                            1
                        } else {
                            0
                        };
                    self.ip += params.len() + 1;
                }
                Instruction::Halt => {
                    break;
                }
            }
        }
    }

    fn decode(&self) -> Instruction {
        let instruction = self[self.ip];

        let mut mode_digits =
            successors(Some(instruction.div_rem(100)), |dr| Some(dr.0.div_rem(10)));

        let (_instruction, opcode) = mode_digits.next().unwrap();

        let mut params = mode_digits
            .map(|dr| Mode::from(dr.1))
            .enumerate()
            .map(|(i, mode)| (self[self.ip + i + 1], mode))
            .map(|(value, mode)| Parameter { value, mode });

        match opcode {
            1 => Instruction::Add(take_params::<3>(&mut params)),
            2 => Instruction::Multiply(take_params::<3>(&mut params)),
            3 => Instruction::Input(params.next().unwrap()),
            4 => Instruction::Output(params.next().unwrap()),
            5 => Instruction::JumpIfTrue(take_params::<2>(&mut params)),
            6 => Instruction::JumpIfFalse(take_params::<2>(&mut params)),
            7 => Instruction::LessThan(take_params::<3>(&mut params)),
            8 => Instruction::Equals(take_params::<3>(&mut params)),
            99 => Instruction::Halt,
            unknown => panic!("Unknown opcode {unknown}"),
        }
    }
}

fn take_params<const N: usize>(params: &mut impl Iterator<Item = Parameter>) -> [Parameter; N] {
    std::array::from_fn(|_| {
        params
            .next()
            .expect("instruction missing expected parameter")
    })
}

trait DivRem
where
    Self: Sized,
{
    fn div_rem(&self, divisor: Self) -> (Self, Self);
}

impl<T> DivRem for T
where
    T: Copy + Div<Output = T> + Rem<Output = T>,
{
    fn div_rem(&self, divisor: Self) -> (Self, Self) {
        (*self / divisor, *self % divisor)
    }
}

impl Index<usize> for IntcodeComputer {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for IntcodeComputer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Add([Parameter; 3]),
    Multiply([Parameter; 3]),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue([Parameter; 2]),
    JumpIfFalse([Parameter; 2]),
    LessThan([Parameter; 3]),
    Equals([Parameter; 3]),
    Halt,
}

#[derive(Debug, Clone)]
struct Parameter {
    value: isize,
    mode: Mode,
}

impl Parameter {
    fn value(&self, computer: &IntcodeComputer) -> isize {
        match self.mode {
            Mode::Position => computer[self.value as usize],
            Mode::Immediate => self.value,
        }
    }
}

#[derive(Debug, Clone)]
enum Mode {
    Position,
    Immediate,
}

impl From<isize> for Mode {
    fn from(value: isize) -> Self {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            v => panic!("Unknown mode {v}"),
        }
    }
}
