use anyhow::{Result, anyhow};
use std::io::BufRead;

fn main() -> Result<()> {
    let mut lights = Lights::default();
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let (action, from, to) = match &tokens[0..=1] {
            &["toggle", _] => (
                Action::Toggle,
                parse_range(tokens[1])?,
                parse_range(tokens[3])?,
            ),
            &["turn", on_off] => {
                let action = if on_off == "on" {
                    Action::On
                } else {
                    Action::Off
                };
                (action, parse_range(tokens[2])?, parse_range(tokens[4])?)
            }
            _ => return Err(anyhow!("Invalid input: {}", &line)),
        };
        lights.apply_action(&action, from, to);
    }
    println!("Part 1: {}", lights.lit_count());

    Ok(())
}

fn parse_range(range: &str) -> Result<Range> {
    let tokens: Vec<&str> = range.split(",").collect();
    if tokens.len() != 2 {
        Err(anyhow!("Invalid range: {}", range))
    } else {
        Ok((tokens[0].parse()?, tokens[1].parse()?))
    }
}

#[derive(Debug)]
enum Action {
    On,
    Off,
    Toggle,
}

impl Action {
    fn apply(&self, light: &mut bool) {
        *light = match self {
            Action::On => true,
            Action::Off => false,
            Action::Toggle => !*light,
        }
    }
}

type Range = (usize, usize);

struct Lights {
    lights: Vec<Vec<bool>>,
}

impl Default for Lights {
    fn default() -> Self {
        Self {
            lights: vec![vec![false; 1000]; 1000],
        }
    }
}

impl Lights {
    fn apply_action(&mut self, action: &Action, from: Range, to: Range) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                action.apply(&mut self.lights[i][j]);
            }
        }
    }

    fn lit_count(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&l| l == true).count())
            .sum()
    }
}
