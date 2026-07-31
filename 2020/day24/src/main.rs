use anyhow::Result;
use std::io::{BufRead, stdin};
use std::str::Chars;

const GRID_SIZE: usize = 1_000;
const MIDDLE: (isize, isize) = (GRID_SIZE as isize / 2, GRID_SIZE as isize / 2);

fn main() -> Result<()> {
    let tile_directions = parse_input()?;

    let mut tiles = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    println!("Part 1: {}", part1(&tile_directions, &mut tiles));
    println!("Part 2: {}", part2(tiles));

    Ok(())
}

fn parse_input() -> Result<Vec<Vec<Direction>>> {
    let mut all_directions = Vec::new();

    for line in stdin().lock().lines() {
        let line = line?;
        let mut chars = line.trim().chars();

        let mut directions = Vec::new();
        while let Some(dir) = Direction::parse_dir(&mut chars) {
            directions.push(dir);
        }
        all_directions.push(directions);
    }

    Ok(all_directions)
}

fn part1(tile_directions: &[Vec<Direction>], tiles: &mut [Vec<bool>]) -> usize {
    for directions in tile_directions.iter() {
        let (mut x, mut y) = MIDDLE;
        directions
            .iter()
            .for_each(|dir| dir.apply((&mut x, &mut y)));

        assert!(x >= 0 && y >= 0 && x < GRID_SIZE as isize && y < GRID_SIZE as isize);
        let (x, y) = (x as usize, y as usize);
        tiles[x][y] = !tiles[x][y];
    }

    tiles.iter().flatten().filter(|&&v| v).count()
}

fn part2(mut tiles: Vec<Vec<bool>>) -> usize {
    for _ in 0..100 {
        let mut new = tiles.clone();
        for (x, row) in tiles.iter().enumerate() {
            for (y, &t) in row.iter().enumerate() {
                let adj = adjacent_black(x, y, &tiles);
                if (t && (adj == 0 || adj > 2)) || (!t && adj == 2) {
                    new[x][y] = !new[x][y];
                }
            }
        }
        tiles = new;
    }

    tiles.iter().flatten().filter(|&&v| v).count()
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn parse_dir(chars: &mut Chars) -> Option<Self> {
        let c = chars.next()?;
        match c {
            'e' => Some(Direction::East),
            'w' => Some(Direction::West),
            's' => match chars.next()? {
                'w' => Some(Direction::SouthWest),
                'e' => Some(Direction::SouthEast),
                _ => None,
            },
            'n' => match chars.next()? {
                'w' => Some(Direction::NorthWest),
                'e' => Some(Direction::NorthEast),
                _ => None,
            },
            _ => None,
        }
    }

    fn apply(&self, (x, y): (&mut isize, &mut isize)) {
        match self {
            Direction::East => *x += 1,
            Direction::SouthEast => {
                *y += 1;
            }
            Direction::SouthWest => {
                *x -= 1;
                *y += 1;
            }
            Direction::West => *x -= 1,
            Direction::NorthWest => {
                *y -= 1;
            }
            Direction::NorthEast => {
                *x += 1;
                *y -= 1;
            }
        }
    }
}

fn adjacent_black(x: usize, y: usize, tiles: &[Vec<bool>]) -> usize {
    let (x, y) = (x as isize, y as isize);
    [
        (x + 1, y),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x, y - 1),
        (x + 1, y - 1),
    ]
    .into_iter()
    .filter_map(|(x, y)| {
        if x >= 0 && y >= 0 && x < GRID_SIZE as isize && y < GRID_SIZE as isize {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
    .filter(|&(x, y)| tiles[x][y])
    .count()
}
