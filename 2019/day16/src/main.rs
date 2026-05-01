use anyhow::{Result, anyhow};
use std::env;

fn main() -> Result<()> {
    let signal = parse_input()?;
    let phases = env::args().nth(1).unwrap_or("100".to_string()).parse()?;

    println!("Part 1: {}", signal.ftf(phases, 0).message());

    let offset = signal
        .sequence
        .iter()
        .take(7)
        .fold(0, |acc, d| acc * 10 + *d as usize);
    println!("Part 2: {}", signal.ftf(phases, offset).message());

    Ok(())
}

fn parse_input() -> Result<Signal> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let sequence = line
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10).map(|d| d as isize).ok_or_else(|| {
                anyhow!("Can't parse '{c}' as digit from input: \"{}\"", line.trim())
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Signal { sequence })
}

#[derive(Debug)]
struct Signal {
    sequence: Vec<isize>,
}

impl Signal {
    fn ftf(&self, phases: usize, offset: usize) -> Self {
        if offset == 0 {
            // Part 1: general FTF
            let mut signal = self.sequence.clone();

            for _ in 0..phases {
                signal = (0..signal.len())
                    .map(|pos| {
                        FtfPattern::new(pos + 1)
                            .zip(signal.iter())
                            .map(|(p, d)| p * d)
                            .sum::<isize>()
                            .abs()
                            % 10
                    })
                    .collect::<Vec<_>>();
            }

            Self { sequence: signal }
        } else {
            // In Part 2: the input and sample cases all have the offset in the second half of the
            // 10,000x sequence. In that case, all pattern coefficients are 0 for the first part
            // and 1 for the second - meaning we don't need to use FtfPattern, and we only care
            // about the second part of the sequence.
            let mut signal: Vec<isize> = self
                .sequence
                .iter()
                .cycle()
                .skip(offset)
                .take(self.sequence.len() * 10_000 - offset)
                .copied()
                .collect();

            for _ in 0..phases {
                let mut sum: isize = 0;
                for i in (0..signal.len()).rev() {
                    sum += signal[i];
                    signal[i] = sum.abs() % 10;
                }
            }

            Self { sequence: signal }
        }
    }

    fn message(&self) -> String {
        self.sequence
            .iter()
            .take(8)
            .fold(0, |acc, d| acc * 10 + *d as usize)
            .to_string()
    }
}

struct FtfPattern {
    base: Vec<isize>,
    repeat: usize,
    repeated: usize,
    position: usize,
}

impl FtfPattern {
    fn new(repeat: usize) -> Self {
        Self {
            base: vec![0, 1, 0, -1],
            repeat,
            // Skip first value exactly once
            repeated: 1,
            position: 0,
        }
    }
}

impl Iterator for FtfPattern {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.repeated == self.repeat {
            self.repeated = 0;
            self.position = (self.position + 1) % self.base.len();
        }

        self.repeated += 1;

        Some(self.base[self.position])
    }
}
