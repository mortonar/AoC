use anyhow::{Result, anyhow};
use std::collections::VecDeque;
use std::io::stdin;

fn main() -> Result<()> {
    let key = stdin()
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No key provided"))??;
    let cells: Vec<_> = (0..128)
        .map(|suffix| KnotHasher::default().hash(format!("{key}-{suffix}")))
        .map(|bin_str| bin_str.chars().map(Cell::from).collect::<Vec<_>>())
        .collect();
    let used = cells
        .iter()
        .flat_map(|cells| cells.iter())
        .filter(|c| c.used)
        .count();
    println!("Part 1: {used}");

    let mut grid = Grid::new(cells);
    println!("Part 2: {}", grid.count_regions());

    Ok(())
}

struct Cell {
    used: bool,
    // 0 == unmarked, > 0 == marked
    region: usize,
}

impl Cell {
    fn fillable(&self) -> bool {
        self.used && self.region == 0
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self {
            used: value == '1',
            region: 0,
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Self {
        Self { cells }
    }

    fn count_regions(&mut self) -> usize {
        let mut region_label = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j].fillable() {
                    region_label += 1;
                    self.flood_fill((i, j), region_label);
                }
            }
        }
        region_label
    }

    fn flood_fill(&mut self, start: (usize, usize), label: usize) {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(next) = queue.pop_front() {
            self.cells[next.0][next.1].region = label;

            for &(di, dj) in ORIENTATIONS.iter() {
                let (i, j) = (next.0 as isize + di, next.1 as isize + dj);
                if i < 0 || j < 0 {
                    continue;
                }
                let (i, j) = (i as usize, j as usize);
                if i < self.cells.len() && j < self.cells[i].len() && self.cells[i][j].fillable() {
                    queue.push_back((i, j));
                }
            }
        }
    }
}

const ORIENTATIONS: [&(isize, isize); 4] = [&(-1, 0), &(1, 0), &(0, -1), &(0, 1)];

// Borrowed from day10 with minor tweaks
#[derive(Default)]
struct KnotHasher {
    current: usize,
    skip: usize,
}

impl KnotHasher {
    fn hash<T: AsRef<str>>(&mut self, data: T) -> String {
        let mut list: Vec<_> = (0..256).collect();
        let mut lengths: Vec<_> = data.as_ref().trim().chars().map(|c| c as usize).collect();
        lengths.append(&mut vec![17, 31, 73, 47, 23]);
        for _ in 0..64 {
            self.hash_inner(&mut list, &lengths);
        }
        KnotHasher::reduce(&list)
            .into_iter()
            .map(|r| format!("{:02x}", r))
            .flat_map(|hex| {
                hex.chars()
                    .map(|h| format!("{:04b}", h.to_digit(16).unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn hash_inner(&mut self, list: &mut [usize], lengths: &[usize]) {
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
