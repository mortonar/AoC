use anyhow::{Error, Result, bail};
use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() -> Result<()> {
    let mut adapters = parse_input()?;
    sort_chain(&mut adapters);

    let diffs = jolt_differences(&adapters)?;
    println!("Part 1: {}", diffs[1] * diffs[3]);

    let arrangements = arrangements_dfs_memo(&adapters, 0, &mut HashMap::default());
    println!("Part 2: {arrangements}");

    Ok(())
}

fn parse_input() -> Result<Vec<usize>> {
    stdin()
        .lock()
        .lines()
        .map(|l| l?.parse().map_err(Error::from))
        .collect()
}

/// Sort adapters and append outlet (0) and device (max(adapters) + 3) joltages
fn sort_chain(adapters: &mut Vec<usize>) {
    adapters.sort_unstable();
    adapters.insert(0, 0);
    let built_in = adapters.last().unwrap() + 3;
    adapters.push(built_in);
}

fn jolt_differences(adapters: &[usize]) -> Result<[usize; 4]> {
    let mut diffs = [0; 4];
    for pair in adapters.windows(2) {
        let diff = pair[1] - pair[0];
        if diff > 3 {
            bail!(
                "Difference {} too large from {} and {}",
                diff,
                pair[0],
                pair[1]
            );
        }
        diffs[diff] += 1;
    }

    Ok(diffs)
}

/// Recursive DFS through the search tree of all arrangements of adapters with memoization:
/// * current - index into adapters (node in tree)
/// * memo - adapter index (node) -> # of arrangements (reachable leaves from that node)
fn arrangements_dfs_memo(
    adapters: &[usize],
    current: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if current == adapters.len() - 1 {
        return 1;
    }

    if let Some(&cached) = memo.get(&current) {
        return cached;
    }

    let mut arrangements = 0;
    for i in current + 1..adapters.len() {
        if adapters[i] - adapters[current] > 3 {
            break;
        }
        arrangements += arrangements_dfs_memo(adapters, i, memo);
    }
    memo.insert(current, arrangements);
    arrangements
}
