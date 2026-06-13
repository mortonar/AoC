use anyhow::Result;
use intcode::{IntcodeComputer, Program, RunState, parse_program};

fn main() -> Result<()> {
    let program = parse_input()?;

    let mut cabinet = ArcadeCabinet::new(&program);
    cabinet.run();
    let block_tiles = cabinet.screen.iter().flatten().filter(|&&x| x == 2).count();
    println!("Part 1: {block_tiles}");

    let mut cabinet = ArcadeCabinet::new(&program);
    // Free play
    cabinet.computer.memory[0] = 2;
    let score = cabinet.run();
    println!("Part 2: {score}");

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

struct ArcadeCabinet {
    screen: Vec<Vec<usize>>,
    computer: IntcodeComputer,
}

const SCREEN_WIDTH: usize = 40;

impl ArcadeCabinet {
    fn new(program: &Program) -> Self {
        Self {
            screen: vec![vec![0; SCREEN_WIDTH]; SCREEN_WIDTH],
            computer: IntcodeComputer::new(program),
        }
    }

    fn run(&mut self) -> isize {
        let mut output = Vec::new();
        let mut score = 0;
        let mut ball_x = 0;
        let mut paddle_x = 0;
        loop {
            match self.computer.run() {
                RunState::Halted => break,
                RunState::ProducedOutput => {
                    output.push(self.computer.output.pop_front().unwrap());

                    if let &[x, y, tile_id] = output.as_slice() {
                        if (-1, 0) == (x, y) {
                            score = tile_id;
                        } else {
                            let (x, y) = (x as usize, y as usize);
                            if tile_id == 3 {
                                paddle_x = x;
                            } else if tile_id == 4 {
                                ball_x = x;
                            }
                            self.screen[y][x] = tile_id as usize;
                        }

                        output.clear();
                    }
                }
                RunState::AwaitingInput => {
                    // Keep paddle under the ball (returns sign of difference: 0, -1, 1)
                    let input = (ball_x as isize - paddle_x as isize).signum();
                    self.computer.input.push_back(input);
                }
            }
        }
        score
    }
}
