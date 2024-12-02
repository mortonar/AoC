use anyhow::Result;
use std::collections::BinaryHeap;
use std::io;

fn main() -> Result<()> {
    let mut list_one = BinaryHeap::new();
    let mut list_two = BinaryHeap::new();
    for line in io::stdin().lines() {
        let nums: Vec<u64> = line?
            .split_ascii_whitespace()
            .take(2)
            .map(|n| n.parse::<u64>())
            .flatten()
            .collect();
        list_one.push(nums[0]);
        list_two.push(nums[1]);
    }
    let total: u64 = list_one
        .into_sorted_vec()
        .into_iter()
        .zip(list_two.into_sorted_vec().into_iter())
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum();
    println!("{}", total);
    Ok(())
}
