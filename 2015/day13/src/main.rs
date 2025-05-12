use anyhow::Result;
use std::cmp::max;
use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Neg;

fn main() -> Result<()> {
    let mut happiness: HashMap<String, HashMap<String, isize>> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        let from = tokens[0].to_string();
        let mut to = tokens[10].to_string();
        // Trim trailing period
        to.pop();
        let mut level = tokens[3].parse::<isize>()?;
        if tokens[2] == "lose" {
            level = level.neg()
        }
        happiness
            .entry(from)
            .or_insert(HashMap::new())
            .entry(to)
            .or_insert(level);
    }

    let mut seating: Vec<&str> = vec![];
    println!("Part 1: {}", arrange(&mut seating, &happiness));

    let mut you_map = HashMap::new();
    for (k, v) in &mut happiness {
        v.insert("You".to_string(), 0);
        you_map.insert(k.clone(), 0);
    }
    happiness.insert("You".to_string(), you_map);
    let mut seating: Vec<&str> = vec![];
    println!("Part 2: {}", arrange(&mut seating, &happiness));

    Ok(())
}

fn arrange<'a>(
    seating: &mut Vec<&'a str>,
    happiness: &'a HashMap<String, HashMap<String, isize>>,
) -> isize {
    if seating.len() == happiness.len() {
        return score(seating, happiness);
    }

    let mut optimal = 0;
    for from in happiness.keys() {
        let from = from.as_str();
        if !seating.contains(&from) {
            seating.push(from);
            optimal = max(optimal, arrange(seating, happiness));
            seating.pop();
        }
    }
    optimal
}

fn score(seating: &mut Vec<&str>, happiness: &HashMap<String, HashMap<String, isize>>) -> isize {
    let score = seating
        .windows(2)
        .map(|people| happiness[people[0]][people[1]] + happiness[people[1]][people[0]])
        .sum::<isize>();
    let first = *seating.first().unwrap();
    let last = *seating.last().unwrap();
    score + happiness[last][first] + happiness[first][last]
}
