use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let food_list = parse_input()?;

    let safe_ingredients = safe_ingredients(&food_list);
    let safe_appearances = food_list
        .ingredients
        .iter()
        .flatten()
        .filter(|i| safe_ingredients.contains(*i))
        .count();
    println!("Part 1: {safe_appearances}");

    println!("Part 2: {}", match_allergens(&food_list));

    Ok(())
}

fn parse_input() -> Result<FoodList> {
    let mut ingredients = Vec::new();
    let mut allergens = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.trim().split(" (").collect();
        let ingredient_list: Vec<_> = tokens[0]
            .split_ascii_whitespace()
            .map(|f| f.to_string())
            .collect();
        let allergen: Vec<_> = tokens[1]
            .split_ascii_whitespace()
            .skip(1)
            .map(|t| t[0..t.len() - 1].to_string())
            .collect();
        ingredients.push(ingredient_list);
        allergens.push(allergen);
    }

    assert_eq!(ingredients.len(), allergens.len());

    Ok(FoodList {
        ingredients,
        allergens,
    })
}

/// `ingredients` | `allergens` at `[i]` is place `i` in food list
#[derive(Debug)]
struct FoodList {
    ingredients: Vec<Vec<String>>,
    allergens: Vec<Vec<String>>,
}

impl FoodList {
    fn len(&self) -> usize {
        self.allergens.len()
    }
}

/// Allergen -> candidate ingredients, narrowed by intersecting the ingredient
/// lists of every food that mentions that allergen.
fn allergen_candidates(food_list: &FoodList) -> HashMap<&String, HashSet<&String>> {
    let mut allergen_candidates: HashMap<&String, HashSet<&String>> = HashMap::new();
    for i in 0..food_list.len() {
        let (ingredients, allergens) = (&food_list.ingredients[i], &food_list.allergens[i]);
        for allergen in allergens {
            if let Some(candidates) = allergen_candidates.get_mut(allergen) {
                let ingredient_set: HashSet<&String> = ingredients.iter().collect();
                candidates.retain(|c| ingredient_set.contains(c));
            } else {
                allergen_candidates.insert(allergen, ingredients.iter().collect());
            }
        }
    }
    allergen_candidates
}

fn safe_ingredients(food_list: &FoodList) -> HashSet<&String> {
    let suspect_ingredients: HashSet<_> = allergen_candidates(food_list)
        .values()
        .flatten()
        .copied()
        .collect();

    food_list
        .ingredients
        .iter()
        .flatten()
        .filter(|i| !suspect_ingredients.contains(i))
        .collect()
}

fn match_allergens(food_list: &FoodList) -> String {
    let mut candidates = allergen_candidates(food_list);
    let mut resolved: Vec<(&String, &String)> = Vec::new();

    while !candidates.is_empty() {
        let (allergen, ingredient) = candidates
            .iter()
            .find_map(|(&a, c)| (c.len() == 1).then(|| (a, *c.iter().next().unwrap())))
            .expect("input should resolve to a unique allergen/ingredient matching");

        candidates.remove(allergen);
        for remaining in candidates.values_mut() {
            remaining.remove(ingredient);
        }

        resolved.push((allergen, ingredient));
    }

    resolved.sort_by_key(|(allergen, _)| *allergen);

    resolved
        .into_iter()
        .map(|(_, ingredient)| ingredient.as_str())
        .collect::<Vec<_>>()
        .join(",")
}
