use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result};

fn main() -> Result<()> {
    let blueprints = parse_input()?;

    println!("Part 1: {}", part1(&blueprints));
    println!("Part 2: {}", part2(&blueprints));

    Ok(())
}

fn parse_input() -> Result<Vec<Blueprint>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|b| b.id * b.max_geodes(24, [1, 0, 0, 0], [0, 0, 0, 0]))
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|b| b.max_geodes(32, [1, 0, 0, 0], [0, 0, 0, 0]))
        .product()
}

#[derive(Debug, Default)]
struct Blueprint {
    id: usize,
    robots: [BRobot; 4],
}

#[derive(Debug, Default)]
struct BRobot {
    costs: [usize; 4],
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

impl Blueprint {
    fn max_geodes(&self, minutes: usize, robots: [usize; 4], resources: [usize; 4]) -> usize {
        let mut best = 0;
        self.search(minutes, robots, resources, [false; 4], &mut best);
        best
    }

    fn search(
        &self,
        minutes: usize,
        robots: [usize; 4],
        resources: [usize; 4],
        skip: [bool; 4],
        best: &mut usize,
    ) {
        if minutes == 0 {
            *best = (*best).max(resources[GEODE]);
            return;
        }

        // Pruning - even building a geode robot every remaining minute can't beat the best found so far
        let upper_bound = resources[GEODE] + robots[GEODE] * minutes + minutes * (minutes - 1) / 2;
        if upper_bound <= *best {
            return;
        }

        let mut new_skip = [false; 4];

        for (resource, to_buy) in self.robots.iter().enumerate() {
            if resources
                .iter()
                .zip(to_buy.costs.iter())
                .any(|(&h, &c)| h < c)
            {
                continue;
            }

            // Pruning - there's no point in building more robots than what the max cost of this
            // resource would be, except geode robots since there's no such thing as too many of those
            if resource != GEODE
                && robots[resource] == self.robots.iter().map(|r| r.costs[resource]).max().unwrap()
            {
                continue;
            }

            new_skip[resource] = true;

            // Pruning - if we could already afford this robot last minute and chose to wait instead,
            // building it now is dominated by having built it then
            if skip[resource] {
                continue;
            }

            let mut new_robots = robots;
            new_robots[resource] += 1;

            let mut new_resources = resources;
            for (j, &r) in to_buy.costs.iter().enumerate() {
                new_resources[j] -= r;
            }

            for (j, &r) in robots.iter().enumerate() {
                new_resources[j] += r;
            }

            self.search(minutes - 1, new_robots, new_resources, [false; 4], best);
        }

        let mut new_resources = resources;
        for (i, &r) in robots.iter().enumerate() {
            new_resources[i] += r;
        }
        self.search(minutes - 1, robots, new_resources, new_skip, best);
    }
}

impl FromStr for Blueprint {
    type Err = Error;

    #[allow(clippy::field_reassign_with_default)]
    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();

        let mut blueprint = Self::default();
        let id = tokens[1];
        blueprint.id = id[0..id.len() - 1].parse()?;
        blueprint.robots[ORE].costs[ORE] = tokens[6].parse()?;
        blueprint.robots[CLAY].costs[ORE] = tokens[12].parse()?;
        blueprint.robots[OBSIDIAN].costs[ORE] = tokens[18].parse()?;
        blueprint.robots[OBSIDIAN].costs[CLAY] = tokens[21].parse()?;
        blueprint.robots[GEODE].costs[ORE] = tokens[27].parse()?;
        blueprint.robots[GEODE].costs[OBSIDIAN] = tokens[30].parse()?;
        Ok(blueprint)
    }
}
