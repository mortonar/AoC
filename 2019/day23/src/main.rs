use anyhow::{Result, bail};
use intcode::{IntcodeComputer, Program, RunState, parse_program};
use std::collections::{HashMap, HashSet};

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
    parse_program(&line)
}

#[derive(Default)]
struct Nat {
    packet: Option<(isize, isize)>,
    y_sent_to_0: HashSet<isize>,
}
