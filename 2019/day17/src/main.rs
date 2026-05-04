use anyhow::Result;
use std::collections::VecDeque;
use std::iter::successors;
use std::ops::{Div, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;
    let map = build_map(&program);

    println!("Part 1: {}", map.alignment_param_sum());

    let mut computer = IntcodeComputer::new(&program);
    computer.memory[0] = 2;
    for line in [
        "A,B,A,C,B,A,B,C,C,B\n",
        "L,12,L,12,R,4\n",
        "R,10,R,6,R,4,R,4\n",
        "R,6,L,12,L,12\n",
        "n\n",
    ] {
        for c in line.chars() {
            computer.input.push_back(c as u8 as isize);
        }
    }

    let mut last_output = None;
    loop {
        match computer.run() {
            RunState::ProducedOutput => {
                last_output = computer.output.pop_front();
            }
            RunState::AwaitingInput => panic!("Robot requested unexpected additional input"),
            RunState::Halted => break,
        }
    }

    println!("Part 2: {}", last_output.expect("No output produced"));

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    line.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

fn build_map(program: &Program) -> ScaffoldMap {
    let mut cells = Vec::new();
    let mut row = Vec::new();
    let mut computer = IntcodeComputer::new(program);
    loop {
        match computer.run() {
            RunState::Halted => break,
            RunState::ProducedOutput => {
                let output = computer.output.pop_front().unwrap();
                if output == 10 {
                    cells.push(row);
                    row = Vec::new();
                } else {
                    row.push(output as u8 as char);
                }
            }
            RunState::AwaitingInput => panic!("Unexpectedly awaiting input"),
        }
    }

    ScaffoldMap { cells }
}

#[derive(Debug)]
struct ScaffoldMap {
    cells: Vec<Vec<char>>,
}

impl ScaffoldMap {
    fn alignment_param_sum(&self) -> usize {
        let mut intersections = 0;
        for (x, row) in self.cells.iter().enumerate() {
            for (y, _c) in row.iter().enumerate() {
                if self.is_intersection(x as isize, y as isize) {
                    intersections += x * y;
                }
            }
        }
        intersections
    }

    fn is_intersection(&self, x: isize, y: isize) -> bool {
        self.is_scaffolding(x, y)
            && [[-1, 0], [0, 1], [1, 0], [0, -1]]
                .iter()
                .all(|&[xd, yd]| self.is_scaffolding(x + xd, y + yd))
    }

    fn is_scaffolding(&self, x: isize, y: isize) -> bool {
        self.in_bounds(x, y) && self.cells[x as usize][y as usize] == '#'
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0
            && (x as usize) < self.cells.len()
            && y >= 0
            && (y as usize) < self.cells[x as usize].len()
    }
}

type Program = Vec<isize>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum RunState {
    Halted,
    ProducedOutput,
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
                    return RunState::ProducedOutput;
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
