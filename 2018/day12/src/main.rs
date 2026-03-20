use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::io::stdin;

fn main() -> Result<()> {
    let mut garden = parse_input()?;
    let generations: usize = env::args().nth(1).unwrap_or("20".to_string()).parse()?;

    (0..generations).for_each(|_| garden.grow());
    println!("Part 1: {}", garden.summed_pots());

    // 50,000,000,000 is too high to compute...but after looking past 20 gens: there's a pattern.
    // After gen 117 (at 10317 plants), gen n+1 always adds 73 to the previous gen
    println!("Part 2: {}", (50_000_000_000u64 - 117) * 73 + 10317);

    Ok(())
}

fn parse_input() -> Result<Garden> {
    let mut lines = stdin().lines();
    let initial = lines.next().unwrap()?;
    let tokens: Vec<_> = initial.trim().split_ascii_whitespace().collect();
    let pots = tokens[2]
        .chars()
        .enumerate()
        .map(|(i, c)| (i as isize, c == '#'))
        .collect();

    let _blank = lines.next().unwrap()?;

    let mut rules = HashMap::new();
    for line in lines {
        let line = line?;
        let tokens: Vec<_> = line.trim().split_ascii_whitespace().collect();
        let lhs: Vec<_> = tokens[0].chars().map(|c| c == '#').collect();
        let rhs = tokens[2].starts_with('#');
        rules.insert(lhs, rhs);
    }

    Ok(Garden { pots, rules })
}

#[derive(Debug)]
struct Garden {
    // idx -> true = plant | false = no plant | no idx = no plant
    pots: HashMap<isize, bool>,
    rules: HashMap<Vec<bool>, bool>,
}

impl Garden {
    fn grow(&mut self) {
        // +/-2 to apply rules and grow the pots outward
        let (min, max) = (
            *self.pots.keys().min().unwrap() - 2,
            *self.pots.keys().max().unwrap() + 2,
        );
        let mut updates = Vec::with_capacity((max - min + 1) as usize);
        for i in min..=max {
            let pattern: Vec<_> = (i - 2..=i + 2)
                .map(|idx| *self.pots.get(&idx).unwrap_or(&false))
                .collect();
            let plant = *self.rules.get(&pattern).unwrap_or(&false);
            updates.push((i, plant));
        }
        for (i, plant) in updates {
            if plant {
                self.pots.insert(i, true);
            } else {
                self.pots.remove(&i);
            }
        }
    }

    fn summed_pots(&self) -> isize {
        self.pots
            .iter()
            .map(|(pot_num, plant)| if *plant { *pot_num } else { 0 })
            .sum()
    }
}
