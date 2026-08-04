use anyhow::Result;
use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let diag_report = parse_input()?;

    println!("Part 1: {}", part1(&diag_report));
    println!("Part 2: {}", part2(&diag_report));

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<bool>>> {
    stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l?;
            Ok(line.chars().map(|c| c == '1').collect())
        })
        .collect()
}

fn part1(diag_report: &Vec<Vec<bool>>) -> usize {
    let counts = diag_report.bit_counts();

    let (mut gamma, mut epsilon) = (0, 0);
    for &count in counts.iter() {
        let (gamma_bit, epsilon_bit) = if count > 0 { (1, 0) } else { (0, 1) };
        gamma = (gamma << 1) | gamma_bit;
        epsilon = (epsilon << 1) | epsilon_bit;
    }

    gamma * epsilon
}

fn part2(diag_report: &[Vec<bool>]) -> usize {
    let mut oxy_candidates: HashSet<_> = (0..diag_report.len()).collect();
    let mut co2_candidates = oxy_candidates.clone();

    #[allow(clippy::needless_range_loop)]
    for bit in 0..diag_report[0].len() {
        if oxy_candidates.len() == 1 && co2_candidates.len() == 1 {
            break;
        }

        if oxy_candidates.len() > 1 {
            let counts = oxy_candidates.iter().fold(0, |counts, &oc| {
                counts + if diag_report[oc][bit] { 1 } else { -1 }
            });
            let oxy_bit = counts >= 0;
            oxy_candidates.retain(|&oc| diag_report[oc][bit] == oxy_bit);
        }

        if co2_candidates.len() > 1 {
            let counts = co2_candidates.iter().fold(0, |counts, &cc| {
                counts + if diag_report[cc][bit] { 1 } else { -1 }
            });
            let co2_bit = counts < 0;
            co2_candidates.retain(|&cc| diag_report[cc][bit] == co2_bit);
        }
    }

    let oxy = *oxy_candidates.iter().next().unwrap();
    let co2 = *co2_candidates.iter().next().unwrap();

    diag_report[oxy].to_decimal() * diag_report[co2].to_decimal()
}

trait BitStats {
    // counts[i] = positive/negative for ith bit having 1 or 0 being most common respectively
    fn bit_counts(&self) -> Vec<i64>;
}

impl BitStats for Vec<Vec<bool>> {
    fn bit_counts(&self) -> Vec<i64> {
        let mut counts = vec![0; self[0].len()];
        self.iter().for_each(|num| {
            num.iter().enumerate().for_each(|(pos, &bit)| {
                if bit {
                    counts[pos] += 1;
                } else {
                    counts[pos] -= 1;
                }
            })
        });
        counts
    }
}

trait ToDecimal {
    fn to_decimal(&self) -> usize;
}

impl ToDecimal for Vec<bool> {
    fn to_decimal(&self) -> usize {
        self.iter().fold(0, |acc, &b| (acc << 1) | b as usize)
    }
}
