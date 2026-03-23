use anyhow::Result;
use std::env;

const INPUT: &str = "540561";

fn main() -> Result<()> {
    let num_recipes = env::args()
        .nth(1)
        .unwrap_or(INPUT.to_string())
        .parse::<usize>()?;
    let digits: Vec<_> = env::args()
        .nth(2)
        .unwrap_or(INPUT.to_string())
        .chars()
        .map(|d| d.to_digit(10).unwrap() as usize)
        .collect();

    let mut recipes = vec![3, 7];
    let (mut e1, mut e2) = (0, 1);
    let mut part1 = None;
    let mut part2 = None;

    while part1.is_none() || part2.is_none() {
        let sum = recipes[e1] + recipes[e2];
        if sum >= 10 {
            recipes.push(sum / 10);
            if part2.is_none() && recipes.ends_with(&digits) {
                part2 = Some(recipes.len() - digits.len());
            }
        }
        recipes.push(sum % 10);
        if part2.is_none() && recipes.ends_with(&digits) {
            part2 = Some(recipes.len() - digits.len());
        }

        e1 = (e1 + recipes[e1] + 1) % recipes.len();
        e2 = (e2 + recipes[e2] + 1) % recipes.len();

        if part1.is_none() && recipes.len() >= num_recipes + 10 {
            part1 = Some(
                recipes[num_recipes..num_recipes + 10]
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<String>(),
            );
        }
    }

    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {}", part2.unwrap());

    Ok(())
}
