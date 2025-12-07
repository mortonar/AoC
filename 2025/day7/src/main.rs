use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let manifold = parse_manifold()?;
    split_beam(&manifold)?;
    Ok(())
}

fn parse_manifold() -> Result<Vec<Vec<char>>> {
    stdin()
        .lock()
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect()
}

fn split_beam(manifold: &[Vec<char>]) -> Result<()> {
    let start_col = manifold[0]
        .iter()
        .position(|&c| c == 'S')
        .ok_or_else(|| anyhow!("Invalid input: no start ('S') found"))?;

    // Map: col the beam is in -> #paths it carries
    let mut beams: HashMap<usize, usize> = HashMap::new();
    beams.insert(start_col, 1);
    let mut total_splits = 0;

    for row in manifold.iter().skip(1) {
        // Get the splitting cols impacting the current beams
        let splitters: HashSet<usize> = row
            .iter()
            .enumerate()
            .filter_map(|(j, &c)| {
                if beams.contains_key(&j) && c == '^' {
                    Some(j)
                } else {
                    None
                }
            })
            .collect();

        if splitters.is_empty() {
            continue;
        }

        // Split the beam, carrying its paths forward.
        // NOTE: this includes adding cumulative paths when two beams merge!
        for splitter in splitters {
            total_splits += 1;

            let num_paths = beams.remove(&splitter).unwrap();
            for split_col in [splitter - 1, splitter + 1] {
                beams
                    .entry(split_col)
                    .and_modify(|p| *p += num_paths)
                    .or_insert(num_paths);
            }
        }
    }

    println!("Part 1: {total_splits}");
    println!("Part 2: {}", beams.values().sum::<usize>());

    Ok(())
}
