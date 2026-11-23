use std::{
    collections::HashMap,
    io::{BufRead, stdin},
};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let rows = parse_input()?;

    println!("Part 1: {}", part1(&rows));
    println!("Part 2: {}", part2(&rows));

    Ok(())
}

type Row = (Springs, Checksum);
type Springs = Vec<char>;
type Checksum = Vec<usize>;

fn parse_input() -> Result<Vec<Row>> {
    stdin().lock().lines().map(|l| parse_row(&l?)).collect()
}

fn parse_row(line: &str) -> Result<Row> {
    let Some((springs, checksum)) = line.split_once(' ') else {
        bail!("ill-formatted row: {line}");
    };
    Ok((parse_springs(springs)?, parse_checksum(checksum)?))
}

fn parse_springs(s: &str) -> Result<Springs> {
    let line = s.trim();

    if line.is_empty() {
        bail!("row requires at least one character");
    }
    if line.chars().any(|c| !matches!(c, '#' | '.' | '?')) {
        bail!("row contains invalid character: {line}");
    }

    Ok(line.chars().collect())
}

fn parse_checksum(s: &str) -> Result<Checksum> {
    s.split(',').map(|n| Ok(n.parse()?)).collect()
}

fn part1(rows: &[Row]) -> usize {
    rows.iter()
        .map(|(springs, checksum)| arrangements_dfs(springs, checksum, &mut HashMap::new()))
        .sum()
}

fn part2(rows: &[Row]) -> usize {
    rows.iter()
        .map(unfold)
        .map(|(springs, checksum)| arrangements_dfs(&springs, &checksum, &mut HashMap::new()))
        .sum()
}

fn unfold(row: &Row) -> Row {
    let (springs, checksum) = row;
    let springs: Vec<char> = vec![springs.iter().collect::<String>(); 5]
        .join("?")
        .chars()
        .collect();
    let checksum = checksum.repeat(5);
    (springs, checksum)
}

fn arrangements_dfs(
    springs: &[char],
    checksum: &[usize],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&cached) = memo.get(&(springs.len(), checksum.len())) {
        return cached;
    }

    match (springs.is_empty(), checksum.is_empty()) {
        (true, true) => return 1,
        (true, false) => return 0,
        _ => {}
    }

    let mut arrangements = 0;
    if matches!(springs[0], '.' | '?') {
        arrangements += arrangements_dfs(&springs[1..], checksum, memo);
    }
    if !checksum.is_empty()
        && springs.len() >= checksum[0]
        && springs
            .iter()
            .take_while(|&s| matches!(s, '#' | '?'))
            .count()
            >= checksum[0]
        // The spring after this group can't be damaged, or the group would be longer
        && springs.get(checksum[0]) != Some(&'#')
    {
        // +1 to leave a gap after this placement
        let rest = springs.get(checksum[0] + 1..).unwrap_or(&[]);
        arrangements += arrangements_dfs(rest, &checksum[1..], memo);
    }

    memo.insert((springs.len(), checksum.len()), arrangements);

    arrangements
}
