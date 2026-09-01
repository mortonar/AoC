use anyhow::{Error, Result};
use std::io::{BufRead, stdin};
use std::str::FromStr;

fn main() -> Result<()> {
    let snail_nums = parse_input()?;

    println!("Part 1: {}", part1(&snail_nums));
    println!("Part 2: {}", part2(&snail_nums));

    Ok(())
}

fn parse_input() -> Result<Vec<SnailNum>> {
    stdin().lock().lines().map(|l| l?.parse()).collect()
}

fn part1(snail_nums: &[SnailNum]) -> usize {
    snail_nums[1..]
        .iter()
        .fold(snail_nums[0].clone(), |acc, sn| acc.add(sn).reduce())
        .magnitude()
}

fn part2(snail_nums: &[SnailNum]) -> usize {
    let mut max = 0;
    for s1 in snail_nums {
        for s2 in snail_nums {
            max = max.max(s1.clone().add(s2).reduce().magnitude());
        }
    }
    max
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SnailNum {
    left: Num,
    right: Num,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Num {
    Val(usize),
    SnailNum(Box<SnailNum>),
}

impl FromStr for SnailNum {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (rem, num) = parse_num(s.trim())?;
        if !rem.is_empty() {
            anyhow::bail!("unexpected trailing input: {rem}");
        }

        match num {
            Num::SnailNum(sn) => Ok(*sn),
            Num::Val(_) => anyhow::bail!("top-level snail number must be a pair"),
        }
    }
}

fn parse_num(s: &str) -> Result<(&str, Num)> {
    if let Some(rem) = s.strip_prefix('[') {
        let (rem, left) = parse_num(rem)?;
        let rem = rem
            .strip_prefix(',')
            .ok_or_else(|| anyhow::anyhow!("expected ','"))?;

        let (rem, right) = parse_num(rem)?;
        let rem = rem
            .strip_prefix(']')
            .ok_or_else(|| anyhow::anyhow!("expected ']'"))?;

        return Ok((rem, Num::SnailNum(Box::new(SnailNum { left, right }))));
    }

    let len = s.bytes().take_while(|b| b.is_ascii_digit()).count();
    if len == 0 {
        anyhow::bail!("expected number");
    }

    let (num, rem) = s.split_at(len);
    Ok((rem, Num::Val(num.parse()?)))
}

impl SnailNum {
    fn reduce(mut self) -> Self {
        while self.explode() || self.split() {}
        self
    }

    fn explode(&mut self) -> bool {
        if let Some((_, r_carry)) = self.left.explode(1) {
            self.right.add_leftmost(r_carry);
            return true;
        }

        if let Some((l_carry, _)) = self.right.explode(1) {
            self.left.add_rightmost(l_carry);
            return true;
        }

        false
    }

    fn split(&mut self) -> bool {
        self.left.split() || self.right.split()
    }

    fn add(self, other: &Self) -> Self {
        SnailNum {
            left: Num::SnailNum(Box::new(self)),
            right: Num::SnailNum(Box::new(other.clone())),
        }
    }

    fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

impl Num {
    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        let Num::SnailNum(pair) = self else {
            return None;
        };

        if depth == 4 {
            let (Num::Val(l), Num::Val(r)) = (&pair.left, &pair.right) else {
                unreachable!("Pair at depth 4 contains a nested pair");
            };

            let carries = (*l, *r);
            *self = Num::Val(0);
            return Some(carries);
        }

        if let Some((lcarry, rcarry)) = pair.left.explode(depth + 1) {
            pair.right.add_leftmost(rcarry);
            return Some((lcarry, 0));
        }

        if let Some((lcarry, rcarry)) = pair.right.explode(depth + 1) {
            pair.left.add_rightmost(lcarry);
            return Some((0, rcarry));
        }

        None
    }

    fn add_leftmost(&mut self, n: usize) {
        match self {
            Num::Val(v) => *v += n,
            Num::SnailNum(pair) => pair.left.add_leftmost(n),
        }
    }

    fn add_rightmost(&mut self, n: usize) {
        match self {
            Num::Val(v) => *v += n,
            Num::SnailNum(pair) => pair.right.add_rightmost(n),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Num::Val(v) if *v >= 10 => {
                let left = *v / 2;
                let right = *v - left;
                let (left, right) = (Num::Val(left), Num::Val(right));
                *self = Num::SnailNum(Box::new(SnailNum { left, right }));
                true
            }
            Num::SnailNum(sn) => sn.left.split() || sn.right.split(),
            _ => false,
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Num::Val(v) => *v,
            Num::SnailNum(sn) => sn.magnitude(),
        }
    }
}
