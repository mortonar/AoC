use anyhow::{Error, Result};
use std::io;

fn in_tolerance(nums: &[u64]) -> bool {
    let diff = nums[0].abs_diff(nums[1]);
    diff >= 1 && diff <= 3
}

fn increasing(nums: &[u64]) -> bool {
    nums[0] < nums[1]
}

fn main() -> Result<()> {
    let mut safe_count = 0;
    for line in io::stdin().lines() {
        let nums: Vec<u64> = line?
            .split_ascii_whitespace()
            .map(|n| n.parse::<u64>())
            .flatten()
            .collect();

        let mut iter = nums.windows(2);
        let first_two = iter.next().ok_or(Error::msg("Need at least two numbers"))?;
        if in_tolerance(first_two) {
            let inc = increasing(first_two);
            let mut safe = true;
            for next_two in iter {
                if !(in_tolerance(next_two) && increasing(next_two) == inc) {
                    safe = false;
                    break;
                }
            }
            if safe {
                safe_count += 1;
            }
        }
    }
    println!("{}", safe_count);
    Ok(())
}
