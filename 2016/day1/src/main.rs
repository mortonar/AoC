use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut coords = Coords::new(0, 0);
    let mut dir = Direction::North;
    let mut visited = HashSet::new();
    let mut twice = None;

    let mut sequence = String::new();
    std::io::stdin().read_line(&mut sequence)?;

    for instruction in sequence.trim().split(", ") {
        dir = dir.turn(&instruction[0..1]);
        let blocks = instruction[1..].parse::<usize>()?;
        coords.walk(blocks, &dir, &mut visited, &mut twice);
    }

    println!("Part 1: {}", coords.distance());
    println!("Part 2: {}", twice.unwrap().distance());

    Ok(())
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn new(x: isize, y: isize) -> Self {
        Coords { x, y }
    }

    fn walk(
        &mut self,
        blocks: usize,
        direction: &Direction,
        visited: &mut HashSet<Coords>,
        twice: &mut Option<Coords>,
    ) {
        let diff = match direction {
            Direction::North => Coords::new(0, 1),
            Direction::South => Coords::new(0, -1),
            Direction::East => Coords::new(1, 0),
            Direction::West => Coords::new(-1, 0),
        };
        let mut next = self.clone();
        while self.distance_from(&next) < blocks {
            let new = visited.insert(next.clone());
            if !new && twice.is_none() {
                *twice = Some(next.clone());
            }

            next.x += diff.x;
            next.y += diff.y;
        }
        self.x = next.x;
        self.y = next.y;
    }

    fn distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn distance_from(&self, other: &Coords) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self, direction: &str) -> Self {
        match self {
            Direction::North => {
                if direction == "R" {
                    Direction::East
                } else {
                    Direction::West
                }
            }
            Direction::East => {
                if direction == "R" {
                    Direction::South
                } else {
                    Direction::North
                }
            }
            Direction::South => {
                if direction == "R" {
                    Direction::West
                } else {
                    Direction::East
                }
            }
            Direction::West => {
                if direction == "R" {
                    Direction::North
                } else {
                    Direction::South
                }
            }
        }
    }
}
