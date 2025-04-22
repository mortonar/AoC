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
                } else if on_off == "off" {
                    Action::Off
                } else {
                    return Err(anyhow!("Unknown action: {on_off}"));
                };
                (action, parse_range(tokens[2])?, parse_range(tokens[4])?)
            }
            _ => return Err(anyhow!("Invalid input: {}", &line)),
        };
        lights.apply_action(&action, from, to);
    }
    println!("Part 1: {}", lights.lit_count());
    println!("Part 2: {}", lights.total_brightness());

    Ok(())
}

fn parse_range(range: &str) -> Result<Point> {
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
    fn apply(&self, light: &mut Light) {
        match self {
            Action::On => {
                light.0 = true;
                light.1 += 1;
            }
            Action::Off => {
                light.0 = true;
                if light.1 != 0 {
                    light.1 -= 1;
                };
            }
            Action::Toggle => {
                light.0 = !light.0;
                light.1 += 2;
            }
        }
    }
}

type Point = (usize, usize);

type Light = (bool, usize);

struct Lights {
    lights: Vec<Vec<Light>>,
}

impl Default for Lights {
    fn default() -> Self {
        Self {
            lights: vec![vec![(false, 0); 1000]; 1000],
        }
    }
}

impl Lights {
    fn apply_action(&mut self, action: &Action, from: Point, to: Point) {
        for i in from.0..=to.0 {
            for j in from.1..=to.1 {
                action.apply(&mut self.lights[i][j]);
            }
        }
    }

    fn lit_count(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().filter(|&&l| l.0 == true).count())
            .sum()
    }

    fn total_brightness(&self) -> usize {
        self.lights
            .iter()
            .map(|row| row.iter().map(|&l| l.1).sum::<usize>())
            .sum()
    }
}
