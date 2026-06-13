use anyhow::{Result, bail};
use intcode::{IntcodeComputer, Program, RunState, parse_program};

fn main() -> Result<()> {
    let program = parse_input()?;
    // Jump if any of A-C is a hole but D is ground
    let damage = run_spring(
        &program,
        &[
            "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK",
        ],
    )?;
    println!("Part 1: {damage}");

    // Jump if any of A-C is a hole but D is ground AND we can either walk on E or jump to H
    let damage = run_spring(
        &program,
        &[
            "NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T", "NOT T T",
            "OR H T", "AND T J", "RUN",
        ],
    )?;
    println!("\nPart 2: {damage}");

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn run_spring(program: &Program, spring_script: &[&str]) -> Result<isize> {
    let mut computer = IntcodeComputer::new(program);
    for line in spring_script {
        for c in line.chars() {
            computer.input.push_back(c as u8 as isize);
        }
        computer.input.push_back(b'\n' as isize);
    }

    loop {
        let state = computer.run();
        match state {
            RunState::Halted => bail!("Halted"),
            RunState::ProducedOutput => {
                let output = computer.output.pop_front().unwrap();
                match char::from_u32(output as u32) {
                    None => {
                        return Ok(output);
                    }
                    Some(char_printable) => {
                        print!("{}", char_printable);
                    }
                }
            }
            RunState::AwaitingInput => {
                dbg!("Awaiting input?");
            }
        }
    }
}
