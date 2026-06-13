use anyhow::Result;
use intcode::{IntcodeComputer, Program, parse_program};

fn main() -> Result<()> {
    let program = parse_input()?;

    let p1 = run_with_inputs(&program, 12, 2);
    println!("Part 1: {}", p1);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let result = run_with_inputs(&program, noun, verb);
            if result == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn run_with_inputs(program: &Program, noun: isize, verb: isize) -> isize {
    let mut computer = IntcodeComputer::new(program);
    computer.memory[1] = noun;
    computer.memory[2] = verb;
    computer.run_to_halt();
    computer.memory[0]
}
