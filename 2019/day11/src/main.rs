use anyhow::Result;
use std::collections::VecDeque;
use std::iter::successors;
use std::ops::{Div, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;

    let panels = paint_hull(&program, 0);
    let panels_painted = panels.iter().flatten().filter(|p| p.painted).count();
    println!("Part 1: {panels_painted}");

    let panels = paint_hull(&program, 1);
    println!("Part 2: HJALJZFH");
    for row in panels.iter() {
        // Trim the output to only rows containing white since message is white-on-black
        if !row.iter().any(|p| p.color == 1) {
            continue;
        }

        for p in row {
            let c = match p.color {
                1 => '#',
                _ => '.',
            };
            print!("{c}");
        }
        println!();
    }

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    line.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

// Adjust as needed :)
const GRID_LENGTH: usize = 150;

fn paint_hull(program: &Program, start_color: isize) -> Vec<Vec<Panel>> {
    let mut grid = vec![vec![Panel::default(); GRID_LENGTH]; GRID_LENGTH];
    let mut robot = Robot {
        computer: IntcodeComputer::new(program),
        direction: Direction::Up,
        x: GRID_LENGTH / 2,
        y: GRID_LENGTH / 2,
    };
    grid[robot.x][robot.y].color = start_color;

    let mut output = Vec::new();
    loop {
        match robot.computer.run() {
            RunState::Halted => break,
            RunState::ProducedOutput => {
                output.push(robot.computer.output.pop_front().unwrap());

                if let &[color, turn] = output.as_slice() {
                    let panel = &mut grid[robot.x][robot.y];
                    panel.color = color;
                    panel.painted = true;

                    robot.apply_movement(turn);

                    output.clear();
                }
            }
            RunState::AwaitingInput => robot.computer.input.push_back(grid[robot.x][robot.y].color),
        }
    }

    grid
}

struct Robot {
    computer: IntcodeComputer,
    direction: Direction,
    x: usize,
    y: usize,
}

impl Robot {
    fn apply_movement(&mut self, turn: isize) {
        self.direction = self.direction.turn(turn);

        let (dx, dy) = self.direction.diff();
        let (x, y) = (self.x as isize + dx, self.y as isize + dy);
        if x < 0 || x > GRID_LENGTH as isize || y < 0 || y > GRID_LENGTH as isize {
            panic!("Out of bounds - adjust GRID_LENGTH");
        }
        self.x = x as usize;
        self.y = y as usize;
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, value: isize) -> Self {
        match self {
            Direction::Up => {
                if value == 0 {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Down => {
                if value == 0 {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Left => {
                if value == 0 {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Right => {
                if value == 0 {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }

    fn diff(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Panel {
    painted: bool,
    color: isize,
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
