use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (p1, p2) = parse_input()?;

    println!("Part 1: {}", part1(p1, p2));
    println!("Part 2: {}", part2(p1, p2));

    Ok(())
}

fn parse_input() -> Result<(usize, usize)> {
    let positions = stdin()
        .lock()
        .lines()
        .take(2)
        .map(|l| {
            let line = l?;
            let token = line
                .split_ascii_whitespace()
                .last()
                .ok_or_else(|| anyhow!("Missing position"))?;
            Ok(token.parse::<usize>()?)
        })
        .collect::<Result<Vec<_>>>()?;

    match positions[..] {
        [p1, p2] => Ok((p1, p2)),
        _ => Err(anyhow!(
            "Expected 2 starting positions, got {}",
            positions.len()
        )),
    }
}

fn part1(p1: usize, p2: usize) -> usize {
    play(p1, p2)
}

fn part2(p1: usize, p2: usize) -> u64 {
    let (w1, w2) = dfs([Player::new(p1), Player::new(p2)], &mut HashMap::new());
    w1.max(w2)
}

fn play(p1: usize, p2: usize) -> usize {
    let mut die = DeterministicDie::new(100);
    let mut players = [Player::new(p1), Player::new(p2)];

    'outer: loop {
        for p in players.iter_mut() {
            let spaces: usize = die.by_ref().take(3).sum();
            p.forward(spaces);
            if p.score >= 1000 {
                break 'outer;
            }
        }
    }

    let loosing = if players[0].score >= 1000 {
        players[1].score
    } else {
        players[0].score
    };
    loosing * die.rolled
}

const ROLL_FREQS: [(usize, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn dfs(players: [Player; 2], memo: &mut HashMap<[Player; 2], (u64, u64)>) -> (u64, u64) {
    if let Some(&cached) = memo.get(&players) {
        return cached;
    }

    let mut wins = (0, 0);
    for (spaces, freq) in ROLL_FREQS {
        let mut mover = players[0].clone();
        mover.forward(spaces);

        if mover.score >= 21 {
            wins.0 += freq;
        } else {
            let (opponent_wins, mover_wins) = dfs([players[1].clone(), mover], memo);
            wins.0 += freq * mover_wins;
            wins.1 += freq * opponent_wins;
        }
    }

    memo.insert(players, wins);
    wins
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        let score = 0;
        Self { position, score }
    }

    fn forward(&mut self, spaces: usize) {
        self.position = (self.position - 1 + spaces) % 10 + 1;
        self.score += self.position;
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct DeterministicDie {
    sides: usize,
    value: usize,
    rolled: usize,
}

impl DeterministicDie {
    fn new(sides: usize) -> Self {
        let value = 1;
        let rolled = 0;
        Self {
            sides,
            value,
            rolled,
        }
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = Some(self.value);
        self.rolled += 1;
        self.value += 1;
        if self.value > self.sides {
            self.value = 1;
        }
        ret
    }
}
