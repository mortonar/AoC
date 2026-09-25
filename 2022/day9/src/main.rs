use std::{
    collections::HashSet,
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let motions = parse_input()?;

    println!("Part 1: {}", part1(&motions));
    println!("Part 2: {}", part2(&motions));

    Ok(())
}

fn parse_input() -> Result<Vec<Motion>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(motions: &[Motion]) -> usize {
    simulate(motions, 2)
}

fn part2(motions: &[Motion]) -> usize {
    simulate(motions, 10)
}

fn simulate(motions: &[Motion], knots: usize) -> usize {
    // knots[0] is head, knots.last() is tail
    let mut knots = vec![(0isize, 0isize); knots];

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for Motion { steps, dir } in motions {
        let &&(dx, dy) = dir;

        for _ in 0..*steps {
            let (hx, hy) = &mut knots[0];
            *hx += dx;
            *hy += dy;

            for i in 0..knots.len() - 1 {
                let (hx, hy) = knots[i];
                let (tx, ty) = &mut knots[i + 1];

                if (hx, hy).dist(&(*tx, *ty)) == 2 {
                    *tx += (hx - *tx).signum();
                    *ty += (hy - *ty).signum();
                }
            }

            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}

const UP: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (0, -1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

#[derive(Debug)]
struct Motion {
    steps: usize,
    dir: &'static (isize, isize),
}

impl FromStr for Motion {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let Some((dir, steps)) = s.split_once(' ') else {
            bail!("bad motion format: {s}");
        };

        let dir = match dir {
            "R" => &RIGHT,
            "L" => &LEFT,
            "U" => &UP,
            "D" => &DOWN,
            _ => bail!("unrecognized direction: {dir}"),
        };

        let steps = steps.parse()?;

        Ok(Motion { steps, dir })
    }
}

trait EuclidDist {
    fn dist(&self, other: &Self) -> usize;
}

impl EuclidDist for (isize, isize) {
    fn dist(&self, other: &Self) -> usize {
        let (x1, y1) = *self;
        let (x2, y2) = *other;
        (x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2)).isqrt()
    }
}
