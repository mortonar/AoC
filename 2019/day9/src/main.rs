use anyhow::{Error, Result};
use std::collections::VecDeque;
use std::iter::successors;
use std::ops::{Div, Index, IndexMut, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", run_boost(&program, 1)?);
    println!("Part 2: {}", run_boost(&program, 2)?);

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let program: Vec<_> = line
        .trim()
        .split(",")
        .map(|n| n.parse().map_err(Error::from))
        .collect::<Result<_, _>>()?;
    Ok(program)
}

fn run_boost(program: &Program, input: isize) -> Result<isize> {
    let mut computer = IntcodeComputer::new(program, IOQueue::new(input));
    computer.run();
    computer.output.pop().ok_or(Error::msg("No output"))
}

type Program = Vec<isize>;

// Generic Queue type to enforce FIFO push/pop semantics.
#[derive(Debug, Clone, Default)]
struct IOQueue<T> {
    queue: VecDeque<T>,
}

impl<T: Default> IOQueue<T> {
    fn new(value: T) -> Self {
        Self {
            queue: VecDeque::from(vec![value]),
        }
    }

    fn push(&mut self, input: T) {
        self.queue.push_back(input);
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    ip: usize,
    relative_base: isize,
    memory: Vec<isize>,
    input: IOQueue<isize>,
    output: IOQueue<isize>,
}

impl IntcodeComputer {
    fn new(program: &Program, input: IOQueue<isize>) -> Self {
        // Allocate and zero-out extra main memory to allow writes past the program's end
        let mut memory = program.clone();
        memory.resize(program.len() * 50, 0);

        Self {
            ip: 0,
            relative_base: 0,
            memory,
            input,
            output: IOQueue::default(),
        }
    }

    // Returns true for halted, false otherwise (e.g. waiting on input)
    fn run(&mut self) -> bool {
        loop {
            match self.decode() {
                Instruction::Add(params) => {
                    let val1 = params[0].value(self);
                    let val2 = params[1].value(self);
                    let addr = params[2].write_addr(self);
                    self[addr] = val1 + val2;
                    self.ip += params.len() + 1;
                }
                Instruction::Multiply(params) => {
                    let val1 = params[0].value(self);
                    let val2 = params[1].value(self);
                    let addr = params[2].write_addr(self);
                    self[addr] = val1 * val2;
                    self.ip += params.len() + 1;
                }
                Instruction::Input(param) => {
                    if let Some(input) = self.input.pop() {
                        let addr = param.write_addr(self);
                        self[addr] = input;
                        self.ip += 2;
                    } else {
                        return false;
                    }
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
                    let val1 = params[0].value(self);
                    let val2 = params[1].value(self);
                    let addr = params[2].write_addr(self);
                    self[addr] = if val1 < val2 { 1 } else { 0 };
                    self.ip += params.len() + 1;
                }
                Instruction::Equals(params) => {
                    let val1 = params[0].value(self);
                    let val2 = params[1].value(self);
                    let addr = params[2].write_addr(self);
                    self[addr] = if val1 == val2 { 1 } else { 0 };
                    self.ip += params.len() + 1;
                }
                Instruction::AdjustRelativeBase(param) => {
                    self.relative_base += param.value(self);
                    self.ip += 2;
                }
                Instruction::Halt => {
                    return true;
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
            9 => Instruction::AdjustRelativeBase(params.next().unwrap()),
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
    AdjustRelativeBase(Parameter),
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
            Mode::Relative => computer[(self.value + computer.relative_base) as usize],
        }
    }

    fn write_addr(&self, computer: &mut IntcodeComputer) -> usize {
        match self.mode {
            Mode::Position => self.value as usize,
            Mode::Immediate => panic!("Can't write in immediate mode"),
            Mode::Relative => (self.value + computer.relative_base) as usize,
        }
    }
}

#[derive(Debug, Clone)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for Mode {
    fn from(value: isize) -> Self {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            v => panic!("Unknown mode {v}"),
        }
    }
}
