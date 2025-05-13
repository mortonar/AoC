use anyhow::Result;
use std::cmp::min;
use std::env::args;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut reindeer = vec![];
    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        reindeer.push(Reindeer {
            fly_speed: tokens[3].parse()?,
            fly_duration: tokens[6].parse()?,
            rest_duration: tokens[13].parse()?,
            points: 0,
        })
    }

    let args = args();
    let total_seconds: usize = args
        .into_iter()
        .skip(1)
        .next()
        .unwrap_or("2503".to_string())
        .parse()?;

    let winner = reindeer
        .iter()
        .map(|r| r.distance(total_seconds))
        .max()
        .unwrap();
    println!("Part 1: {}", winner);

    for s in 1..=total_seconds {
        let lead_dist = reindeer.iter().map(|r| r.distance(s)).max().unwrap();
        reindeer
            .iter_mut()
            .filter(|r| r.distance(s) == lead_dist)
            .for_each(|r| r.points += 1);
    }
    let winner = reindeer.iter().map(|r| r.points).max().unwrap();
    println!("Part 2: {}", winner);

    Ok(())
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Reindeer {
    fly_speed: usize,
    fly_duration: usize,
    rest_duration: usize,
    points: usize,
}

impl Reindeer {
    fn distance(&self, total_seconds: usize) -> usize {
        let cycle_duration = self.fly_duration + self.rest_duration;
        let full_cycles = total_seconds / cycle_duration;
        let remaining_time = total_seconds % cycle_duration;
        self.fly_speed * self.fly_duration * full_cycles
            + min(remaining_time, self.fly_duration) * self.fly_speed
    }
}
