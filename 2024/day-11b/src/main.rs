use anyhow::Result;
use std::collections::HashMap;
use std::io::stdin;

fn main() -> Result<()> {
    let mut stones = String::new();
    stdin().read_line(&mut stones)?;
    let stones: Vec<u64> = stones
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse())
        .flatten()
        .collect();

    let mut memo: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    let total: u64 = stones.iter().map(|s| blink(75, *s, &mut memo)).sum();
    println!("{}", total);

    Ok(())
}

// DFS + memoization
fn blink(times: u64, stone: u64, memo: &mut HashMap<u64, HashMap<u64, u64>>) -> u64 {
    if memo.contains_key(&stone) && memo[&stone].contains_key(&times) {
        return memo[&stone][&times];
    } else if times == 0 {
        return 1;
    }

    let mut total = 0;
    if stone == 0 {
        total += blink(times - 1, 1, memo);
    } else if stone.to_string().len() % 2 == 0 {
        let stone = stone.to_string();
        total += blink(times - 1, stone[..stone.len() / 2].parse().unwrap(), memo);
        total += blink(times - 1, stone[(stone.len() / 2)..].parse().unwrap(), memo);
    } else {
        total += blink(times - 1, stone * 2024, memo);
    }
    memo.entry(stone)
        .and_modify(|m| {
            m.insert(times, total);
        })
        .or_insert(HashMap::from([(times, total)]));
    total
}
