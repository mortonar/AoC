use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut lab: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut current = lab
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == '^').map(|j| (i, j)))
        .unwrap();

    let mut direction = Direction::Up;
    loop {
        lab[current.0][current.1] = 'X';

        let offset = direction.offset();
        let next = (current.0 as isize + offset.0, current.1 as isize + offset.1);
        if next.0 < 0
            || next.0 > lab.len() as isize - 1
            || next.1 < 0
            || next.1 > lab[0].len() as isize - 1
        {
            break;
        }
        let next = (next.0 as usize, next.1 as usize);

        match lab[next.0][next.1] {
            '#' => direction = direction.turn_right(),
            _ => {
                current = next;
            }
        }
    }

    let total: usize = lab
        .iter()
        .map(|line| line.iter().filter(|&&c| c == 'X').count())
        .sum();
    println!("{}", total);
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
