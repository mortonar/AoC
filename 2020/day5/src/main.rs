use anyhow::{Error, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let passes = parse_input()?;

    let seat_ids = {
        let mut seat_ids: Vec<usize> = passes.iter().map(|p| p.seat_id()).collect();
        seat_ids.sort_unstable();
        seat_ids
    };

    let max = seat_ids[seat_ids.len() - 1];
    println!("Part 1: {max}");

    for seats in seat_ids.windows(2) {
        let (s1, s2) = (seats[0], seats[1]);
        if s2 == s1 + 2 {
            println!("Part 2: {}", s1 + 1);
            break;
        }
    }

    Ok(())
}

fn parse_input() -> Result<Vec<String>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l.map_err(Error::from))
        .collect()
}

trait BoardingPass {
    fn seat_id(&self) -> usize;
}

impl BoardingPass for String {
    // Apparently "binary space partitioning" the problem is describing amounts to representing the
    // row/col selection as one 10-bit binary number. Multiplying row num (7 bits) by 8 is a shift
    // by 3, making room for the last 3 bits of the column.
    fn seat_id(&self) -> usize {
        let mut id = 0;
        let mut pow_2 = 1;
        for s in self.chars().rev() {
            match s {
                'B' | 'R' => {
                    id += pow_2;
                }
                _ => {}
            }
            pow_2 <<= 1;
        }
        id
    }
}
