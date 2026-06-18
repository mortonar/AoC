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
    fn seat_id(&self) -> usize {
        let row = partition_select(&self[0..=6], 127);
        let col = partition_select(&self[7..], 7);
        row * 8 + col
    }
}

fn partition_select(seq: &str, upper: usize) -> usize {
    let (mut lower, mut upper) = (0, upper);
    for s in seq.chars() {
        match s {
            'F' | 'L' => {
                upper = (upper - lower) / 2 + lower;
            }
            'B' | 'R' => {
                lower = (upper - lower) / 2 + lower + 1;
            }
            _ => panic!("Unrecognised character: {s}"),
        }
    }

    if lower != upper {
        panic!("Sequence {seq} did not perfectly split the space. lower: {lower} | upper: {upper}");
    }

    lower
}
