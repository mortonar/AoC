use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::io::BufRead;

fn main() -> Result<()> {
    let to_store: usize = env::args().nth(1).unwrap_or("150".to_string()).parse()?;

    let containers: Vec<usize> = {
        let mut containers = Vec::new();
        for line in std::io::stdin().lock().lines() {
            containers.push(line?.parse()?);
        }
        containers
    };

    // Selection length -> # of ways to build a selection of that length
    let mut combinations = HashMap::new();
    // mask @ bit i == 1 -> choosing containers[i]
    for mask in 1..=(1 << containers.len()) {
        // Calculate what containers are selected with this mask
        let (num_selected, sum) = containers
            .iter()
            .enumerate()
            .filter(|(i, _c)| mask & (1 << *i) > 0)
            .fold((0, 0), |(num_selected, sum), (_i, c)| {
                (num_selected + 1, sum + *c)
            });

        if sum == to_store {
            combinations
                .entry(num_selected)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    println!("Part 1: {}", combinations.values().sum::<usize>());
    println!(
        "Part 2: {}",
        combinations[combinations.keys().min().unwrap()]
    );

    Ok(())
}
