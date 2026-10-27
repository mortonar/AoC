use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, stdin},
};

use anyhow::Result;

fn main() -> Result<()> {
    let elves = parse_input()?;

    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));

    Ok(())
}

fn parse_input() -> Result<Vec<Elf>> {
    let mut elves = Vec::new();
    for (x, line) in stdin().lock().lines().enumerate() {
        for (y, c) in line?.chars().enumerate() {
            if c == '#' {
                let (x, y) = (x as isize, y as isize);
                elves.push(Elf { x, y });
            }
        }
    }
    Ok(elves)
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Elf {
    x: isize,
    y: isize,
}

fn part1(elves: &[Elf]) -> usize {
    let mut elves: HashSet<_> = elves.iter().cloned().collect();

    for round in 0..10 {
        elves = run_round(&elves, round);
    }

    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
    for &Elf { x, y } in elves.iter() {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let mut empties = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !elves.contains(&Elf { x, y }) {
                empties += 1;
            }
        }
    }
    empties
}

fn part2(elves: &[Elf]) -> usize {
    let mut elves: HashSet<_> = elves.iter().cloned().collect();

    for round in 0.. {
        let new_elves = run_round(&elves, round);
        if new_elves == elves {
            return round + 1;
        }
        elves = new_elves;
    }

    panic!("no settling round found")
}

const N: (isize, isize) = (-1, 0);
const NE: (isize, isize) = (-1, 1);
const NW: (isize, isize) = (-1, -1);
const S: (isize, isize) = (1, 0);
const SE: (isize, isize) = (1, 1);
const SW: (isize, isize) = (1, -1);
const E: (isize, isize) = (0, 1);
const W: (isize, isize) = (0, -1);

#[allow(clippy::type_complexity)]
const CHECKS: [([(isize, isize); 3], (isize, isize)); 4] = [
    ([N, NE, NW], N),
    ([S, SE, SW], S),
    ([W, NW, SW], W),
    ([E, NE, SE], E),
];

fn run_round(elves: &HashSet<Elf>, round: usize) -> HashSet<Elf> {
    // Proposed elf position to original position of elves considering this position
    let mut proposed: HashMap<Elf, Vec<&Elf>> = HashMap::new();

    for e in elves.iter() {
        let occupied = |(dx, dy): (isize, isize)| {
            elves.contains(&Elf {
                x: e.x + dx,
                y: e.y + dy,
            })
        };

        let alone = [N, NE, NW, S, SE, SW, E, W]
            .into_iter()
            .all(|d| !occupied(d));

        let prop = if alone {
            None
        } else {
            (0..4)
                .map(|i| CHECKS[(round + i) % 4])
                .find(|(dirs, _)| dirs.iter().all(|&d| !occupied(d)))
                .map(|(_, (mx, my))| Elf {
                    x: e.x + mx,
                    y: e.y + my,
                })
        };

        let (prop, orig) = (prop.unwrap_or_else(|| e.clone()), e);

        proposed
            .entry(prop)
            .and_modify(|list| list.push(orig))
            .or_insert(vec![orig]);
    }

    proposed
        .into_iter()
        .flat_map(|(prop, orig_elves)| {
            if orig_elves.len() == 1 {
                vec![prop]
            } else {
                orig_elves.into_iter().cloned().collect()
            }
        })
        .collect()
}
