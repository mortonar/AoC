use anyhow::Result;
use std::collections::HashSet;
use std::io::stdin;

fn main() -> Result<()> {
    let frequencies = parse_input()?;

    let frequency = frequencies.iter().sum::<isize>();
    println!("Part 1: {frequency}");

    let mut seen = HashSet::new();
    let mut frequency = 0;
    for change in frequencies.iter().cycle() {
        if !seen.insert(frequency) {
            println!("Part 2: {frequency}");
            break;
        }
        frequency += change;
    }

    Ok(())
}

fn parse_input() -> Result<Vec<isize>> {
    stdin()
        .lines()
        .map(|l| l?.trim_end().parse::<isize>().map_err(Into::into))
        .collect()
}
