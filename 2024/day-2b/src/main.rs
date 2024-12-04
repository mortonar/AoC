use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut safe_count = 0;
    for line in io::stdin().lines() {
        let nums = parse_line(line?);
        for skip in 0..nums.len() {
            let nums = skipped(&nums, skip);
            if is_safe(&nums) {
                safe_count += 1;
                break;
            }
        }
    }
    println!("{}", safe_count);
    Ok(())
}

fn parse_line(line: String) -> Vec<u64> {
    line.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

// Return a Vec with the given element skipped
fn skipped(nums: &[u64], skip: usize) -> Vec<u64> {
    nums.iter()
        .enumerate()
        .filter(|&(i, _)| i != skip)
        .map(|(_, v)| *v)
        .collect()
}

fn is_safe(nums: &[u64]) -> bool {
    let mut iter = nums.windows(2);
    let first_two = iter.next().unwrap();
    let mut safe = false;
    if in_tolerance(first_two) {
        let inc = increasing(first_two);
        safe = true;
        for next_two in iter {
            if !(in_tolerance(next_two) && increasing(next_two) == inc) {
                safe = false;
                break;
            }
        }
    }
    return safe;
}

fn in_tolerance(nums: &[u64]) -> bool {
    let diff = nums[0].abs_diff(nums[1]);
    diff >= 1 && diff <= 3
}

fn increasing(nums: &[u64]) -> bool {
    nums[0] < nums[1]
}
