use std::io::{BufRead, stdin};

use anyhow::{Result, bail};

fn main() -> Result<()> {
    let rocks = parse_input()?;

    println!("Part 1: {}", part1(&rocks));
    println!("Part 2: {}", part2(&rocks));

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<(usize, usize)>>> {
    let mut rocks = Vec::new();
    for line in stdin().lock().lines() {
        let line = line?;
        let segments: Vec<_> = line
            .split(" -> ")
            .map(parse_segment)
            .collect::<Result<Vec<_>>>()?;
        rocks.push(segments);
    }
    Ok(rocks)
}

fn parse_segment(s: &str) -> Result<(usize, usize)> {
    let Some((y, x)) = s.split_once(',') else {
        bail!("ill-formatted segment: {s}");
    };
    Ok((y.parse()?, x.parse()?))
}

fn part1(rocks: &[Vec<(usize, usize)>]) -> usize {
    let mut cave = Cave::from_rocks(rocks, false);
    let mut units = 0;
    while cave.trickle_unit().is_some() {
        units += 1;
    }
    units
}

fn part2(rocks: &[Vec<(usize, usize)>]) -> usize {
    let mut cave = Cave::from_rocks(rocks, true);
    let mut units = 0;
    while let Some((y, x)) = cave.trickle_unit() {
        units += 1;
        if (y, x) == (500, 0) {
            break;
        }
    }
    units
}

#[derive(Debug, Clone)]
struct Cave {
    cells: Vec<Vec<Cell>>,
    max_x: usize,
}

#[derive(Debug, Clone)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Cell {
    fn is_open(&self) -> bool {
        matches!(self, Cell::Air)
    }
}

impl Cave {
    fn from_rocks(rocks: &[Vec<(usize, usize)>], floor: bool) -> Self {
        let (mut max_y, mut max_x) = rocks
            .iter()
            .flat_map(|r| r.iter())
            .fold((0, 0), |(my, mx), &(y, x)| (my.max(y), mx.max(x)));

        if floor {
            max_x += 2;
            max_y = max_y.max(500 + max_x);
        }

        let mut cells = vec![vec![Cell::Air; max_y + 1]; max_x + 1];

        for rock in rocks.iter() {
            for from_to in rock.windows(2) {
                let ((fy, fx), (ty, tx)) = (from_to[0], from_to[1]);
                let ((mut fy, mut fx), (ty, tx)) =
                    ((fy as isize, fx as isize), (ty as isize, tx as isize));

                let diff = ((ty - fy).signum(), (tx - fx).signum());

                loop {
                    cells[fx as usize][fy as usize] = Cell::Rock;

                    if (fy, fx) == (ty, tx) {
                        break;
                    }

                    fy += diff.0;
                    fx += diff.1;
                }
            }
        }

        if floor {
            cells
                .last_mut()
                .unwrap()
                .iter_mut()
                .for_each(|c| *c = Cell::Rock);
        }

        Self { cells, max_x }
    }

    fn trickle_unit(&mut self) -> Option<(usize, usize)> {
        let (mut y, mut x) = (500, 0);
        let (dy, dx) = (0, 1);

        loop {
            if x == self.max_x {
                return None;
            }

            let (ny, nx) = (y + dy as usize, x + dx as usize);
            match self.cells[nx][ny] {
                Cell::Air => (y, x) = (ny, nx),
                _ => {
                    if self.cells[nx][ny - 1].is_open() {
                        (y, x) = (ny - 1, nx);
                    } else if self.cells[nx][ny + 1].is_open() {
                        (y, x) = (ny + 1, nx);
                    } else {
                        self.cells[x][y] = Cell::Sand;
                        return Some((y, x));
                    }
                }
            }
        }
    }
}
