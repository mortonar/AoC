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
    let gamma = diag_report.bit_majorities();
    let epsilon: Vec<_> = gamma.iter().map(|b| !b).collect();
    gamma.to_decimal() * epsilon.to_decimal()
}

fn part2(diag_report: &Vec<Vec<bool>>) -> usize {
    let oxy = diag_report.filter_to_candidate(|counts| counts >= 0);
    let co2 = diag_report.filter_to_candidate(|counts| counts < 0);
    oxy.to_decimal() * co2.to_decimal()
}

trait BitStats {
    // counts[i] = positive/negative for ith bit having 1 or 0 being most common respectively
    fn bit_majorities(&self) -> Vec<bool>;
    fn filter_to_candidate<F>(&self, count_to_bit: F) -> Vec<bool>
    where
        F: Fn(i64) -> bool;
}

impl BitStats for Vec<Vec<bool>> {
    fn bit_majorities(&self) -> Vec<bool> {
        let mut counts = vec![0; self[0].len()];
        self.iter().for_each(|num| {
            num.iter().enumerate().for_each(|(pos, &bit)| {
                counts[pos] += if bit { 1 } else { -1 };
            })
        });
        counts.iter().map(|&count| count > 0).collect()
    }

    fn filter_to_candidate<F>(&self, count_to_bit: F) -> Vec<bool>
    where
        F: Fn(i64) -> bool,
    {
        let mut candidates: HashSet<_> = (0..self.len()).collect();

        #[allow(clippy::needless_range_loop)]
        for bit in 0..self[0].len() {
            if candidates.len() == 1 {
                break;
            }

            let counts = candidates
                .iter()
                .fold(0, |counts, &c| counts + if self[c][bit] { 1 } else { -1 });
            let selected_bit = count_to_bit(counts);
            candidates.retain(|&oc| self[oc][bit] == selected_bit);
        }

        assert_eq!(candidates.len(), 1);

        let choice = *candidates.iter().next().unwrap();
        self[choice].clone()
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
