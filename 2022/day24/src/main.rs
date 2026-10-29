use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, stdin},
};

use anyhow::{Result, bail};
use num_integer::lcm;

fn main() -> Result<()> {
    let (valley, blizzards) = parse_input()?;

    println!("Part 1: {}", part1(&valley, &blizzards)?);
    println!("Part 2: {}", part2(&valley, &blizzards)?);

    Ok(())
}

fn parse_input() -> Result<(Valley, Blizzards)> {
    let start = (0, 1);
    let (mut max_x, mut max_y) = (0, 0);
    let mut blizzard_locs: Blizzards = HashMap::new();

    for (x, line) in stdin().lock().lines().enumerate() {
        let line = line?;

        (max_x, max_y) = (max_x.max(x), max_y.max(line.len() - 1));

        for (y, c) in line.chars().enumerate() {
            if matches!(c, '<' | '^' | '>' | 'v') {
                blizzard_locs
                    .entry((x, y))
                    .and_modify(|list| list.push(c))
                    .or_insert(vec![c]);
            }
        }
    }

    let end = (max_x, max_y - 1);

    Ok((
        Valley {
            start,
            end,
            max_x,
            max_y,
        },
        blizzard_locs,
    ))
}

fn part1(valley: &Valley, blizzards: &Blizzards) -> Result<usize> {
    valley.min_bfs(blizzards, valley.start, valley.end, 0)
}

fn part2(valley: &Valley, blizzards: &Blizzards) -> Result<usize> {
    let t1 = valley.min_bfs(blizzards, valley.start, valley.end, 0)?;
    let t2 = valley.min_bfs(blizzards, valley.end, valley.start, t1)?;
    valley.min_bfs(blizzards, valley.start, valley.end, t2)
}

#[derive(Debug)]
struct Valley {
    start: (usize, usize),
    end: (usize, usize),
    max_x: usize,
    max_y: usize,
}

type Blizzards = HashMap<(usize, usize), Vec<char>>;

impl Valley {
    /// Earliest arrival time at `to`, leaving `from` at `start_time`
    fn min_bfs(
        &self,
        blizzards: &Blizzards,
        from: (usize, usize),
        to: (usize, usize),
        start_time: usize,
    ) -> Result<usize> {
        // Blizzards are predictable and repeat every period
        let period = lcm(self.max_x - 1, self.max_y - 1);

        let (x, y) = from;
        let mut queue = VecDeque::from([(x, y, start_time)]);
        let mut visited = HashSet::from([(x, y, start_time % period)]);

        while let Some((cx, cy, ct)) = queue.pop_front() {
            if (cx, cy) == to {
                return Ok(ct);
            }

            let nt = ct + 1;
            let moves = [
                Some((cx, cy)), // wait
                cx.checked_sub(1).map(|x| (x, cy)),
                Some((cx + 1, cy)),
                cy.checked_sub(1).map(|y| (cx, y)),
                Some((cx, cy + 1)),
            ];

            for (nx, ny) in moves.into_iter().flatten() {
                if self.is_free(blizzards, (nx, ny), nt) && visited.insert((nx, ny, nt % period)) {
                    queue.push_back((nx, ny, nt));
                }
            }
        }

        bail!("path not found")
    }

    /// Can we stand on `pos` at time `t`?
    fn is_free(&self, blizzards: &Blizzards, pos: (usize, usize), t: usize) -> bool {
        // blizzards never reach these
        if pos == self.start || pos == self.end {
            return true;
        }

        let (x, y) = pos;
        (1..self.max_x).contains(&x)
            && (1..self.max_y).contains(&y)
            && !self.has_blizzard(blizzards, pos, t)
    }

    /// Is any blizzard on inner cell `(x, y)` at time `t`?
    fn has_blizzard(&self, blizzards: &Blizzards, (x, y): (usize, usize), t: usize) -> bool {
        // grid -> inner coords for wrapping (e.g. leftmost inner cell past the wall is now 0)
        let (h, w) = (self.max_x as i64 - 1, self.max_y as i64 - 1);
        let (r, c, t) = (x as i64 - 1, y as i64 - 1, t as i64);
        // wrap, then +1 to translate back to the original given coordinate system of the blizzards
        let col = |c: i64| 1 + c.rem_euclid(w) as usize;
        let row = |r: i64| 1 + r.rem_euclid(h) as usize;

        let has = |pos, d| {
            blizzards
                .get(&pos)
                .is_some_and(|v: &Vec<char>| v.contains(&d))
        };
        has((x, col(c - t)), '>')
            || has((x, col(c + t)), '<')
            || has((row(r - t), y), 'v')
            || has((row(r + t), y), '^')
    }
}
