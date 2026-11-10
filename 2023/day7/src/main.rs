use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, anyhow, bail};

fn main() -> Result<()> {
    let hands = parse_input()?;

    println!("Part 1: {}", part1(&hands));
    println!("Part 2: {}", part2(&hands));

    Ok(())
}

fn parse_input() -> Result<Vec<Hand>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(hands: &[Hand]) -> usize {
    winnings(hands, false)
}

fn part2(hands: &[Hand]) -> usize {
    winnings(hands, true)
}

fn winnings(hands: &[Hand], wild: bool) -> usize {
    let mut hands = hands.to_vec();

    if wild {
        for h in hands.iter_mut() {
            for v in h.cards.iter_mut() {
                if *v == Val::Jack {
                    *v = Val::Joker;
                }
            }
        }
    }

    hands.sort_by(|h1, h2| match h1.hand_type(wild).cmp(&h2.hand_type(wild)) {
        std::cmp::Ordering::Less => Less,
        std::cmp::Ordering::Equal => h1.cards.cmp(&h2.cards),
        std::cmp::Ordering::Greater => Greater,
    });

    hands.iter().zip(1..).map(|(h, rank)| h.bid * rank).sum()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    cards: [Val; 5],
    bid: usize,
}

impl Hand {
    fn hand_type(&self, wild_cards: bool) -> HandType {
        let mut hist: HashMap<Val, u8> = HashMap::new();
        self.cards.iter().for_each(|c| {
            hist.entry(c.clone()).and_modify(|c| *c += 1).or_insert(1);
        });

        // Join Jokers to the largest group
        let jokers = if wild_cards {
            hist.remove(&Val::Joker).unwrap_or(0)
        } else {
            0
        };

        let mut counts: Vec<u8> = hist.into_values().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        match counts.first_mut() {
            Some(largest) => *largest += jokers,
            // All Jokers
            None => counts.push(jokers),
        }

        match counts.as_slice() {
            [5] => HandType::Five,
            [4, ..] => HandType::Four,
            [3, 2] => HandType::Full,
            [3, ..] => HandType::Three,
            [2, 2, ..] => HandType::Two,
            [2, ..] => HandType::One,
            _ => HandType::High,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Val {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some((cards, bid)) = s.split_once(' ') else {
            bail!("illformatted hand");
        };

        let cards = cards
            .chars()
            .map(Val::new)
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .map_err(|v: Vec<Val>| anyhow!("expected 5 cards, got {}", v.len()))?;

        Ok(Hand {
            cards,
            bid: bid.parse()?,
        })
    }
}

impl Val {
    fn new(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Val::Ace),
            'K' => Ok(Val::King),
            'Q' => Ok(Val::Queen),
            'J' => Ok(Val::Jack),
            'T' => Ok(Val::Ten),
            '9' => Ok(Val::Nine),
            '8' => Ok(Val::Eight),
            '7' => Ok(Val::Seven),
            '6' => Ok(Val::Six),
            '5' => Ok(Val::Five),
            '4' => Ok(Val::Four),
            '3' => Ok(Val::Three),
            '2' => Ok(Val::Two),
            _ => bail!("unrecognized card: {c}"),
        }
    }
}
