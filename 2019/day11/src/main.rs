use anyhow::Result;
use intcode::{IntcodeComputer, Program, RunState, parse_program};

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
    parse_program(&line)
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
