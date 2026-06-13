use anyhow::Result;
use intcode::{IntcodeComputer, Program, RunState, parse_program};
use itertools::Itertools;
use std::ops::Range;

fn main() -> Result<()> {
    let program = parse_input()?;

    println!("Part 1: {}", max_signal(&program, 0..5, false));
    println!("Part 2: {}", max_signal(&program, 5..10, true));

    Ok(())
}

fn parse_input() -> Result<Program> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    parse_program(&line)
}

fn max_signal(program: &Program, phase_range: Range<isize>, feedback: bool) -> isize {
    // Grab this first since permutations() consumes self
    let k = phase_range.len();
    phase_range
        .permutations(k)
        .map(|phase_sequence| run_chain(program, 0, &phase_sequence, feedback))
        .max()
        .unwrap()
}

fn run_chain(program: &Program, input: isize, phase_sequence: &[isize], feedback: bool) -> isize {
    let mut amp_chain: Vec<_> = phase_sequence
        .iter()
        .map(|&phase_setting| {
            let mut computer = IntcodeComputer::new(program);
            computer.input.push_back(phase_setting);
            computer
        })
        .collect();

    amp_chain[0].input.push_back(input);

    let mut i = 0;
    let mut halt_count = 0;
    let mut sent_to_thrusters = 0;
    loop {
        let state = amp_chain[i].run();
        let halted = state == RunState::Halted;
        if halted {
            halt_count += 1;
        }

        if let Some(output) = amp_chain[i].output.pop_front() {
            if i != amp_chain.len() - 1 {
                amp_chain[i + 1].input.push_back(output);
            } else {
                sent_to_thrusters = output;

                if feedback {
                    amp_chain[0].input.push_back(output);
                } else {
                    break;
                }
            }
        }

        if halt_count == amp_chain.len() {
            break;
        }

        i += 1;
        if i == amp_chain.len() {
            i = 0;
            halt_count = 0;
        }
    }

    sent_to_thrusters
}
