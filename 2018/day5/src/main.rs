use anyhow::Result;
use std::collections::HashSet;
use std::io::stdin;

fn main() -> Result<()> {
    let polymer = parse_input()?;

    println!("Part 1: {}", polymer.fully_react().len());

    let shortest = polymer
        .unique_units()
        .iter()
        .map(|&r| polymer.removed(r).fully_react().len())
        .min()
        .unwrap();
    println!("Part 2: {shortest}");

    Ok(())
}

fn parse_input() -> Result<Polymer> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(Polymer {
        units: input.trim().chars().collect(),
    })
}

struct Polymer {
    units: Vec<char>,
}

impl Polymer {
    /// Return the fully reacted version of this Polymer
    fn fully_react(&self) -> Self {
        let mut stack = Vec::new();

        // "Collapse" from left to right with a stack so we only need one pass
        self.units.iter().fold(&mut stack, |stack, &u| {
            // In ASCII table, lower and upper case versions of each letter have diff of 32
            if stack.is_empty() || (*stack.last().unwrap() as u32) ^ (u as u32) != 32 {
                stack.push(u);
            } else {
                stack.pop();
            }
            stack
        });

        Self { units: stack }
    }

    fn len(&self) -> usize {
        self.units.len()
    }

    fn unique_units(&self) -> HashSet<char> {
        self.units.iter().map(|u| u.to_ascii_lowercase()).collect()
    }

    /// Return the version of this polymer with the given unit removed
    fn removed(&self, unit: char) -> Self {
        let mut units = self.units.clone();
        units.retain(|u| !u.eq_ignore_ascii_case(&unit));
        Self { units }
    }
}
