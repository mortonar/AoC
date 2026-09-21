use std::io::{BufRead, stdin};

use anyhow::{Result, anyhow, bail};

fn main() -> Result<()> {
    let (stacks, moves) = parse_input()?;

    println!("Part 1: {}", part1(&stacks, &moves));
    println!("Part 2: {}", part2(&stacks, &moves));

    Ok(())
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn part1(stacks: &[Vec<char>], moves: &[Move]) -> String {
    do_moves(stacks, moves, false)
}

fn part2(stacks: &[Vec<char>], moves: &[Move]) -> String {
    do_moves(stacks, moves, true)
}

fn do_moves(stacks: &[Vec<char>], moves: &[Move], multi: bool) -> String {
    let mut stacks = stacks.to_owned();
    moves.iter().for_each(|m| stacks.do_move(m, multi));
    stacks.top_crates()
}

trait Stacks {
    fn do_move(&mut self, m: &Move, multi: bool);
    fn top_crates(&self) -> String;
}

impl Stacks for Vec<Vec<char>> {
    fn do_move(&mut self, m: &Move, multi: bool) {
        let &Move { amount, from, to } = m;
        let len = self[from].len();
        let mut to_move: Vec<_> = self[from].drain(len - amount..).collect();
        // Simulate moving 1 at a time
        if !multi {
            to_move.reverse();
        }
        self[to].extend(to_move);
    }

    fn top_crates(&self) -> String {
        self.iter().filter_map(|s| s.last()).collect()
    }
}

fn parse_input() -> Result<(Vec<Vec<char>>, Vec<Move>)> {
    let mut lines = stdin().lock().lines().peekable();
    let Some(line) = lines.peek() else {
        bail!("missing input");
    };
    let num_stacks = match line {
        // +1 for trimmed newline
        Ok(line) => (line.chars().count() + 1) / 4,
        Err(e) => bail!("{e}"),
    };
    let mut stacks = vec![vec![]; num_stacks];

    for line in lines.by_ref() {
        let line = line?;
        if !line.contains('[') {
            break;
        }

        let chars: Vec<_> = line.chars().collect();
        for (stack, chunk) in chars.chunks(4).enumerate() {
            let c = chunk[1];
            if c != ' ' {
                stacks[stack].push(c);
            }
        }
    }

    // Stacks are parsed top down but should be stored bottom up
    stacks.iter_mut().for_each(|s| s.reverse());

    let _blank = lines
        .next()
        .ok_or_else(|| anyhow!("missing blank line"))??;

    let mut moves = Vec::new();
    for line in lines {
        let line = line?;
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let (amount, from, to) = (
            tokens[1].parse::<usize>()?,
            tokens[3].parse::<usize>()?,
            tokens[5].parse::<usize>()?,
        );
        // Crates are 1-indexed
        let (from, to) = (from - 1, to - 1);
        moves.push(Move { amount, from, to })
    }

    Ok((stacks, moves))
}
