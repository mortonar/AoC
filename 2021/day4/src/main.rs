use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (numbers, boards) = parse_input()?;

    println!("Part 1: {}", part1(&numbers, &boards)?);
    println!("Part 2: {}", part2(&numbers, &boards)?);

    Ok(())
}

fn parse_input() -> Result<(Vec<usize>, Vec<Board>)> {
    let mut lines = stdin().lock().lines();

    let numbers = lines.next().ok_or_else(|| anyhow!("No numbers given"))??;
    let numbers = numbers
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let _blank = lines
        .next()
        .ok_or_else(|| anyhow!("Expected blank line"))??;

    let mut boards = Vec::new();
    let mut spaces = Vec::new();
    for line in lines {
        let line = line?;

        if line.trim().is_empty() {
            boards.push(Board { spaces });
            spaces = Vec::new();
            continue;
        }

        let line = line
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        spaces.push(line.into_iter().map(|num| (num, false)).collect());
    }
    boards.push(Board { spaces });

    Ok((numbers, boards))
}

fn part1(numbers: &[usize], boards: &[Board]) -> Result<usize> {
    let mut boards = boards.to_vec();

    for &number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.check_bingo() {
                return Ok(board.score(number));
            }
        }
    }

    Err(anyhow!("No first winning board found"))
}

fn part2(numbers: &[usize], boards: &[Board]) -> Result<usize> {
    let mut boards = boards.to_vec();
    let len = boards.len();

    let mut winners = HashSet::new();
    for &number in numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            board.mark(number);

            if board.check_bingo() {
                winners.insert(idx);

                if winners.len() == len {
                    return Ok(board.score(number));
                }
            }
        }
    }

    Err(anyhow!("No last winning board found"))
}

#[derive(Debug, Clone)]
struct Board {
    spaces: Vec<Vec<(usize, bool)>>,
}

impl Board {
    fn mark(&mut self, number: usize) {
        self.spaces.iter_mut().flatten().for_each(|(n, marked)| {
            if *n == number {
                *marked = true
            }
        });
    }

    fn check_bingo(&self) -> bool {
        if self
            .spaces
            .iter()
            .any(|row| row.iter().all(|&(_, marked)| marked))
        {
            return true;
        }

        for col in 0..self.spaces[0].len() {
            if self.spaces.iter().all(|row| row[col].1) {
                return true;
            }
        }

        false
    }

    fn score(&self, last_num: usize) -> usize {
        self.spaces
            .iter()
            .flatten()
            .filter_map(|&(n, m)| if !m { Some(n) } else { None })
            .sum::<usize>()
            * last_num
    }
}
