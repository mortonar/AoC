use anyhow::Result;
use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let groups = parse_input()?;

    println!("Part 1: {}", groups.unique_sum());
    println!("Part 2: {}", groups.common_sum());

    Ok(())
}

fn parse_input() -> Result<Vec<Group>> {
    let mut groups = Vec::new();
    let mut group = Group::default();
    for line in stdin().lock().lines() {
        let line = line?;

        if line.trim().is_empty() {
            groups.push(group);
            group = Group::default();
            continue;
        }

        let answers = line.trim().chars().collect();
        group.answers.push(answers);
    }
    groups.push(group);

    Ok(groups)
}

#[derive(Debug, Default)]
struct Group {
    /// answers[i] = person i's 'yes' answers
    answers: Vec<HashSet<char>>,
}

impl Group {
    fn unique_answers(&self) -> HashSet<char> {
        self.answers.iter().flatten().copied().collect()
    }

    fn common_answers(&self) -> HashSet<char> {
        self.answers
            .iter()
            .skip(1)
            .fold(self.answers[0].clone(), |acc, x| {
                acc.intersection(x).copied().collect()
            })
    }
}

trait Stats {
    fn unique_sum(&self) -> usize;
    fn common_sum(&self) -> usize;
}

impl Stats for Vec<Group> {
    fn unique_sum(&self) -> usize {
        self.iter().map(|g| g.unique_answers().len()).sum()
    }

    fn common_sum(&self) -> usize {
        self.iter().map(|g| g.common_answers().len()).sum()
    }
}
