use anyhow::Result;
use std::collections::HashMap;
use std::io::stdin;

fn main() -> Result<()> {
    let (samples, mut program) = parse_input()?;

    // frequencies[sample-op][applied-op] = # of times sample-op worked when applying it as applied-op
    let mut frequencies = [[0usize; 16]; 16];

    let part1 = samples
        .iter()
        .filter(|s| {
            let mut applied = 0;
            for (i, ins) in INSTRUCTIONS.iter().enumerate() {
                if ins.applies(s) {
                    applied += 1;
                    frequencies[s.instruction[0]][i] += 1;
                }
            }
            applied >= 3
        })
        .count();
    println!("Part 1: {part1}");

    // Greedy algo: if there's only one op that applied the most in the samples, consider it mapped
    // but set it to 0 for the rest of the frequencies to indicate it's already mapped...repeat.
    let mut mapped = HashMap::new();
    while mapped.len() < 16 {
        for sample_op in 0..frequencies.len() {
            if mapped.contains_key(&sample_op) {
                continue;
            }

            let max = *frequencies[sample_op].iter().max().unwrap();
            let num_occ = frequencies[sample_op].iter().filter(|o| **o == max).count();
            if num_occ == 1 {
                let choice_op = frequencies[sample_op]
                    .iter()
                    .position(|f| *f == max)
                    .unwrap();
                mapped.insert(sample_op, choice_op);

                for f in &mut frequencies {
                    f[choice_op] = 0;
                }
            }
        }
    }

    let mut registers = [0usize; 4];
    for i in 0..program.len() {
        let unmapped_op = program[i][0];
        program[i][0] = mapped[&unmapped_op];

        registers = INSTRUCTIONS[program[i][0]]
            .apply(&registers, &program[i])
            .unwrap();
    }
    println!("Part 2: {}", registers[0]);

    Ok(())
}

fn parse_input() -> Result<(Vec<Sample>, Vec<[usize; 4]>)> {
    let mut samples = Vec::new();
    let mut sample = Vec::new();

    let mut program = Vec::new();

    let mut prev_blank = false;
    let mut parse_program = false;

    for line in stdin().lines() {
        let line = line?;

        if !parse_program {
            if line.trim_end().is_empty() {
                if prev_blank {
                    parse_program = true;
                }
                prev_blank = true;
            } else {
                prev_blank = false;

                let arr = parse_sample_line(line.trim())?;
                sample.push(arr);
                if sample.len() == 3 {
                    samples.push(Sample {
                        before: sample[0],
                        instruction: sample[1],
                        after: sample[2],
                    });
                    sample.clear();
                }
            }
        } else if !line.trim_end().is_empty() {
            program.push(parse_sample_line(&line)?);
        }
    }

    Ok((samples, program))
}

fn parse_sample_line(line: &str) -> Result<[usize; 4]> {
    let nums: Vec<&str> = if line.starts_with("Before: [") || line.starts_with("After:  [") {
        ["Before: [", "After:  ["]
            .iter()
            .find_map(|prefix| line.strip_prefix(prefix))
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .split(", ")
            .collect()
    } else {
        line.split_ascii_whitespace().collect()
    };
    let nums: Vec<usize> = nums
        .iter()
        .map(|n| {
            n.parse()
                .map_err(|e| anyhow::anyhow!("Failed to parse '{}': {}", n, e))
        })
        .collect::<Result<Vec<_>>>()?;
    nums.try_into()
        .map_err(|v: Vec<usize>| anyhow::anyhow!("Expected 4 numbers, got {}: {:?}", v.len(), v))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani, // 13
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri, // 12
    Eqrr,
}

impl Instruction {
    fn applies(&self, sample: &Sample) -> bool {
        self.apply(&sample.before, &sample.instruction)
            .is_some_and(|out| sample.after == out)
    }

    fn apply(&self, registers: &[usize; 4], inputs: &[usize; 4]) -> Option<[usize; 4]> {
        let (_op, a_v, b_v, c_v) = (inputs[0], inputs[1], inputs[2], inputs[3]);
        let (a_r, b_r, _c_r) = (registers.get(a_v), registers.get(b_v), registers.get(c_v));

        let mut registers = *registers;

        *registers.get_mut(c_v)? = match self {
            Instruction::Addr => a_r? + b_r?,
            Instruction::Addi => a_r? + b_v,
            Instruction::Mulr => a_r? * b_r?,
            Instruction::Muli => a_r? * b_v,
            Instruction::Banr => a_r? & b_r?,
            Instruction::Bani => a_r? & b_v,
            Instruction::Borr => a_r? | b_r?,
            Instruction::Bori => a_r? | b_v,
            Instruction::Setr => *a_r?,
            Instruction::Seti => a_v,
            Instruction::Gtir => {
                if a_v > *b_r? {
                    1
                } else {
                    0
                }
            }
            Instruction::Gtri => {
                if *a_r? > b_v {
                    1
                } else {
                    0
                }
            }
            Instruction::Gtrr => {
                if *a_r? > *b_r? {
                    1
                } else {
                    0
                }
            }

            Instruction::Eqir => {
                if a_v == *b_r? {
                    1
                } else {
                    0
                }
            }
            Instruction::Eqri => {
                if *a_r? == b_v {
                    1
                } else {
                    0
                }
            }
            Instruction::Eqrr => {
                if *a_r? == *b_r? {
                    1
                } else {
                    0
                }
            }
        };

        Some(registers)
    }
}

const INSTRUCTIONS: [Instruction; 16] = [
    Instruction::Addr,
    Instruction::Addi,
    Instruction::Mulr,
    Instruction::Muli,
    Instruction::Banr,
    Instruction::Bani,
    Instruction::Borr,
    Instruction::Bori,
    Instruction::Setr,
    Instruction::Seti,
    Instruction::Gtir,
    Instruction::Gtri,
    Instruction::Gtrr,
    Instruction::Eqir,
    Instruction::Eqri,
    Instruction::Eqrr,
];
