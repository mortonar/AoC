use anyhow::{Result, anyhow};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let lines: Vec<_> = stdin().lock().lines().map_while(|l| l.ok()).collect();
    part1(&lines)?;
    part2(&lines)?;
    Ok(())
}

fn part1(lines: &[String]) -> Result<()> {
    let mut numbers = Vec::new();

    for line in lines.iter() {
        if line.starts_with("+") || line.starts_with("*") {
            let mut total = 0;
            for (col, op) in line.split_ascii_whitespace().enumerate() {
                let iter = numbers.iter().map(|row: &Vec<usize>| row[col]);
                match op {
                    "+" => {
                        total += iter.sum::<usize>();
                    }
                    "*" => {
                        total += iter.product::<usize>();
                    }
                    _ => return Err(anyhow!("Invalid op: {}", op)),
                }
            }

            println!("Part 1: {total}");

            break;
        } else {
            let row: Result<Vec<usize>, _> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>())
                .collect();
            numbers.push(row?);
        }
    }

    Ok(())
}

fn part2(lines: &[String]) -> Result<()> {
    let mut total = 0;
    let mut col_nums: Vec<usize> = Vec::new();
    let mut current_num = String::new();

    // Go through lines from right to left, top to bottom.
    for c in (0..lines[0].chars().count())
        .rev()
        .flat_map(|col| lines.iter().map(move |l| l.chars().nth(col).unwrap()))
    {
        if c == ' ' {
            if !current_num.is_empty() {
                col_nums.push(current_num.parse()?);
                current_num.clear();
            }
        } else if c.is_ascii_digit() {
            current_num.push(c);
        } else if matches!(c, '*' | '+') {
            // We may or may not have a number right above the operator (see the sample).
            if !current_num.is_empty() {
                col_nums.push(current_num.parse()?);
                current_num.clear();
            }
            if c == '+' {
                total += col_nums.iter().sum::<usize>();
            } else {
                total += col_nums.iter().product::<usize>();
            }
            col_nums.clear();
        } else {
            return Err(anyhow!("Unhandled character: {c}"));
        }
    }

    println!("Part 2: {total}");

    Ok(())
}
