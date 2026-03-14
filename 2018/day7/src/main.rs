use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::env;
use std::io::stdin;

fn main() -> Result<()> {
    let instructions = parse_input()?;
    let workers: usize = env::args().nth(1).unwrap_or("5".to_string()).parse()?;
    let step_offset = env::args().nth(2).unwrap_or("60".to_string()).parse()?;

    println!("Part 1 : {}", instructions.order());
    println!(
        "Part 2 : {}",
        instructions.time_to_complete(workers, step_offset)
    );

    Ok(())
}

fn parse_input() -> Result<Instructions> {
    let mut all_pieces = HashSet::new();
    let mut rules = HashMap::new();
    for line in stdin().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let p1 = tokens[1].chars().next().unwrap();
        let p2 = tokens[7].chars().next().unwrap();
        all_pieces.extend([p1, p2]);
        rules
            .entry(p2)
            .and_modify(|requires: &mut Vec<_>| requires.push(p1))
            .or_insert(vec![p1]);
    }

    let mut all_pieces: Vec<_> = all_pieces.into_iter().collect();
    all_pieces.sort();

    Ok(Instructions { all_pieces, rules })
}

struct Instructions {
    all_pieces: Vec<char>,
    rules: HashMap<char, Vec<char>>,
}

impl Instructions {
    fn order(&self) -> String {
        let mut done = Vec::new();

        while let Some(next) = self
            .all_pieces
            .iter()
            .find(|&p| !done.contains(p) && self.reqs_done(*p, &done))
        {
            done.push(*next);
        }

        done.into_iter().collect()
    }

    fn time_to_complete(&self, num_workers: usize, step_offset: usize) -> usize {
        let mut done = Vec::new();
        // Store (piece, seconds left) for each worker
        let mut workers: Vec<(char, usize)> = Vec::with_capacity(num_workers);

        let mut seconds = 0;
        while done.len() != self.all_pieces.len() {
            let to_work: Vec<_> = self
                .all_pieces
                .iter()
                .filter(|&p| !done.contains(p) && self.reqs_done(*p, &done))
                .filter(|&p| {
                    workers.is_empty() || !workers.iter().any(|(work_p, _sec)| *work_p == *p)
                })
                .collect();
            for &tw in to_work.into_iter() {
                if workers.len() == num_workers {
                    break;
                }
                let time = tw as usize - 'A' as usize + step_offset + 1;
                workers.push((tw, time));
            }

            seconds += 1;
            let mut finished = Vec::new();
            workers.retain_mut(|(work_p, sec)| {
                *sec -= 1;
                if *sec == 0 {
                    finished.push(*work_p);
                }
                *sec > 0
            });
            finished.sort();
            done.extend(finished);
        }

        seconds
    }

    fn reqs_done(&self, piece: char, done: &[char]) -> bool {
        !self.rules.contains_key(&piece) || self.rules[&piece].iter().all(|req| done.contains(req))
    }
}
