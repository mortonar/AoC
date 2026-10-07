use std::{
    collections::HashSet,
    env,
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let pairs = parse_input()?;
    let row = get_arg(1, "2000000")?;
    let limit = get_arg(2, "4000000")?;

    println!("Part 1: {}", part1(&pairs, row));
    println!("Part 2: {}", part2(&pairs, limit)?);

    Ok(())
}

fn parse_input() -> Result<Vec<Pair>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn get_arg(pos: usize, default: &str) -> Result<isize> {
    env::args()
        .nth(pos)
        .unwrap_or(default.to_string())
        .parse()
        .map_err(Error::from)
}

fn part1(pairs: &[Pair], row: isize) -> usize {
    let beacons: HashSet<_> = pairs.iter().map(|p| p.beacon).collect();

    let (min_x, max_x) = pairs
        .iter()
        .filter_map(|p| p.reach_at_row(row))
        .fold((isize::MAX, isize::MIN), |(min_x, max_x), (lo, hi)| {
            (min_x.min(lo), max_x.max(hi))
        });

    let mut beaconless = 0;
    let y = row;
    for x in min_x..=max_x {
        let pos = Coords { x, y };

        if beacons.contains(&pos) {
            continue;
        }

        for pair in pairs {
            let pair_manhattan = pair.manhattan();
            let pos_manhattan = pair.sensor.manhattan(&pos);
            if pos_manhattan <= pair_manhattan {
                beaconless += 1;
                break;
            }
        }
    }
    beaconless
}

fn part2(pairs: &[Pair], limit: isize) -> Result<isize> {
    for row in 0..=limit {
        let mut intervals: Vec<_> = pairs.iter().filter_map(|p| p.reach_at_row(row)).collect();

        intervals.sort_by_key(|&(low, _high)| low);

        let mut covered_to = -1;
        for (low, high) in intervals {
            let next = covered_to + 1;
            if low > next {
                return Ok(next * 4_000_000 + row);
            }

            covered_to = covered_to.max(high);
        }
    }

    bail!("beacon not found")
}

#[derive(Debug, Copy, Clone)]
struct Pair {
    sensor: Coords,
    beacon: Coords,
}

impl Pair {
    fn reach_at_row(&self, row: isize) -> Option<(isize, isize)> {
        let remaining = self.manhattan() as isize - self.sensor.y.abs_diff(row) as isize;
        (remaining >= 0).then(|| (self.sensor.x - remaining, self.sensor.x + remaining))
    }

    fn manhattan(&self) -> usize {
        self.sensor.manhattan(&self.beacon)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn manhattan(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split(':').collect();
        let (sensor, beacon) = (tokens[0].parse()?, tokens[1].parse()?);
        Ok(Self { sensor, beacon })
    }
}

impl FromStr for Coords {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        let (x, y) = (tokens[tokens.len() - 2], tokens[tokens.len() - 1]);
        let x = x[2..x.len() - 1].parse()?;
        let y = y[2..].parse()?;
        Ok(Self { x, y })
    }
}
