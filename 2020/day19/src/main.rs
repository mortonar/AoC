use anyhow::{Error, Result};
use std::collections::HashMap;
use std::io::{BufRead, stdin};

type Rules = HashMap<String, Vec<Vec<String>>>;

fn main() -> Result<()> {
    let (mut rules, messages) = parse_input()?;

    println!("Part 1: {}", match0(&rules, &messages));

    rules.insert(
        "8".into(),
        vec![vec!["42".into()], vec!["42".into(), "8".into()]],
    );
    rules.insert(
        "11".into(),
        vec![
            vec!["42".into(), "31".into()],
            vec!["42".into(), "11".into(), "31".into()],
        ],
    );
    println!("Part 2: {}", match0(&rules, &messages));

    Ok(())
}

fn parse_input() -> Result<(Rules, Vec<String>)> {
    let mut lines = stdin().lock().lines();

    let mut rules = HashMap::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }

        let mut sub_rules = Vec::new();
        let mut sub_rule = Vec::new();
        let tokens: Vec<_> = line.trim().split(": ").collect();
        let rule_num = tokens[0].to_string();
        for matches in tokens[1].split_ascii_whitespace() {
            if matches == "|" {
                sub_rules.push(sub_rule);
                sub_rule = Vec::new();
            } else {
                sub_rule.push(matches.to_string());
            }
        }
        sub_rules.push(sub_rule);
        rules.insert(rule_num, sub_rules);
    }

    let messages = lines
        .map(|line| line.map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;

    Ok((rules, messages))
}

fn match0(rules: &Rules, messages: &[String]) -> usize {
    messages
        .iter()
        .filter(|m| match_rule(rules, "0", 0, m.as_bytes(), &mut HashMap::new()).contains(&m.len()))
        .count()
}

fn match_rule(
    rules: &Rules,
    rule_id: &str,
    pos: usize,
    msg: &[u8],
    memo: &mut HashMap<(String, usize), Vec<usize>>,
) -> Vec<usize> {
    let cache_key = (rule_id.to_string(), pos);
    if let Some(cached) = memo.get(&cache_key) {
        return cached.clone();
    }

    let out = if let Some(expected) = terminal_char(rules.get(rule_id).unwrap()) {
        if pos < msg.len() && msg[pos] == expected {
            vec![pos + 1]
        } else {
            Vec::new()
        }
    } else {
        let mut out = Vec::new();
        for alt in rules.get(rule_id).unwrap() {
            let mut frontier = vec![pos];
            for next_rule in alt {
                let mut next_frontier = Vec::new();
                for p in frontier {
                    next_frontier.extend(match_rule(rules, next_rule, p, msg, memo));
                }
                frontier = next_frontier;
                if frontier.is_empty() {
                    break;
                }
            }
            out.extend(frontier);
        }
        out.sort_unstable();
        out.dedup();
        out
    };

    memo.insert(cache_key, out.clone());
    out
}

fn terminal_char(rule: &[Vec<String>]) -> Option<u8> {
    if rule.len() != 1 || rule[0].len() != 1 {
        return None;
    }
    let token = &rule[0][0];
    token
        .strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .filter(|t| t.len() == 1)
        .map(|t| t.as_bytes()[0])
}
