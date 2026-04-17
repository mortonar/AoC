use anyhow::{Error, Result};
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::successors;
use std::ops::{Div, Index, IndexMut, Range, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", max_signal(&program, 0..5, false));
    println!("Part 2: {}", max_signal(&program, 5..10, true));

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

fn max_signal(program: &Program, phase_range: Range<isize>, feedback: bool) -> isize {
    // Grab this first since permutations() consumes self
    let k = phase_range.len();
    phase_range
        .permutations(k)
        .map(|phase_sequence| run_chain(program, 0, &phase_sequence, feedback))
        .max()
        .unwrap()
}

fn run_chain(program: &Program, input: isize, phase_sequence: &[isize], feedback: bool) -> isize {
    let mut amp_chain: Vec<_> = phase_sequence
        .iter()
        .map(|&phase_setting| {
            IntcodeComputer::new(program, IOQueue::new(VecDeque::from([phase_setting])))
        })
        .collect();

    amp_chain[0].input.push(input);

    let mut i = 0;
    let mut halt_count = 0;
    let mut sent_to_thrusters = 0;
    loop {
        let halted = amp_chain[i].run();
        if halted {
            halt_count += 1;
        }

        if let Some(output) = amp_chain[i].output.pop() {
            if i != amp_chain.len() - 1 {
                amp_chain[i + 1].input.push(output);
            } else {
                sent_to_thrusters = output;

                if feedback {
                    amp_chain[0].input.push(output);
                } else {
                    break;
                }
            }
        }

        if halt_count == amp_chain.len() {
            break;
        }

        i += 1;
        if i == amp_chain.len() {
            i = 0;
            halt_count = 0;
        }
    }

    sent_to_thrusters
}

type Program = Vec<isize>;

// Generic Queue type to enforce FIFO push/pop semantics.
#[derive(Debug, Clone, Default)]
struct IOQueue<T> {
    queue: VecDeque<T>,
}

impl<T> IOQueue<T> {
    fn new(queue: VecDeque<T>) -> Self {
        Self { queue }
    }

    fn push(&mut self, input: T) {
        self.queue.push_front(input);
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_back()
    }
}

#[derive(Debug, Clone)]
struct IntcodeComputer {
    ip: usize,
    memory: Vec<isize>,
    input: IOQueue<isize>,
    output: IOQueue<isize>,
}

impl IntcodeComputer {
    fn new(program: &Program, input: IOQueue<isize>) -> Self {
        Self {
            ip: 0,
            memory: program.clone(),
            input,
            output: IOQueue::default(),
        }
    }

    // Returns true for halted, false otherwise (e.g. waiting on input)
    fn run(&mut self) -> bool {
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
                    if let Some(input) = self.input.pop() {
                        self[param.value as usize] = input;
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
