use anyhow::Result;
use std::env::args;
use std::io::stdin;

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    let size = parse_size()?;

    let lengths: Vec<_> = line
        .split(",")
        .map(|l| l.trim().parse().map_err(anyhow::Error::from))
        .collect::<Result<_>>()?;
    let mut list: Vec<_> = (0..size).collect();
    KnotHasher::default().hash(&mut list, &lengths);
    println!("Part 1: {}", list.iter().take(2).product::<usize>());

    let mut list: Vec<_> = (0..size).collect();
    let mut lengths: Vec<_> = line.trim().chars().map(|c| c as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut hasher = KnotHasher::default();
    for _ in 0..64 {
        hasher.hash(&mut list, &lengths);
    }
    let reduced = KnotHasher::reduce(&list);
    let formatted: String = reduced.iter().map(|r| format!("{:02x}", r)).collect();
    println!("Part 2: {}", &formatted);

    Ok(())
}

fn parse_size() -> Result<usize> {
    args()
        .nth(1)
        .unwrap_or("256".to_string())
        .trim()
        .parse::<usize>()
        .map_err(anyhow::Error::from)
}

#[derive(Default)]
struct KnotHasher {
    current: usize,
    skip: usize,
}

impl KnotHasher {
    fn hash(&mut self, list: &mut [usize], lengths: &[usize]) {
        for &length in lengths.iter() {
            let rev: Vec<_> = (0..length)
                .map(|i| list[(self.current + i) % list.len()])
                .rev()
                .collect();
            (0..length).for_each(|i| list[(self.current + i) % list.len()] = rev[i]);
            self.current += length + self.skip;
            self.skip += 1;
        }
    }

    fn reduce(list: &[usize]) -> Vec<usize> {
        list.chunks(16)
            .map(|block| block.iter().fold(0, |acc, &n| acc ^ n))
            .collect()
    }
}
