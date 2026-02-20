use anyhow::{Result, anyhow};
use std::io::stdin;

fn main() -> Result<()> {
    let diagram = parse_input()?;

    let (path, steps) = diagram.follow_path()?;
    println!("Part 1: {path}");
    println!("Part 2: {steps}");

    Ok(())
}

fn parse_input() -> Result<Diagram> {
    let cells = stdin()
        .lines()
        .map(|l| l.map(|line| line.trim_end().chars().collect::<Vec<_>>()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Diagram { cells })
}

#[derive(Debug)]
struct Diagram {
    cells: Vec<Vec<char>>,
}

impl Diagram {
    fn follow_path(&self) -> Result<(String, usize)> {
        let start = self
            .cells
            .first()
            .and_then(|row| row.iter().position(|&c| c == '|'))
            .ok_or_else(|| anyhow!("Unable to locate starting position in diagram"))?;

        let mut current = Position::new((0, start), Direction::Down);
        let mut path = String::new();
        // Count starting position as a step
        let mut steps = 1;
        loop {
            let next = current.next();
            if !self.in_bounds(next) {
                break;
            }

            current.coords = (next.0 as usize, next.1 as usize);

            let cell = self.cells[current.coords.0][current.coords.1];
            match cell {
                c if c.is_ascii_alphabetic() => path.push(c),
                '+' => {
                    for &dir in ORIENTATIONS.iter() {
                        if dir == current.direction.opposite() {
                            continue;
                        }
                        let peek = Position::new(current.coords, dir);
                        let peek_next = peek.next();
                        if !self.in_bounds(peek_next) {
                            continue;
                        }
                        let peek_next = (peek_next.0 as usize, peek_next.1 as usize);
                        let peek_cell = self.cells[peek_next.0][peek_next.1];
                        if peek_cell == dir.cell_type() || peek_cell.is_ascii_alphabetic() {
                            current.direction = dir;
                            break;
                        }
                    }
                }
                ' ' => break,
                _ => {}
            }

            steps += 1;
        }

        Ok((path, steps))
    }

    fn in_bounds(&self, coords: (isize, isize)) -> bool {
        let (i, j) = coords;
        i >= 0
            && i < self.cells.len() as isize
            && j >= 0
            && j < self.cells[i as usize].len() as isize
    }
}

struct Position {
    coords: (usize, usize),
    direction: Direction,
}

impl Position {
    fn new(coords: (usize, usize), direction: Direction) -> Self {
        Self { coords, direction }
    }

    fn next(&self) -> (isize, isize) {
        self.direction.apply(self.coords)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, coords: (usize, usize)) -> (isize, isize) {
        let (i, j) = (coords.0 as isize, coords.1 as isize);
        match self {
            Direction::Up => (i - 1, j),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
        }
    }

    fn cell_type(&self) -> char {
        match self {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

const ORIENTATIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
