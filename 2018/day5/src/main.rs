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
        let mut units = self.units.clone();

        loop {
            let mut i = 0;
            let mut reaction = false;
            while units.len() > 1 && i < units.len() - 1 {
                if self.do_react(units[i], units[i + 1]) {
                    units.remove(i);
                    units.remove(i);
                    reaction = true;
                } else {
                    i += 1;
                }
            }
            if !reaction {
                break;
            }
        }

        Self { units }
    }

    fn len(&self) -> usize {
        self.units.len()
    }

    fn do_react(&self, c1: char, c2: char) -> bool {
        c1.eq_ignore_ascii_case(&c2) && c1.is_ascii_lowercase() != c2.is_ascii_lowercase()
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
