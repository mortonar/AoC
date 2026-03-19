use anyhow::Result;
use std::cmp::Ordering;
use std::env;
use std::fmt::{Display, Formatter};

const MAX: usize = 300;

fn main() -> Result<()> {
    let serial: isize = env::args().nth(1).unwrap_or("5093".to_string()).parse()?;

    // Looking over solutions for this second iteration pointed me to:
    // https://en.wikipedia.org/wiki/Summed-area_table
    // https://www.reddit.com/r/adventofcode/comments/a53r6i/comment/ebjogd7/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    // Using a summed-area table takes the total-power calculation of a square to a constant time
    // which speeds this up from ~60s to ~0.70s.
    let mut sa_table = vec![vec![0; MAX + 1]; MAX + 1];
    for x in 1..=MAX {
        for y in 1..=MAX {
            sa_table[x][y] = (x, y).power_level(serial) + sa_table[x - 1][y] + sa_table[x][y - 1]
                - sa_table[x - 1][y - 1];
        }
    }

    let mut best = Square::default();
    let mut best_3x3 = Square::default();
    for square in 1..=MAX {
        for x in square..=MAX {
            for y in square..=MAX {
                let total_power =
                    sa_table[x][y] - sa_table[x - square][y] - sa_table[x][y - square]
                        + sa_table[x - square][y - square];
                // Calculate the top left corner of the square
                let coords = (x - square + 1, y - square + 1);
                let new = Square {
                    total_power,
                    coords,
                    square,
                };
                best = best.max(new.clone());
                if square == 3 {
                    best_3x3 = best_3x3.max(new);
                }
            }
        }
    }

    println!("Part 1: {}", best_3x3);
    println!("Part 2: {}", best);

    Ok(())
}

trait FuelCell {
    fn power_level(&self, serial: isize) -> isize;
}

impl FuelCell for (usize, usize) {
    fn power_level(&self, serial: isize) -> isize {
        let (x, y) = (self.0 as isize, self.1 as isize);
        let rack_id = x + 10;
        let pow = (rack_id * y + serial) * rack_id;
        ((pow / 100) % 10) - 5
    }
}

#[derive(Default, Clone)]
struct Square {
    total_power: isize,
    coords: (usize, usize),
    square: usize,
}

impl Eq for Square {}

impl PartialEq<Self> for Square {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Square {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Square {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_power.cmp(&other.total_power)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.coords.0, self.coords.1, self.square)
    }
}
