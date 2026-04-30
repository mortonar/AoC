use anyhow::{Result, anyhow, bail};
use std::collections::{HashSet, VecDeque};
use std::iter::successors;
use std::ops::{Div, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;

    let map = map_ship_bfs(&program)?;
    println!("Part 1: {}", map.min_to_oxygen);
    println!("Part 2: {}", map.min_fill_oxygen());

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    line.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

fn map_ship_bfs(program: &Program) -> Result<ShipMap> {
    let mut queue = VecDeque::new();
    queue.push_back(SearchContext::new(program));
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut walls = HashSet::new();
    let mut oxygen_sys = None;

    while let Some(current) = queue.pop_front() {
        for [command, x, y] in [[1, -1, 0], [2, 1, 0], [3, 0, -1], [4, 0, 1]] {
            let mut neighbor = current.clone();
            neighbor.computer.input.push_back(command);
            neighbor.x += x;
            neighbor.y += y;
            neighbor.movements += 1;

            match neighbor.computer.run() {
                RunState::ProducedOutput => {
                    let output = neighbor.computer.output.pop_front().unwrap();
                    match output {
                        0 => {
                            walls.insert((neighbor.x, neighbor.y));
                        }
                        1 | 2 => {
                            if visited.insert((neighbor.x, neighbor.y)) {
                                if output == 2 {
                                    oxygen_sys =
                                        Some(((neighbor.x, neighbor.y), neighbor.movements));
                                }
                                queue.push_back(neighbor);
                            }
                        }
                        output => bail!("Unexpected output: {output}"),
                    }
                }
                run_state => bail!("Unexpected run state: {run_state:?}"),
            }
        }
    }

    oxygen_sys
        .map(|(oxygen_sys, min_to_oxygen)| ShipMap {
            open_cells: visited,
            walls,
            oxygen_sys,
            min_to_oxygen,
        })
        .ok_or_else(|| anyhow!("No solution found"))
}

#[derive(Debug, Clone)]
struct SearchContext {
    computer: IntcodeComputer,
    x: isize,
    y: isize,
    movements: usize,
}

struct ShipMap {
    open_cells: HashSet<(isize, isize)>,
    walls: HashSet<(isize, isize)>,
    oxygen_sys: (isize, isize),
    min_to_oxygen: usize,
}

impl ShipMap {
    fn min_fill_oxygen(&self) -> usize {
        let mut minutes = 0;
        let mut filled = HashSet::from([self.oxygen_sys]);

        while self.open_cells.iter().any(|c| !filled.contains(c)) {
            let new: HashSet<_> = filled
                .iter()
                .flat_map(|&(x, y)| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter())
                .filter(|n| !self.walls.contains(n))
                .collect();
            filled.extend(new.into_iter());
            minutes += 1;
        }

        minutes
    }
}

impl SearchContext {
    fn new(program: &Program) -> Self {
        Self {
            computer: IntcodeComputer::new(program),
            x: 0,
            y: 0,
            movements: 0,
        }
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
