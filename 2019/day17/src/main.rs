use anyhow::Result;
use intcode::{IntcodeComputer, Program, RunState, parse_program};

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
    parse_program(&line)
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
