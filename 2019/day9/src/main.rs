use anyhow::{Result, anyhow};
use std::collections::VecDeque;
use std::iter::successors;
use std::ops::{Div, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", run_boost(&program, 1)?);
    println!("Part 2: {}", run_boost(&program, 2)?);

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    line.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

fn run_boost(program: &Program, input: isize) -> Result<isize> {
    let mut computer = IntcodeComputer::new(program);
    computer.input.push_back(input);
    computer.run();
    computer.output.pop_front().ok_or(anyhow!("No output"))
}

type Program = Vec<isize>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum RunState {
    Halted,
    AwaitingInput,
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    ip: usize,
    relative_base: isize,
    memory: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}

impl IntcodeComputer {
    fn new(program: &Program) -> Self {
        // Allocate and zero-out extra main memory to allow writes past the program's end
        let mut memory = program.clone();
        memory.resize(program.len() * 50, 0);

        Self {
            ip: 0,
            relative_base: 0,
            memory,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    fn run(&mut self) -> RunState {
        loop {
            match self.decode() {
                Instruction::Add(params) => {
                    let result = self.param_value(&params[0]) + self.param_value(&params[1]);
                    let addr = self.write_addr(&params[2]);
                    self.memory[addr] = result;
                    self.ip += params.len() + 1;
                }
                Instruction::Multiply(params) => {
                    let result = self.param_value(&params[0]) * self.param_value(&params[1]);
                    let addr = self.write_addr(&params[2]);
                    self.memory[addr] = result;
                    self.ip += params.len() + 1;
                }
                Instruction::Input(param) => {
                    let Some(input) = self.input.pop_front() else {
                        return RunState::AwaitingInput;
                    };
                    let addr = self.write_addr(&param);
                    self.memory[addr] = input;
                    self.ip += 2;
                }
                Instruction::Output(param) => {
                    self.output.push_back(self.param_value(&param));
                    self.ip += 2;
                }
                Instruction::JumpIfTrue(params) => {
                    if self.param_value(&params[0]) != 0 {
                        self.ip = self.param_value(&params[1]) as usize;
                    } else {
                        self.ip += params.len() + 1;
                    }
                }
                Instruction::JumpIfFalse(params) => {
                    if self.param_value(&params[0]) == 0 {
                        self.ip = self.param_value(&params[1]) as usize;
                    } else {
                        self.ip += params.len() + 1;
                    }
                }
                Instruction::LessThan(params) => {
                    let result =
                        isize::from(self.param_value(&params[0]) < self.param_value(&params[1]));
                    let addr = self.write_addr(&params[2]);
                    self.memory[addr] = result;
                    self.ip += params.len() + 1;
                }
                Instruction::Equals(params) => {
                    let result =
                        isize::from(self.param_value(&params[0]) == self.param_value(&params[1]));
                    let addr = self.write_addr(&params[2]);
                    self.memory[addr] = result;
                    self.ip += params.len() + 1;
                }
                Instruction::AdjustRelativeBase(param) => {
                    self.relative_base += self.param_value(&param);
                    self.ip += 2;
                }
                Instruction::Halt => return RunState::Halted,
            }
        }
    }

    fn decode(&self) -> Instruction {
        let instruction = self.memory[self.ip];
        let mut mode_digits =
            successors(Some(instruction.div_rem(100)), |dr| Some(dr.0.div_rem(10)));

        let (_instruction, opcode) = mode_digits.next().unwrap();

        let mut params = mode_digits
            .map(|dr| Mode::from(dr.1))
            .enumerate()
            .map(|(i, mode)| Parameter {
                value: self.memory[self.ip + i + 1],
                mode,
            });

        match opcode {
            1 => Instruction::Add(take_params(&mut params)),
            2 => Instruction::Multiply(take_params(&mut params)),
            3 => Instruction::Input(params.next().unwrap()),
            4 => Instruction::Output(params.next().unwrap()),
            5 => Instruction::JumpIfTrue(take_params(&mut params)),
            6 => Instruction::JumpIfFalse(take_params(&mut params)),
            7 => Instruction::LessThan(take_params(&mut params)),
            8 => Instruction::Equals(take_params(&mut params)),
            9 => Instruction::AdjustRelativeBase(params.next().unwrap()),
            99 => Instruction::Halt,
            unknown => panic!("Unknown opcode {unknown}"),
        }
    }

    fn param_value(&self, param: &Parameter) -> isize {
        match param.mode {
            Mode::Position => self.memory[param.value as usize],
            Mode::Immediate => param.value,
            Mode::Relative => self.memory[(param.value + self.relative_base) as usize],
        }
    }

    fn write_addr(&self, param: &Parameter) -> usize {
        match param.mode {
            Mode::Position => param.value as usize,
            Mode::Immediate => panic!("Can't write in immediate mode"),
            Mode::Relative => (param.value + self.relative_base) as usize,
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

trait DivRem: Sized {
    fn div_rem(&self, divisor: Self) -> (Self, Self);
}

impl<T: Copy + Div<Output = T> + Rem<Output = T>> DivRem for T {
    fn div_rem(&self, divisor: Self) -> (Self, Self) {
        (*self / divisor, *self % divisor)
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
    AdjustRelativeBase(Parameter),
    Halt,
}

#[derive(Debug, Clone, Copy)]
struct Parameter {
    value: isize,
    mode: Mode,
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for Mode {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            v => panic!("Unknown mode {v}"),
        }
    }
}
