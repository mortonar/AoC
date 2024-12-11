use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{Lines, StdinLock};

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();

    // Map of page # to required preceding page #s
    let mut rules = parse_rules(&mut lines);

    let mut total = 0;
    for l in lines.flatten() {
        let updates: Vec<u64> = l.split(",").map(|p| p.parse::<u64>()).flatten().collect();
        if in_order(&updates, &rules) {
            total += updates[updates.len() / 2];
        }
    }

    println!("{}", total);
    Ok(())
}

fn parse_rules(lines: &mut Lines<StdinLock>) -> HashMap<u64, HashSet<u64>> {
    let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
    for l in lines.flatten() {
        if l.is_empty() {
            break;
        }

        let page_nums: Vec<u64> = l
            .split("|")
            .map(|page_num| page_num.parse::<u64>())
            .flatten()
            .collect();
        rules
            .entry(page_nums[1])
            .and_modify(|rule| {
                rule.insert(page_nums[0]);
            })
            .or_insert(HashSet::from([page_nums[0]]));
    }
    rules
}

fn in_order(pages: &Vec<u64>, rules: &HashMap<u64, HashSet<u64>>) -> bool {
    let all_pages: HashSet<u64> = HashSet::from_iter(pages.iter().map(|p| *p));
    let mut processed = HashSet::new();
    let mut in_order = true;
    for p in pages {
        if let Some(preceding) = rules.get(p) {
            if preceding
                .iter()
                .any(|prec| all_pages.contains(prec) && !processed.contains(prec))
            {
                in_order = false;
                break;
            }
        }
        processed.insert(*p);
    }
    in_order
}
