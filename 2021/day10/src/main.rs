use anyhow::{Error, Result};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let nav = parse_input()?;

    println!("Part 1: {}", part1(&nav));
    println!("Part 2: {}", part2(&nav));

    Ok(())
}

fn parse_input() -> Result<Vec<String>> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.map_err(Error::from))
        .collect()
}

fn part1(nav: &[String]) -> usize {
    nav.iter().filter_map(|n| syntax_eval(n).err()).sum()
}

fn part2(nav: &[String]) -> usize {
    let mut scores: Vec<_> = nav
        .iter()
        .filter_map(|n| syntax_eval(n).ok())
        .map(|stack| close_score(&stack))
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

/// Return either OK with remaining stack or error with syntax error score
fn syntax_eval(n: &str) -> Result<Vec<char>, usize> {
    let mut stack = Vec::new();
    for c in n.trim().chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            close => {
                let open = stack.pop().unwrap_or('x');
                let s = match (open, close) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => 0,
                    (_, ')') => 3,
                    (_, ']') => 57,
                    (_, '}') => 1197,
                    (_, '>') => 25137,
                    _ => panic!("Unhandled case: ({open}, {close})"),
                };
                if s != 0 {
                    return Err(s);
                }
            }
        }
    }

    Ok(stack)
}

fn close_score(stack: &[char]) -> usize {
    stack.iter().rev().fold(0, |score, open| {
        score * 5
            + match open {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unhandled open: {open}"),
            }
    })
}
