use anyhow::{Error, Result, anyhow, bail};
use std::collections::HashSet;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let mut particles = parse_input()?;
    let mut particles2 = particles.clone();

    simulate(&mut particles, false);
    let (idx, _closest) = particles
        .iter()
        .enumerate()
        .min_by(|(_i, p1), (_j, p2)| p1.dist_from_origin().cmp(&p2.dist_from_origin()))
        .ok_or_else(|| anyhow!("Minimum dist not found"))?;
    println!("Part 1: {idx}");

    simulate(&mut particles2, true);
    println!("Part 2: {}", particles2.len());

    Ok(())
}

fn simulate(particles: &mut Vec<Particle>, collisions: bool) {
    for _ in 0..10_000 {
        if collisions {
            let mut positions = HashSet::new();
            let mut dupes = HashSet::new();

            for p in particles.iter() {
                if !positions.insert(p.position) {
                    dupes.insert(p.position);
                }
            }

            particles.retain(|p| !dupes.contains(&p.position));
        }

        particles.iter_mut().for_each(Particle::update);
    }
}

fn parse_input() -> Result<Vec<Particle>> {
    stdin()
        .lines()
        .map(|l| l?.parse::<Particle>())
        .collect::<Result<Vec<_>, Error>>()
}

#[derive(Clone, Debug)]
struct Particle {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
    acceleration: (isize, isize, isize),
}

impl Particle {
    fn update(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn dist_from_origin(&self) -> usize {
        self.position.0.unsigned_abs()
            + self.position.1.unsigned_abs()
            + self.position.2.unsigned_abs()
    }
}

impl FromStr for Particle {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        Ok(Particle {
            position: parse_vector(tokens[0])?,
            velocity: parse_vector(tokens[1])?,
            acceleration: parse_vector(tokens[2])?,
        })
    }
}

fn parse_vector(s: &str) -> Result<(isize, isize, isize)> {
    let left = s.find('<').ok_or_else(|| anyhow!("< not found"))?;
    let right = s.find('>').ok_or_else(|| anyhow!("> not found"))?;
    let vals: Vec<_> = s[(left + 1)..right].split(',').collect();
    if vals.len() != 3 {
        bail!("Expected exactly 3 values but got {}", vals.len());
    }
    let parsed: Vec<isize> = vals
        .iter()
        .map(|v| v.trim().parse::<isize>().map_err(|e| anyhow!(e)))
        .collect::<Result<_, _>>()?;
    Ok((parsed[0], parsed[1], parsed[2]))
}
