use anyhow::{Result, bail};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::successors;
use std::ops::{Div, Rem};

fn main() -> Result<()> {
    let nic_software = parse_input()?;

    let mut network = vec![IntcodeComputer::new(&nic_software); 50];
    for (address, computer) in network.iter_mut().enumerate() {
        computer.input.push_back(address as isize);
    }
    let mut nat = Nat::default();

    'outer: loop {
        let mut outputs_produced: HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
        let mut idle_count = 0;

        for (i, computer) in network.iter_mut().enumerate() {
            match computer.run() {
                RunState::Halted => bail!("Computer {i} halted"),
                RunState::ProducedOutput => {
                    if computer.output.len() == 3 {
                        let dest = computer.output.pop_front().unwrap() as usize;
                        let x = computer.output.pop_front().unwrap();
                        let y = computer.output.pop_front().unwrap();
                        outputs_produced
                            .entry(dest)
                            .and_modify(|outputs| outputs.push((x, y)))
                            .or_insert(vec![(x, y)]);
                    }
                }
                RunState::AwaitingInput => {
                    if let Some(outputs) = outputs_produced.remove(&i) {
                        for (x, y) in outputs {
                            computer.input.push_back(x);
                            computer.input.push_back(y);
                        }
                    } else {
                        computer.input.push_back(-1);
                        idle_count += 1;
                    }
                }
            }
        }

        if idle_count == 50
            && let Some((x, y)) = nat.packet
        {
            network[0].input.push_back(x);
            network[0].input.push_back(y);

            if !nat.y_sent_to_0.insert(y) {
                println!("Part 2: {y}");
                break 'outer;
            }
        } else {
            for (dest, outputs) in outputs_produced.into_iter() {
                for (x, y) in outputs {
                    if dest == 255 {
                        if nat.packet.is_none() {
                            println!("Part 1: {y}");
                        }
                        nat.packet = Some((x, y));
                    } else {
                        let computer = &mut network[dest];
                        computer.input.push_back(x);
                        computer.input.push_back(y);
                    }
                }
            }
        }
    }

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    line.trim().split(',').map(|n| Ok(n.parse()?)).collect()
}

#[derive(Default)]
struct Nat {
    packet: Option<(isize, isize)>,
    y_sent_to_0: HashSet<isize>,
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
