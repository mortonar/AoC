use anyhow::{Result, bail};
use std::collections::HashMap;
use std::io::stdin;

const TARGET: usize = 1_000_000_000;

fn main() -> Result<()> {
    let mut land = parse_input()?;

    {
        let mut land = land.clone();
        for _ in 0..10 {
            land.magic();
        }
        println!("Part 1: {}", land.resource_val());
    }

    // After "Start" iterations, the cycle repeats every "End - Start" iterations
    // ...S...E...S...E..-....X
    let (start, end) = land.find_cycle()?;
    let period = end - start;
    // -1 since we're starting after E
    let cycles_left = (TARGET - end - 1) % period;
    for _ in 0..cycles_left {
        land.magic();
    }
    println!("Part 2: {}", land.resource_val());

    Ok(())
}

fn parse_input() -> Result<Land> {
    let acres: Vec<Vec<char>> = stdin()
        .lines()
        .map(|l| l.map(|s| s.chars().collect()))
        .collect::<Result<_, _>>()?;
    Ok(Land { acres })
}

#[derive(Debug, Clone)]
struct Land {
    acres: Vec<Vec<char>>,
}

impl Land {
    fn magic(&mut self) {
        let mut new_acres = self.acres.clone();

        for (i, row) in self.acres.iter().enumerate() {
            for (j, &acre) in row.iter().enumerate() {
                new_acres[i][j] = match acre {
                    '.' if self.count_adjacent(i, j, '|') >= 3 => '|',
                    '|' if self.count_adjacent(i, j, '#') >= 3 => '#',
                    '#' if self.count_adjacent(i, j, '#') < 1
                        || self.count_adjacent(i, j, '|') < 1 =>
                    {
                        '.'
                    }
                    other => other,
                };
            }
        }

        self.acres = new_acres;
    }

    fn count_adjacent(&self, i: usize, j: usize, target: char) -> usize {
        self.adjacent(i, j).iter().filter(|&&c| c == target).count()
    }

    fn adjacent(&self, i: usize, j: usize) -> Vec<char> {
        #[rustfmt::skip]
        const DIRS: [(isize, isize); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1) ,          (0, 1) ,
            (1, -1) , (1, 0) , (1, 1) ,
        ];

        DIRS.iter()
            .filter_map(|&(di, dj)| {
                let i = i.checked_add_signed(di)?;
                let j = j.checked_add_signed(dj)?;
                self.acres.get(i)?.get(j).copied()
            })
            .collect()
    }

    fn resource_val(&self) -> usize {
        let (wood, lumber) =
            self.acres
                .iter()
                .flatten()
                .copied()
                .fold((0, 0), |(wood, lumber), a| match a {
                    '|' => (wood + 1, lumber),
                    '#' => (wood, lumber + 1),
                    _ => (wood, lumber),
                });
        wood * lumber
    }

    fn find_cycle(&mut self) -> Result<(usize, usize)> {
        let mut seen = HashMap::new();
        for i in 0..TARGET {
            self.magic();
            if let Some(prev) = seen.insert(self.acres.clone(), i) {
                return Ok((prev, i));
            }
        }
        bail!("Could not find cycle")
    }
}
