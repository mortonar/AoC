use std::{
    cmp::Ordering,
    io::{BufRead, stdin},
    iter::Peekable,
    str::FromStr,
};

use anyhow::{Error, Result, anyhow, bail};

fn main() -> Result<()> {
    let packets = parse_input()?;

    println!("Part 1: {}", part1(&packets));
    println!("Part 2: {}", part2(&packets));

    Ok(())
}

fn parse_input() -> Result<Vec<(List, List)>> {
    let mut lines = stdin().lock().lines();
    let mut pairs = Vec::new();

    while let Some(l1) = lines.next() {
        let l1 = l1?;

        if l1.is_empty() {
            continue;
        }

        let l2 = lines
            .next()
            .ok_or_else(|| anyhow!("missing second list"))??;

        pairs.push((l1.parse()?, l2.parse()?));
    }

    Ok(pairs)
}

fn part1(packets: &[(List, List)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1 <= p2)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(packets: &[(List, List)]) -> usize {
    let mut all_packets: Vec<_> = packets.iter().flat_map(|(p1, p2)| [p1, p2]).collect();

    let dividers: [List; 2] = ["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    all_packets.extend(dividers.iter());

    all_packets.sort();

    dividers
        .iter()
        .map(|d| all_packets.binary_search(&d).unwrap() + 1)
        .product()
}

#[derive(Debug)]
struct List {
    items: Vec<Item>,
}

#[derive(Debug)]
enum Item {
    Val(usize),
    List(List),
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for List {}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.items.cmp(&other.items)
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Val(v1), Item::Val(v2)) => v1.cmp(v2),
            (Item::Val(_), Item::List(l2)) => std::slice::from_ref(self).cmp(&l2.items),
            (Item::List(l1), Item::Val(_)) => l1.items[..].cmp(std::slice::from_ref(other)),
            (Item::List(l1), Item::List(l2)) => l1.cmp(l2),
        }
    }
}

impl FromStr for List {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut tokens = tokenize(s, &['[', ']', ',']).peekable();
        parse_list(&mut tokens)
    }
}

fn tokenize(s: &str, delimeters: &[char]) -> impl Iterator<Item = Result<Token>> {
    let mut remaining = s;
    std::iter::from_fn(move || {
        if remaining.is_empty() {
            return None;
        }

        let idx = remaining
            .find(|c| delimeters.contains(&c))
            .unwrap_or(remaining.len());
        let idx = if idx == 0 { 1 } else { idx };

        let (token, rest) = remaining.split_at(idx);
        remaining = rest;
        Some(token.parse())
    })
}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    ListOpen,
    ListClose,
    Comma,
    Num(usize),
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "[" => Ok(Token::ListOpen),
            "]" => Ok(Token::ListClose),
            "," => Ok(Token::Comma),
            s => Ok(Token::Num(s.parse()?)),
        }
    }
}

fn parse_list(tokens: &mut Peekable<impl Iterator<Item = Result<Token>>>) -> Result<List> {
    let mut items = Vec::new();

    let tok = tokens.next().ok_or_else(|| anyhow!("expected token"))??;
    if tok != Token::ListOpen {
        bail!("expected list open");
    }

    loop {
        let Some(next_tok) = tokens.peek() else {
            bail!("expected list close");
        };

        match next_tok {
            Ok(Token::ListOpen) => items.push(Item::List(parse_list(tokens)?)),
            Ok(Token::ListClose) => {
                let _ = tokens.next();
                break;
            }
            Ok(Token::Comma) => {
                let _ = tokens.next();
            }
            Ok(Token::Num(v)) => {
                items.push(Item::Val(*v));
                let _ = tokens.next();
            }
            Err(_) => {
                tokens.next().unwrap()?;
                unreachable!()
            }
        }
    }

    Ok(List { items })
}
