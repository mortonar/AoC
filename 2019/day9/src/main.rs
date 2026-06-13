use anyhow::{Result, anyhow};
use intcode::{IntcodeComputer, Program, parse_program};

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", run_boost(&program, 1)?);
    println!("Part 2: {}", run_boost(&program, 2)?);

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn run_boost(program: &Program, input: isize) -> Result<isize> {
    let mut computer = IntcodeComputer::new(program);
    computer.input.push_back(input);
    computer.run_to_halt();
    computer.output.pop_front().ok_or(anyhow!("No output"))
}
