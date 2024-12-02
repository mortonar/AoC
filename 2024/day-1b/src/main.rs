use anyhow::Result;
use std::collections::HashMap;
use std::io;

fn main() -> Result<()> {
    let mut list_one = Vec::new();
    let mut list_two = HashMap::new();
    for line in io::stdin().lines() {
        let nums: Vec<u64> = line?
            .split_ascii_whitespace()
            .take(2)
            .map(|n| n.parse::<u64>())
            .flatten()
            .collect();
        list_one.push(nums[0]);
        list_two
            .entry(nums[1])
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let total: u64 = list_one
        .iter()
        .map(|n1| n1 * list_two.get(n1).unwrap_or(&0))
        .sum();
    println!("{}", total);
    Ok(())
}
