use anyhow::{Error, Result, anyhow};
use std::env;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let ops = parse_input()?;

    let size = env::args()
        .nth(1)
        .unwrap_or("10007".to_string())
        .parse::<usize>()?;
    let mut deck = Deck::new(size);
    deck.shuffle(&ops);
    let pos = deck.position(2019)?;
    println!("Part 1: {}", pos);

    // Reddit + AI for part 2 (this was beyond my abilities...)
    // From what I understand, positions can be calculated by a modular arithmetic function that's
    // derived from the input shuffle instructions.
    let deck_size: i128 = 119_315_717_514_047;
    let repeats: i128 = 101_741_582_076_661;
    let target_pos: i128 = 2020;

    let (a, b) = build_forward_affine(&ops, deck_size);
    let (a_k, b_k) = affine_pow(a, b, repeats, deck_size);
    let card = apply_inverse(a_k, b_k, target_pos, deck_size)?;
    println!("Part 2: {}", card);

    Ok(())
}

fn parse_input() -> Result<Vec<Ops>> {
    stdin().lines().map(|l| l?.parse()).collect()
}

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    fn new(size: usize) -> Self {
        let mut cards = Vec::with_capacity(size);
        (0..size).for_each(|i| cards.push(i));
        Self { cards }
    }

    fn shuffle(&mut self, ops: &[Ops]) {
        for op in ops {
            match op {
                Ops::NewStack => self.cards.reverse(),
                Ops::Cut(c) => {
                    let abs = c.unsigned_abs() % self.cards.len();
                    if c.is_negative() {
                        let mut cut: Vec<_> =
                            self.cards.drain((self.cards.len() - abs)..).collect();
                        cut.append(&mut self.cards);
                        self.cards = cut;
                    } else {
                        let mut cut: Vec<_> = self.cards.drain(0..abs).collect();
                        self.cards.append(&mut cut);
                    };
                }
                Ops::Increment(i) => {
                    let mut inc = vec![0; self.cards.len()];
                    let mut inc_idx = 0;
                    for c in self.cards.iter() {
                        inc[inc_idx] = *c;
                        inc_idx = (inc_idx + *i) % self.cards.len();
                    }
                    self.cards = inc;
                }
            }
        }
    }

    fn position(&self, card_num: usize) -> Result<usize> {
        self.cards
            .iter()
            .position(|&x| x == card_num)
            .ok_or_else(|| anyhow!("No {} card found", card_num))
    }
}

#[derive(Debug)]
enum Ops {
    NewStack,
    Cut(isize),
    Increment(usize),
}

impl FromStr for Ops {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens = s.trim().split_ascii_whitespace().collect::<Vec<_>>();
        if s.starts_with("cut") {
            Ok(Self::Cut(tokens[1].parse()?))
        } else if s.starts_with("deal into") {
            Ok(Self::NewStack)
        } else if s.starts_with("deal with increment") {
            Ok(Self::Increment(tokens[3].parse()?))
        } else {
            Err(anyhow!("Unknown op {}", s))
        }
    }
}

fn norm(x: i128, m: i128) -> i128 {
    let r = x % m;
    if r < 0 { r + m } else { r }
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = egcd(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

fn mod_inv(a: i128, m: i128) -> Result<i128> {
    let (g, x, _) = egcd(norm(a, m), m);
    if g != 1 {
        Err(anyhow!("No modular inverse for {} mod {}", a, m))
    } else {
        Ok(norm(x, m))
    }
}

fn compose(a2: i128, b2: i128, a1: i128, b1: i128, m: i128) -> (i128, i128) {
    (norm(a2 * a1, m), norm(a2 * b1 + b2, m))
}

fn build_forward_affine(ops: &[Ops], m: i128) -> (i128, i128) {
    let mut a = 1_i128;
    let mut b = 0_i128;
    for op in ops {
        let (oa, ob) = match op {
            Ops::NewStack => (-1_i128, -1_i128),
            Ops::Cut(n) => (1_i128, -(*n as i128)),
            Ops::Increment(n) => (*n as i128, 0_i128),
        };
        (a, b) = compose(oa, ob, a, b, m);
    }
    (a, b)
}

fn affine_pow(mut a: i128, mut b: i128, mut k: i128, m: i128) -> (i128, i128) {
    let mut ra = 1_i128;
    let mut rb = 0_i128;
    while k > 0 {
        if (k & 1) == 1 {
            (ra, rb) = compose(a, b, ra, rb, m);
        }
        (a, b) = compose(a, b, a, b, m);
        k >>= 1;
    }
    (ra, rb)
}

fn apply_inverse(a: i128, b: i128, y: i128, m: i128) -> Result<i128> {
    let inv_a = mod_inv(a, m)?;
    Ok(norm((y - b) * inv_a, m))
}
