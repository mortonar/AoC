use anyhow::Result;
use regex::Regex;
use std::io;

fn main() -> Result<()> {
    let pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let mut total = 0;
    for line in io::stdin().lines() {
        for capture in pattern.captures_iter(&line?) {
            let (_full, [n1, n2]) = capture.extract();
            total += n1.parse::<u64>()? * n2.parse::<u64>()?;
        }
    }
    println!("{}", total);
    Ok(())
}
