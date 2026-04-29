use anyhow::{Context, Result};
use binary_search::{Direction, binary_search};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> Result<()> {
    let chem_rules = parse_input()?;

    let required = chem_rules.min_required_for("ORE", Chemical::single("FUEL"));
    println!("Part 1: {}", required.amount);

    // Assume the best we could do is 1:1 ORE -> FUEL: part 1 demonstrates this is an upper bound.
    // Use that as an upper bound on FUEL in a binary search over ORE -> FUEL to find the point
    // where we spend one trillion ORE.
    let target_ore = 1_000_000_000_000;
    let ((max_fuel, ()), _) = binary_search((0, ()), (target_ore, ()), |fuel| {
        let spent_ore = chem_rules
            .min_required_for("ORE", Chemical::new(fuel, "FUEL"))
            .amount;
        match spent_ore.cmp(&target_ore) {
            Ordering::Less => Direction::Low(()),
            _ => Direction::High(()),
        }
    });
    println!("Part 2: {max_fuel}");

    Ok(())
}

fn parse_input() -> Result<Reactions> {
    let mut rules = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let (inputs, output) = line
            .split_once(" => ")
            .context("expected ' => ' delimiter")?;
        let requires = inputs
            .split(", ")
            .map(|r| r.parse::<Chemical>())
            .collect::<Result<Vec<_>>>()?;
        let chemical = output.parse()?;
        rules.push((chemical, requires));
    }
    Ok(Reactions { rules })
}

#[derive(Debug, Clone)]
struct Reactions {
    rules: Vec<(Chemical, Vec<Chemical>)>,
}

impl Reactions {
    /// Minimum required "base" to make "target".
    fn min_required_for(&self, base: &str, target: Chemical) -> Chemical {
        let mut to_make = vec![target];
        let mut required_for = Chemical::new(0, base);
        // Since rules must be used in integer amounts, applying some rules yields extra chemicals.
        let mut leftovers: HashMap<String, usize> = HashMap::new();

        while let Some(mut target) = to_make.pop() {
            if target.name == base {
                required_for.amount += target.amount;
                continue;
            }

            if let Some(leftover) = leftovers.get_mut(&target.name) {
                let used = target.amount.min(*leftover);
                *leftover -= used;
                target.amount -= used;
            }
            leftovers.retain(|_, amount| *amount > 0);
            if target.amount == 0 {
                continue;
            }

            let (makes, made_by) = self
                .rules
                .iter()
                .find(|(output, _)| output.name == target.name)
                .unwrap();
            let times_applied = target.amount.div_ceil(makes.amount);
            let excess = makes.amount * times_applied - target.amount;
            if excess > 0 {
                *leftovers.entry(target.name.clone()).or_default() += excess;
            }

            for m in made_by {
                if let Some(existing) = to_make.iter_mut().find(|tm| tm.name == m.name) {
                    existing.amount += m.amount * times_applied;
                } else {
                    to_make.push(Chemical::new(m.amount * times_applied, &m.name));
                }
            }
        }

        required_for
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Chemical {
    amount: usize,
    name: String,
}

impl Chemical {
    fn new(amount: usize, name: &str) -> Self {
        Self {
            amount,
            name: name.to_owned(),
        }
    }

    fn single(name: &str) -> Self {
        Self::new(1, name)
    }
}

impl FromStr for Chemical {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (amount, name) = s.split_once(' ').context("expected '<amount> <name>'")?;
        Ok(Self::new(amount.parse()?, name))
    }
}
