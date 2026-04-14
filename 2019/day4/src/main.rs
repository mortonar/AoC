use anyhow::Result;
use std::ops::RangeInclusive;
use text_io::try_scan;

fn main() -> Result<()> {
    let range = parse_input()?;

    let num_passwords = range.clone().filter(|p| p.is_valid(false)).count();
    println!("Part 1: {num_passwords}");

    let num_passwords = range.filter(|p| p.is_valid(true)).count();
    println!("Part 2: {num_passwords}");

    Ok(())
}

fn parse_input() -> Result<RangeInclusive<usize>> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    let (start, end): (usize, usize);
    try_scan!(line.bytes() => "{}-{}", start, end);

    Ok(start..=end)
}

trait Password {
    fn is_valid(&self, small_groups: bool) -> bool;
}

impl Password for usize {
    fn is_valid(&self, small_groups: bool) -> bool {
        let mut double = false;
        let mut prev = *self % 10;
        let mut seq = 1;

        let mut num = *self / 10;

        while num > 0 {
            let digit = num % 10;
            if digit > prev {
                return false;
            }

            if digit == prev {
                seq += 1;
            } else {
                double = double || if small_groups { seq == 2 } else { seq > 1 };
                seq = 1;
            }

            prev = digit;
            num /= 10;
        }

        // Handle last digit == to prev
        double = double || if small_groups { seq == 2 } else { seq > 1 };

        double
    }
}
