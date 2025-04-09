use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::io::{BufRead, stdin};

type Coords = (isize, isize);

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;

    println!("Part 1: {}", houses_delivered(&line, false)?);
    println!("Part 2: {}", houses_delivered(&line, true)?);

    Ok(())
}

fn houses_delivered(instructions: &str, split: bool) -> Result<usize> {
    let mut santa: Coords = (0, 0);
    let mut robot: Coords = (0, 0);

    let mut santa_deliver = true;

    let mut houses = HashSet::new();
    houses.insert(santa);

    for ins in instructions.trim().chars() {
        let to_move = if !split || santa_deliver {
            &mut santa
        } else {
            &mut robot
        };
        match ins {
            '^' => to_move.0 -= 1,
            'v' => to_move.0 += 1,
            '>' => to_move.1 += 1,
            '<' => to_move.1 -= 1,
            _ => return Err(anyhow!(format!("Invalid input char {ins}"))),
        }
        houses.insert(santa);
        houses.insert(robot);
        santa_deliver = !santa_deliver;
    }

    Ok(houses.len())
}
