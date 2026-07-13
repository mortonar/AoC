use anyhow::{Error, Result, bail};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let (rules, my_ticket, other_tickets) = parse_input()?;

    println!("Part 1: {}", part1(&rules, &other_tickets));
    println!("Part 2: {}", part2(&rules, &my_ticket, &other_tickets)?);

    Ok(())
}

fn parse_input() -> Result<(Rules, Ticket, Vec<Ticket>)> {
    let mut lines = stdin().lock().lines();

    let mut rules = Rules::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let tokens: Vec<_> = line.trim().split(':').collect();
        let ranges: Vec<_> = tokens[1].split_ascii_whitespace().collect();
        rules.insert(
            tokens[0].to_string(),
            vec![parse_range(ranges[0])?, parse_range(ranges[2])?],
        );
    }

    // your ticket:
    lines.next().unwrap()?;
    let your_ticket = parse_ticket(&lines.next().unwrap()?)?;
    lines.next().unwrap()?;

    // nearby tickets:
    lines.next().unwrap()?;
    let mut other_tickets = Vec::new();
    for line in lines.by_ref() {
        other_tickets.push(parse_ticket(&line?)?);
    }

    Ok((rules, your_ticket, other_tickets))
}

fn parse_range(s: &str) -> Result<RangeInclusive<usize>> {
    let tokens: Vec<_> = s.trim().split('-').collect();
    Ok(tokens[0].parse()?..=tokens[1].parse()?)
}

fn parse_ticket(s: &str) -> Result<Ticket> {
    s.split(",")
        .map(|x| x.parse().map_err(Error::from))
        .collect()
}

type Rules = HashMap<String, Vec<RangeInclusive<usize>>>;

type Ticket = Vec<usize>;

fn part1(rules: &Rules, tickets: &[Ticket]) -> usize {
    tickets
        .iter()
        .flatten()
        .filter(|&&field| none_apply(rules, field))
        .sum()
}

fn part2(rules: &Rules, your_ticket: &Ticket, other_tickets: &[Ticket]) -> Result<usize> {
    let valid_ticket_indices = (0..other_tickets.len())
        .filter(|&t| {
            !other_tickets[t]
                .iter()
                .any(|&field| none_apply(rules, field))
        })
        .collect::<Vec<_>>();

    let rule_names = rules.keys().cloned().collect::<Vec<_>>();
    let mut candidates =
        vec![rule_names.iter().cloned().collect::<HashSet<_>>(); your_ticket.len()];

    for column_idx in 0..your_ticket.len() {
        for rule_name in &rule_names {
            let ranges = &rules[rule_name];
            let valid_for_nearby = valid_ticket_indices.iter().all(|&ticket_idx| {
                let value = other_tickets[ticket_idx][column_idx];
                ranges.iter().any(|range| range.contains(&value))
            });
            let valid_for_mine = ranges
                .iter()
                .any(|range| range.contains(&your_ticket[column_idx]));

            if !(valid_for_nearby && valid_for_mine) {
                candidates[column_idx].remove(rule_name);
            }
        }
    }

    let mut resolved: Vec<Option<String>> = vec![None; your_ticket.len()];
    while resolved.iter().any(|field| field.is_none()) {
        let mut progress = false;
        for i in 0..candidates.len() {
            if resolved[i].is_some() || candidates[i].len() != 1 {
                continue;
            }
            let field = candidates[i].iter().next().unwrap().clone();
            resolved[i] = Some(field.clone());
            for (j, candidate_set) in candidates.iter_mut().enumerate() {
                if i != j {
                    candidate_set.remove(&field);
                }
            }
            progress = true;
        }

        if !progress {
            bail!("Could not resolve all field positions");
        }
    }

    Ok(resolved
        .iter()
        .enumerate()
        .filter_map(|(i, field)| {
            field
                .as_ref()
                .and_then(|name| name.starts_with("departure").then_some(your_ticket[i]))
        })
        .product())
}

fn none_apply(rules: &Rules, field: usize) -> bool {
    !rules
        .values()
        .any(|ranges| ranges.iter().any(|range| range.contains(&field)))
}
