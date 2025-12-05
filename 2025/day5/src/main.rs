use anyhow::Result;
use std::cmp::{max, min};
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let (ranges, ingredients) = parse_input()?;

    let fresh: usize = ingredients
        .iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count();
    println!("Part 1: {fresh}");

    let ranges = ranges.merge();
    let fresh: usize = ranges.iter().map(|r| r.1 - r.0 + 1).sum();
    println!("Part 1: {fresh}");

    Ok(())
}

#[allow(clippy::type_complexity)]
fn parse_input() -> Result<(Vec<(usize, usize)>, Vec<usize>)> {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut lines = stdin().lock().lines();

    for line in lines.by_ref() {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }
        ranges.push(parse_range(&line)?);
    }

    for line in lines.by_ref() {
        let line = line?;
        ids.push(line.trim().parse()?);
    }

    Ok((ranges, ids))
}

fn parse_range(line: &str) -> Result<(usize, usize)> {
    let tokens: Vec<_> = line.split('-').collect();
    Ok((tokens[0].parse()?, tokens[1].parse()?))
}

// Custom Range trait since RangeInclusive isn't orderable/sortable
trait Range<T> {
    fn contains(&self, n: &T) -> bool;
    fn intersect(&self, other: &Self) -> bool;
}

impl Range<usize> for (usize, usize) {
    fn contains(&self, n: &usize) -> bool {
        *n >= self.0 && *n <= self.1
    }

    fn intersect(&self, other: &Self) -> bool {
        (self.0 >= other.0 && self.0 <= other.1)
            || (self.1 >= other.0 && self.1 <= other.1)
            || (other.0 >= self.0 && other.0 <= self.1)
            || (other.1 >= self.0 && other.1 <= self.1)
    }
}

// Add ability to sort then merge to eliminate range overlaps/redundancies
trait Merge {
    fn merge(self) -> Self;
}

impl Merge for Vec<(usize, usize)> {
    fn merge(mut self) -> Self {
        // Don't bother "merging" an empty Vec or a Vec of 1 element
        if self.len() < 2 {
            return self;
        }

        self.sort();
        let mut new = Vec::new();
        let mut iter = self.into_iter();
        let mut prev = iter.next().unwrap();
        for next in iter {
            if prev.intersect(&next) {
                prev = (min(prev.0, next.0), max(prev.1, next.1));
            } else {
                new.push(prev);
                prev = next;
            }
        }
        new.push(prev);
        new
    }
}
