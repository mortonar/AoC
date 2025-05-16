use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<()> {
    let to_match = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let prop_parser = Regex::new(r#"([a-z]+): ([0-9]+),?"#)?;
    let mut sue_num: usize = 0;
    for line in std::io::stdin().lock().lines() {
        sue_num += 1;
        let line = line?;

        let mut properties = HashMap::new();
        for (_, [label, value]) in prop_parser.captures_iter(&line).map(|c| c.extract()) {
            properties.insert(label, value.parse::<usize>()?);
        }

        if properties.iter().all(|(prop, val)| to_match[prop] == *val) {
            println!("Part 1: {sue_num}");
        }

        if properties.iter().all(|(&prop, &val)| match prop {
            "cats" | "trees" => to_match[prop] < val,
            "pomeranians" | "goldfish" => to_match[prop] > val,
            _ => to_match[prop] == properties[prop],
        }) {
            println!("Part 2: {sue_num}");
        }
    }

    Ok(())
}
