use anyhow::Result;
use regex::Regex;
use std::io;

fn main() -> Result<()> {
    // Use multiple patterns since unbalanced capture groups aren't supported
    let ins_pattern = Regex::new(r"((mul)\([0-9]{1,3},[0-9]{1,3}\))|((do)\(\))|((don)'t\(\))")?;
    let mult_pattern = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let mut total = 0;
    let mut mult_enabled = true;
    for line in io::stdin().lines() {
        for capture in ins_pattern.captures_iter(&line?) {
            let (full, [_, instruction]) = capture.extract();
            match instruction {
                "do" => mult_enabled = true,
                "don" => mult_enabled = false,
                "mul" => {
                    if mult_enabled {
                        let (_full, [n1, n2]) = mult_pattern.captures(full).unwrap().extract();
                        total += n1.parse::<u64>()? * n2.parse::<u64>()?;
                    }
                }
                _ => panic!("Unrecognized instruction: {}", instruction),
            }
        }
    }
    println!("{}", total);
    Ok(())
}
