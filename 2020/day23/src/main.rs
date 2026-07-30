use anyhow::{Result, bail};
use std::env;
use std::io::stdin;
use std::str::FromStr;

fn main() -> Result<()> {
    let cups = parse_input()?;
    let moves = env::args().nth(1).unwrap_or("100".to_string()).parse()?;

    println!("Part 1: {}", part1(&cups, moves));
    println!("Part 2: {}", part2(&cups));

    Ok(())
}

fn parse_input() -> Result<Cups> {
    stdin()
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No input"))??
        .parse()
}

fn part1(cups: &Cups, moves: usize) -> usize {
    let mut cups = cups.clone();
    (0..moves).for_each(|_| cups.do_move());
    cups.labels()
}

fn part2(cups: &Cups) -> usize {
    let mut cups = cups.clone();
    cups.enlarge_by(1_000_000);

    (0..10_000_000).for_each(|_| cups.do_move());

    let a = cups.next[1];
    let b = cups.next[a];
    a * b
}

#[derive(Debug, Clone)]
struct Cups {
    next: Vec<usize>,
    current: usize,
    min: usize,
    max: usize,
}

impl FromStr for Cups {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cups: Vec<_> = s
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let current = cups[0];

        let (min, max) = cups
            .iter()
            .fold((usize::MAX, usize::MIN), |(min, max), &x| {
                (min.min(x), max.max(x))
            });

        let mut next = vec![0; max + 1];
        for window in cups.windows(2) {
            let &[p, n] = window else {
                bail!("Need at least 2 cups")
            };
            next[p] = n;
        }
        next[*cups.last().unwrap()] = current;

        Ok(Cups {
            next,
            current,
            min,
            max,
        })
    }
}

impl Cups {
    fn do_move(&mut self) {
        let a = self.next[self.current];
        let b = self.next[a];
        let c = self.next[b];
        self.next[self.current] = self.next[c];

        let mut dest = if self.current == self.min {
            self.max
        } else {
            self.current - 1
        };
        while dest == a || dest == b || dest == c {
            if dest == self.min {
                dest = self.max;
            } else {
                dest -= 1;
            }
        }

        self.next[c] = self.next[dest];
        self.next[dest] = a;

        self.current = self.next[self.current];
    }

    fn labels(&mut self) -> usize {
        let mut idx = 1;
        let mut res = 0;
        loop {
            let c = self.next[idx];
            if c == 1 {
                break;
            }
            res = res * 10 + c;
            idx = c;
        }
        res
    }

    fn enlarge_by(&mut self, to_size: usize) {
        self.next.resize(to_size + 1, 0);

        let mut c = self.current;
        while self.next[c] != self.current {
            c = self.next[c];
        }

        for n in (self.max + 1)..=to_size {
            self.next[c] = n;
            c = n;
        }
        self.next[c] = self.current;

        self.max = to_size;
    }
}
