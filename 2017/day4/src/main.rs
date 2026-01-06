use anyhow::Result;
use std::collections::HashSet;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let lines = stdin().lock().lines().collect::<Result<Vec<_>, _>>()?;
    let part1 = lines.iter().filter(|l| l.is_valid()).count();
    let part2 = lines.iter().filter(|l| l.is_anagram_valid()).count();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    Ok(())
}

trait Passphrase {
    fn is_valid(&self) -> bool;
    fn is_anagram_valid(&self) -> bool;
}

impl<T: AsRef<str>> Passphrase for T {
    /// Check if all words in the passphrase can be uniquely inserted into a set.
    fn is_valid(&self) -> bool {
        let mut words = HashSet::new();
        self.as_ref()
            .split_ascii_whitespace()
            .all(|w| words.insert(w))
    }

    /// Check if all words in the passphrase can be uniquely inserted into a set, representing each
    /// word as the sorted list of its composite characters.
    fn is_anagram_valid(&self) -> bool {
        let mut words = HashSet::new();
        self.as_ref()
            .split_ascii_whitespace()
            .map(|w| {
                let mut chars: Vec<_> = w.chars().collect();
                chars.sort();
                chars
            })
            .all(|chars| words.insert(chars))
    }
}
