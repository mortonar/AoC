use anyhow::Result;
use intcode::{IntcodeComputer, Program, parse_program};
use std::env;

fn main() -> Result<()> {
    let program = parse_input()?;

    let diagnostic = run(&program, 1);
    println!("Part 1: {diagnostic}");

    let input = env::args().nth(1).unwrap_or(String::from("5")).parse()?;
    let diagnostic = run(&program, input);
    println!("Part 2: {diagnostic}");

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn run(program: &Program, input: isize) -> isize {
    let mut computer = IntcodeComputer::new(program);
    computer.input.push_back(input);
    computer.run_to_halt();
    computer.output.pop_back().unwrap()
}
