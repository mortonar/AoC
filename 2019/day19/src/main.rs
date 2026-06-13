use anyhow::Result;
use intcode::{IntcodeComputer, Program, RunState, parse_program};
use std::env;

fn main() -> Result<()> {
    let program = parse_input()?;
    let (grid, ship) = parse_args()?;

    println!("Part 1: {}", part1(&program, grid));
    println!("Part 2: {}", part2(&program, ship));

    Ok(())
}

fn part1(program: &Program, grid: usize) -> usize {
    (0..grid)
        .flat_map(|y| (0..grid).map(move |x| (x, y)))
        .filter(|&(x, y)| is_affected(program, x, y))
        .count()
}

fn part2(program: &Program, ship: usize) -> usize {
    let mut y = ship - 1;
    let mut x = 0usize;

    loop {
        while !is_affected(program, x, y) {
            x += 1;
        }

        let top_y = y + 1 - ship;
        let right_x = x + ship - 1;

        if is_affected(program, right_x, top_y) {
            return x * 10_000 + top_y;
        }

        y += 1;
    }
}

fn is_affected(program: &Program, x: usize, y: usize) -> bool {
    let mut computer = IntcodeComputer::new(program);
    computer.input.push_back(x as isize);
    computer.input.push_back(y as isize);

    match computer.run() {
        RunState::ProducedOutput => computer.output.pop_front() == Some(1),
        state => panic!("Unexpected state {:?}", state),
    }
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn parse_args() -> Result<(usize, usize)> {
    let grid = env::args().nth(1).unwrap_or("50".to_string()).parse()?;
    let ship = env::args().nth(2).unwrap_or("100".to_string()).parse()?;
    Ok((grid, ship))
}
