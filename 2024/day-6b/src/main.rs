use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let mut lab: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let start = lab
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|&c| c == '^').map(|j| (i, j)))
        .unwrap();

    let mut total = 0;
    for i in 0..lab.len() {
        for j in 0..lab[i].len() {
            if (start.0 == i && start.1 == j) || lab[i][j] == '#' {
                continue;
            }
            let mut new_lab = lab.clone();
            new_lab[i][j] = '#';
            if in_loop(&mut new_lab, &start) {
                total += 1;
            }
        }
    }
    println!("{}", total);
    Ok(())
}

fn in_loop(lab: &mut Vec<Vec<char>>, start: &(usize, usize)) -> bool {
    let mut direction = Direction::Up;
    let mut found_loop = false;
    let mut current = *start;
    loop {
        if let None = Direction::from_label(&lab[current.0][current.1]) {
            lab[current.0][current.1] = direction.label();
        }

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

        if lab[next.0][next.1] == direction.label() {
            found_loop = true;
            break;
        }

        match lab[next.0][next.1] {
            '#' => direction = direction.turn_right(),
            _ => {
                current = next;
            }
        }
    }
    found_loop
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

    fn label(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    fn from_label(label: &char) -> Option<Direction> {
        match *label {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }
}
