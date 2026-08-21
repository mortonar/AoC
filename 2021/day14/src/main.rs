use anyhow::Result;
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (template, rules) = parse_input()?;

    println!("Part 1: {}", part1(&template, &rules));
    println!("Part 2: {}", part2(&template, &rules));

    Ok(())
}

type Input = (Vec<char>, HashMap<[char; 2], char>);

fn parse_input() -> Result<Input> {
    let mut lines = stdin().lock().lines();

    let template = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing template"))??;
    let template = template.chars().collect();
    let _blank = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing blank line"))??;

    let mut rules = HashMap::new();
    for line in lines.by_ref() {
        let line = line?;
        let (from, to) = line
            .split_once(" -> ")
            .ok_or_else(|| anyhow::anyhow!("Invalid rule: {line}"))?;

        let from_chars: Vec<char> = from.chars().collect();
        if from_chars.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid rule lhs (expected 2 chars): {line}"
            ));
        }

        let to_chars: Vec<char> = to.chars().collect();
        if to_chars.len() != 1 {
            return Err(anyhow::anyhow!(
                "Invalid rule rhs (expected 1 char): {line}"
            ));
        }

        rules.insert([from_chars[0], from_chars[1]], to_chars[0]);
    }

    Ok((template, rules))
}

fn part1(template: &[char], rules: &HashMap<[char; 2], char>) -> usize {
    poly_steps(template, rules, 10)
}

fn part2(template: &[char], rules: &HashMap<[char; 2], char>) -> usize {
    poly_steps(template, rules, 40)
}

fn poly_steps(template: &[char], rules: &HashMap<[char; 2], char>, steps: usize) -> usize {
    let mut counts = template.counts();
    // Expanding character pair for remaining steps produces character count
    let mut memo: HashMap<([char; 2], usize), HashMap<char, usize>> = HashMap::new();

    template
        .windows(2)
        .for_each(|w| counts.accum(&poly_steps_rec([w[0], w[1]], rules, steps, &mut memo)));

    let (min, max) = counts
        .values()
        .fold((usize::MAX, usize::MIN), |(min, max), &v| {
            (min.min(v), max.max(v))
        });

    max - min
}

fn poly_steps_rec(
    pair: [char; 2],
    rules: &HashMap<[char; 2], char>,
    steps: usize,
    memo: &mut HashMap<([char; 2], usize), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    if steps == 0 {
        return HashMap::new();
    }

    if let Some(cached) = memo.get(&(pair, steps)) {
        return cached.clone();
    }

    let mid = rules[&pair];
    let mut counts: HashMap<char, usize> = HashMap::from([(mid, 1)]);
    counts.accum(&poly_steps_rec([pair[0], mid], rules, steps - 1, memo));
    counts.accum(&poly_steps_rec([mid, pair[1]], rules, steps - 1, memo));
    memo.insert((pair, steps), counts.clone());

    counts
}

trait CharCounts {
    fn counts(&self) -> HashMap<char, usize>;
}

impl CharCounts for &[char] {
    fn counts(&self) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        self.iter()
            .for_each(|&c| *counts.entry(c).or_default() += 1);
        counts
    }
}

trait Accum {
    fn accum(&mut self, other: &HashMap<char, usize>);
}

impl Accum for HashMap<char, usize> {
    fn accum(&mut self, other: &HashMap<char, usize>) {
        for (k, v) in other {
            *self.entry(*k).or_default() += v;
        }
    }
}
