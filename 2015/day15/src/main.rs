use anyhow::Result;
use std::cmp::max;
use std::io::BufRead;

fn main() -> Result<()> {
    // Capacity(0), durability(1), flavor(2), texture(3), calories(4)
    let mut properties: [Vec<isize>; 5] = [const { Vec::new() }; 5];
    // Mapping[i] = input line position of property i
    let mapping: [usize; 5] = [2, 4, 6, 8, 10];
    for line in std::io::stdin().lock().lines() {
        let line = line?.trim().replace(',', "");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        for (i, &j) in mapping.iter().enumerate() {
            properties[i].push(tokens[j].parse()?);
        }
    }

    let num_ingredients = properties[0].len();
    let mut selection = vec![0; num_ingredients];
    println!(
        "Part 1 {}",
        max_score(&properties, &mut selection, num_ingredients, None)
    );
    println!(
        "Part 2 {}",
        max_score(&properties, &mut selection, num_ingredients, Some(500))
    );

    Ok(())
}

fn max_score(
    properties: &[Vec<isize>; 5],
    selection: &mut [usize],
    num_ingredients: usize,
    calorie_target: Option<usize>,
) -> usize {
    if num_ingredients == 0 {
        if let Some(target) = calorie_target {
            let calories = properties
                .last()
                .unwrap()
                .iter()
                .zip(&mut *selection)
                .map(|(c, s)| c * *s as isize)
                .sum::<isize>();
            let calories = max(calories, 0) as usize;
            if calories != target {
                return 0;
            }
        }

        let mut total = 1;
        for prop in properties[0..properties.len() - 1].iter() {
            let prop_total = prop
                .iter()
                .zip(&mut *selection)
                .map(|(p, s)| p * *s as isize)
                .sum::<isize>();
            total *= max(prop_total, 0) as usize;
        }
        return total;
    }

    // Only iterate over a range of possible selections based on what's already chosen
    let upper = 100 - selection[num_ingredients..].iter().sum::<usize>() as isize;
    let upper = max(upper, 0) as usize;
    let (lower, upper) = if num_ingredients != 1 {
        (0, upper)
    } else {
        (upper, upper)
    };
    (lower..=upper)
        .into_iter()
        .map(|i| {
            selection[num_ingredients - 1] = i;
            max_score(properties, selection, num_ingredients - 1, calorie_target)
        })
        .max()
        .unwrap()
}
