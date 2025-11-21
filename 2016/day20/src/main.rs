use anyhow::Result;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let deny_ranges = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line?;
            let tokens: Vec<_> = line.split("-").collect();
            Ok((tokens[0].parse::<u32>()?, tokens[1].parse::<u32>()?))
        })
        .collect::<Result<Vec<_>>>()?;

    let valid_ranges: Vec<_> = deny_ranges
        .iter()
        .fold(vec![(0u32, u32::MAX)], |valid, deny| {
            valid.into_iter().flat_map(|v| v.split(deny)).collect()
        });

    println!("Part 1: {}", valid_ranges.first().unwrap().0);
    println!(
        "Part 2: {}",
        valid_ranges
            .iter()
            .map(|range| range.1 - range.0 + 1)
            .sum::<u32>()
    );

    Ok(())
}

trait Split {
    fn split(self, deny: &Self) -> Vec<Self>
    where
        Self: Sized;
}

impl Split for (u32, u32) {
    // Handle these cases respectively:
    // * Middle split: |---xxxx---| -> |---|X|---|
    // * Left split: xxxx---| -> X|---|
    // * Right split: |---xxxx -> |---|X
    // * Encompassing
    // * No split (no overlap)
    fn split(self, deny: &Self) -> Vec<Self> {
        if self.0 < deny.0 && self.1 > deny.1 {
            vec![(self.0, deny.0 - 1), (deny.1 + 1, self.1)]
        } else if deny.0 <= self.0 && deny.1 >= self.0 && deny.1 < self.1 {
            vec![(deny.1 + 1, self.1)]
        } else if deny.0 > self.0 && deny.0 <= self.1 && deny.1 >= self.1 {
            vec![(self.0, deny.0 - 1)]
        } else if deny.0 <= self.0 && deny.1 >= self.1 {
            vec![]
        } else {
            vec![self]
        }
    }
}

// I've done a problem somewhat recently where I really struggled to nail this kind of split logic.
// correct. So I was REALLY paranoid this time ;). Maybe this logic could be cleaner but this works for now...
#[cfg(test)]
mod tests {
    use crate::Split;

    #[test]
    fn test_splits() {
        // Middle
        run((0, 9), (5, 8), vec![(0, 4), (9, 9)]);
        run((1, 9), (4, 7), vec![(1, 3), (8, 9)]);
        // Left
        run((0, 9), (0, 5), vec![(6, 9)]);
        run((1, 3), (1, 2), vec![(3, 3)]);
        // Right
        run((0, 9), (5, 9), vec![(0, 4)]);
        run((2, 5), (3, 7), vec![(2, 2)]);
        // Bigger/encompassing
        run((2, 5), (0, 9), vec![]);
        run((1, 1), (0, 9), vec![]);
        run((1, 1), (1, 1), vec![]);
        // No split (no overlap) right
        run((1, 5), (8, 9), vec![(1, 5)]);
        // No split (no overlap) left
        run((8, 9), (1, 5), vec![(8, 9)]);
    }

    fn run(valid: (u32, u32), deny: (u32, u32), expected: Vec<(u32, u32)>) {
        let res = valid.split(&deny);
        assert_eq!(&expected, &res);
    }
}
