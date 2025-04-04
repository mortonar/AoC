use anyhow::Result;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line)?;

    let mut floor = 0;
    let mut basement_ins = 0;
    line.chars().enumerate().for_each(|(pos, ins)| {
        if ins == '(' {
            floor += 1
        } else {
            floor -= 1
        };
        if basement_ins == 0 && floor == -1 {
            basement_ins = pos + 1;
        }
    });
    println!("Part 1: {floor}");
    println!("Part 2: {basement_ins}");

    Ok(())
}
