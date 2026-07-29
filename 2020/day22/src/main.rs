use anyhow::{Result, bail};
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (p1, p2) = parse_input()?;

    println!("Part 1: {}", combat(p1.clone(), p2.clone())?);
    let (_winner, deck) = recursive_combat(p1, p2)?;
    println!("Part 2: {}", deck.score());

    Ok(())
}

fn parse_input() -> Result<(Deck, Deck)> {
    let (mut p1, mut p2) = (Deck::default(), Deck::default());
    let mut player = &mut p1;
    for line in stdin().lock().lines() {
        let line = line?;
        if line.starts_with("Player 1:") {
            player = &mut p1;
        } else if line.starts_with("Player 2:") {
            player = &mut p2;
        } else if !line.trim().is_empty() {
            player.take_bottom(line.trim().parse()?);
        }
    }
    Ok((p1, p2))
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Deck {
    cards: VecDeque<usize>,
}

impl Deck {
    fn draw_top(&mut self) -> Option<usize> {
        self.cards.pop_front()
    }

    fn take_bottom(&mut self, card: usize) {
        self.cards.push_back(card);
    }

    fn take_top(&mut self, card: usize) {
        self.cards.push_front(card);
    }

    fn cut(&self, cards: usize) -> Deck {
        Self {
            cards: self.cards.iter().take(cards).cloned().collect(),
        }
    }

    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &c)| (i + 1) * c)
            .sum()
    }
}

fn combat(p1: Deck, p2: Deck) -> Result<usize> {
    let (mut p1, mut p2) = (p1.clone(), p2.clone());
    loop {
        match (p1.draw_top(), p2.draw_top()) {
            (Some(c1), Some(c2)) => {
                if c1 > c2 {
                    p1.take_bottom(c1);
                    p1.take_bottom(c2);
                } else {
                    p2.take_bottom(c2);
                    p2.take_bottom(c1);
                }
            }
            (Some(c1), None) => {
                p1.take_top(c1);
                return Ok(p1.score());
            }
            (None, Some(c2)) => {
                p2.take_top(c2);
                return Ok(p2.score());
            }
            (None, None) => bail!("Both players empty"),
        }
    }
}

fn recursive_combat(p1: Deck, p2: Deck) -> Result<(Winner, Deck)> {
    let mut previous = HashSet::new();
    let (mut p1, mut p2) = (p1.clone(), p2.clone());

    loop {
        if !previous.insert((p1.clone(), p2.clone())) {
            return Ok((Winner::P1, p1));
        }

        match (p1.draw_top(), p2.draw_top()) {
            (Some(c1), Some(c2)) if p1.cards.len() >= c1 && p2.cards.len() >= c2 => {
                let (winner, _) = recursive_combat(p2.cut(c1), p2.cut(c2))?;
                if matches!(winner, Winner::P1) {
                    p1.take_bottom(c1);
                    p1.take_bottom(c2);
                } else {
                    p2.take_bottom(c2);
                    p2.take_bottom(c1);
                }
            }
            (Some(c1), Some(c2)) => {
                if c1 > c2 {
                    p1.take_bottom(c1);
                    p1.take_bottom(c2);
                } else {
                    p2.take_bottom(c2);
                    p2.take_bottom(c1);
                }
            }
            (Some(c1), None) => {
                p1.take_top(c1);
                return Ok((Winner::P1, p1));
            }
            (None, Some(c2)) => {
                p2.take_top(c2);
                return Ok((Winner::P2, p2));
            }
            (None, None) => bail!("Both players empty"),
        }
    }
}

enum Winner {
    P1,
    P2,
}
