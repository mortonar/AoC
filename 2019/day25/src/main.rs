use anyhow::{Result, bail};
use std::collections::{HashSet, VecDeque};
use std::iter::successors;
use std::ops::{Div, Rem};

fn main() -> Result<()> {
    let program = parse_input()?;
    solve(&program)?;
    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    line.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

fn solve(program: &Program) -> Result<()> {
    let mut droid = IntcodeComputer::new(program);

    let mut initial_output = String::new();
    loop {
        match droid.run() {
            RunState::Halted => break,
            RunState::ProducedOutput => {
                initial_output.push(droid.output.pop_front().unwrap() as u8 as char);
            }
            RunState::AwaitingInput => break,
        }
    }

    let mut visited: HashSet<String> = HashSet::new();
    let mut collected_items: Vec<String> = Vec::new();
    let mut checkpoint_path: Vec<String> = Vec::new();
    let mut security_dir: Option<String> = None;
    let mut path = Vec::new();
    explore(
        &mut droid,
        &initial_output,
        &mut path,
        &mut visited,
        &mut collected_items,
        &mut checkpoint_path,
        &mut security_dir,
    );

    for dir in &checkpoint_path {
        send_command(&mut droid, dir);
    }

    let security_dir = security_dir.unwrap_or_else(|| {
        for dir in &["north", "south", "east", "west"] {
            let test_output = send_command(&mut droid, dir);
            if test_output.contains("Pressure-Sensitive Floor") {
                return dir.to_string();
            }
            let (room, _, _) = parse_room(&test_output);
            if !room.is_empty() && room != "Security Checkpoint" {
                send_command(&mut droid, opposite_dir(dir));
            }
        }
        "north".to_string()
    });

    let n = collected_items.len();

    for item in &collected_items {
        send_command(&mut droid, &format!("drop {}", item));
    }

    for mask in 0..(1 << n) {
        let mut current_items: Vec<&String> = Vec::new();
        for (i, item) in collected_items.iter().enumerate() {
            if mask & (1 << i) != 0 {
                send_command(&mut droid, &format!("take {}", item));
                current_items.push(item);
            }
        }

        let result = send_command(&mut droid, &security_dir);

        if !result.contains("lighter") && !result.contains("heavier") {
            println!("{}", result);
            return Ok(());
        }

        for item in &current_items {
            send_command(&mut droid, &format!("drop {}", item));
        }
    }

    bail!("Could not find correct item combination")
}

fn explore(
    droid: &mut IntcodeComputer,
    current_output: &str,
    path: &mut Vec<String>,
    visited: &mut HashSet<String>,
    collected_items: &mut Vec<String>,
    checkpoint_path: &mut Vec<String>,
    security_dir: &mut Option<String>,
) {
    let (room_name, doors, items) = parse_room(current_output);

    if room_name.is_empty() || visited.contains(&room_name) {
        return;
    }
    visited.insert(room_name.clone());

    for item in &items {
        if !DANGEROUS_ITEMS.contains(&item.as_str()) {
            let cmd = format!("take {}", item);
            send_command(droid, &cmd);
            collected_items.push(item.clone());
        }
    }

    if room_name == "Security Checkpoint" {
        *checkpoint_path = path.clone();
        for door in &doors {
            if door != opposite_dir(path.last().unwrap_or(&String::new())) {
                let test_output = send_command(droid, door);
                if test_output.contains("Pressure-Sensitive Floor")
                    || test_output.contains("Security Checkpoint")
                {
                    *security_dir = Some(door.clone());
                }
            }
        }
        return;
    }

    for door in &doors {
        path.push(door.clone());
        let new_output = send_command(droid, door);

        let (new_room, _, _) = parse_room(&new_output);
        if !new_room.is_empty() && !visited.contains(&new_room) {
            explore(
                droid,
                &new_output,
                path,
                visited,
                collected_items,
                checkpoint_path,
                security_dir,
            );

            send_command(droid, opposite_dir(door));
        } else if !new_room.is_empty() && new_room != room_name {
            send_command(droid, opposite_dir(door));
        }
        path.pop();
    }
}

const DANGEROUS_ITEMS: &[&str] = &[
    "giant electromagnet",
    "infinite loop",
    "molten lava",
    "photons",
    "escape pod",
];

fn send_command(droid: &mut IntcodeComputer, command: &str) -> String {
    for c in command.chars() {
        droid.input.push_back(c as isize);
    }
    droid.input.push_back('\n' as isize);

    let mut output = String::new();
    loop {
        match droid.run() {
            RunState::Halted => break,
            RunState::ProducedOutput => {
                output.push(droid.output.pop_front().unwrap() as u8 as char);
            }
            RunState::AwaitingInput => break,
        }
    }
    output
}

fn parse_room(output: &str) -> (String, Vec<String>, Vec<String>) {
    let mut room_name = String::new();
    let mut doors = Vec::new();
    let mut items = Vec::new();

    let mut in_doors = false;
    let mut in_items = false;

    for line in output.lines() {
        if line.starts_with("== ") && line.ends_with(" ==") {
            room_name = line[3..line.len() - 3].to_string();
            in_doors = false;
            in_items = false;
        } else if line == "Doors here lead:" {
            in_doors = true;
            in_items = false;
        } else if line == "Items here:" {
            in_doors = false;
            in_items = true;
        } else if let Some(stripped) = line.strip_prefix("- ") {
            let item = stripped.to_string();
            if in_doors {
                doors.push(item);
            } else if in_items {
                items.push(item);
            }
        } else if line.is_empty() {
            in_doors = false;
            in_items = false;
        }
    }

    (room_name, doors, items)
}

fn opposite_dir(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "east" => "west",
        "west" => "east",
        _ => panic!("Unknown direction"),
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
