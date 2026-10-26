use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

use anyhow::{Error, Result, bail};

fn main() -> Result<()> {
    let (board, path) = parse_input()?;

    println!("Part 1: {}", part1(&board, &path));
    println!("Part 2: {}", part2(&board, &path));

    Ok(())
}

fn parse_input() -> Result<(Board, Vec<Move>)> {
    let mut lines = stdin().lock().lines();

    let mut cells = Vec::new();
    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        cells.push(line.chars().collect());
    }

    let mut path = Vec::new();
    let Some(Ok(line)) = lines.next() else {
        bail!("path line not read")
    };
    let mut remaining = line.as_str();
    while !remaining.is_empty() {
        if remaining.starts_with(['L', 'R']) {
            path.push(remaining[0..1].parse()?);
            remaining = &remaining[1..];
        } else {
            match remaining.find(['L', 'R']) {
                Some(i) => {
                    path.push(remaining[0..i].parse()?);
                    remaining = &remaining[i..];
                }
                None => {
                    path.push(remaining.parse()?);
                    remaining = "";
                }
            }
        }
    }

    Ok((Board { cells }, path))
}

fn part1(board: &Board, path: &[Move]) -> usize {
    password(board.follow(path, |pos| board.step_flat(pos)))
}

fn part2(board: &Board, path: &[Move]) -> usize {
    let cube = board.fold();
    password(board.follow(path, |pos| board.step_cube(&cube, pos)))
}

fn password(Position { x, y, direction }: Position) -> usize {
    [1000 * (x + 1), 4 * (y + 1), direction].iter().sum()
}

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<char>>,
}

#[derive(Debug)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

// Right, Down, Left, Up
const D_DIFF: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    // Index into D_DIFF
    direction: usize,
}

impl Board {
    fn follow(&self, path: &[Move], step: impl Fn(Position) -> Position) -> Position {
        let (x, y) = self.start();
        let mut pos = Position { x, y, direction: 0 };

        for mv in path.iter() {
            match mv {
                Move::Left => pos.direction = (pos.direction as isize - 1).rem_euclid(4) as usize,
                Move::Right => pos.direction = (pos.direction as isize + 1).rem_euclid(4) as usize,
                &Move::Forward(spaces) => {
                    for _ in 0..spaces {
                        let next = step(pos);
                        if self.cells[next.x][next.y] == '#' {
                            break;
                        }
                        pos = next;
                    }
                }
            }
        }

        pos
    }

    fn step_flat(&self, pos: Position) -> Position {
        let (xd, yd) = D_DIFF[pos.direction];
        let (x, y) = self.wrap((pos.x as isize + xd, pos.y as isize + yd), (xd, yd));
        Position { x, y, ..pos }
    }

    fn step_cube(&self, cube: &Cube, pos: Position) -> Position {
        let (xd, yd) = D_DIFF[pos.direction];
        // Faces adjacent in the net are also adjacent on the cube
        match self.in_bounds((pos.x as isize + xd, pos.y as isize + yd)) {
            Some((x, y)) => Position { x, y, ..pos },
            None => cube.wrap(pos),
        }
    }

    // Walk the net, assigning each face its orientation on the cube
    fn fold(&self) -> Cube {
        let tiles = self.cells.iter().flatten().filter(|&&c| c != ' ').count();
        let size = ((tiles / 6) as f64).sqrt() as usize;
        let (x, y) = self.start();
        let mut faces = vec![Face {
            x,
            y: y / size * size,
            normal: [0, 0, 1],
            right: [1, 0, 0],
            down: [0, 1, 0],
        }];

        let mut stack = vec![0];
        while let Some(i) = stack.pop() {
            let face = faces[i];
            for (direction, (xd, yd)) in D_DIFF.iter().enumerate() {
                let next = (
                    face.x as isize + xd * size as isize,
                    face.y as isize + yd * size as isize,
                );
                let Some((x, y)) = self.in_bounds(next) else {
                    continue;
                };
                if faces.iter().any(|f| (f.x, f.y) == (x, y)) {
                    continue;
                }
                let Face {
                    normal,
                    right,
                    down,
                    ..
                } = face;
                // Roll the cube over the shared edge
                let (normal, right, down) = match direction {
                    0 => (right, neg(normal), down),
                    1 => (down, right, neg(normal)),
                    2 => (neg(right), normal, down),
                    _ => (neg(down), right, normal),
                };
                faces.push(Face {
                    x,
                    y,
                    normal,
                    right,
                    down,
                });
                stack.push(faces.len() - 1);
            }
        }

        Cube { size, faces }
    }

    fn start(&self) -> (usize, usize) {
        for (x, row) in self.cells.iter().enumerate() {
            for (y, &c) in row.iter().enumerate() {
                if c == '.' {
                    return (x, y);
                }
            }
        }
        panic!("no starting space found")
    }

    fn wrap(&self, (x, y): (isize, isize), (xd, yd): (isize, isize)) -> (usize, usize) {
        let (rows, cols) = (
            self.cells.len() as isize,
            self.cells.iter().map(|r| r.len()).max().unwrap() as isize,
        );
        let (mut x, mut y) = (x, y);
        loop {
            if let Some((x, y)) = self.in_bounds((x, y)) {
                return (x, y);
            }
            x = (x + xd).rem_euclid(rows);
            y = (y + yd).rem_euclid(cols);
        }
    }

    fn in_bounds(&self, (x, y): (isize, isize)) -> Option<(usize, usize)> {
        if x >= 0
            && (x as usize) < self.cells.len()
            && y >= 0
            && (y as usize) < self.cells[x as usize].len()
            && matches!(self.cells[x as usize][y as usize], '.' | '#')
        {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(match s {
            "L" => Move::Left,
            "R" => Move::Right,
            spaces => Move::Forward(spaces.parse()?),
        })
    }
}

type Vec3 = [i32; 3];

fn neg(v: Vec3) -> Vec3 {
    v.map(|c| -c)
}

fn add(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn scale(v: Vec3, k: i32) -> Vec3 {
    v.map(|c| c * k)
}

fn dot(a: Vec3, b: Vec3) -> i32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[derive(Debug, Clone, Copy)]
struct Face {
    // Top-left cell of the face in the net
    x: usize,
    y: usize,
    // Outward normal, and the 3D directions of increasing y and x
    normal: Vec3,
    right: Vec3,
    down: Vec3,
}

impl Face {
    // 3D vectors matching each D_DIFF direction
    fn directions(&self) -> [Vec3; 4] {
        [self.right, self.down, neg(self.right), neg(self.down)]
    }
}

#[derive(Debug)]
struct Cube {
    size: usize,
    faces: Vec<Face>,
}

impl Cube {
    // Step off the edge of a face onto the adjacent one. Cell centers are placed on a
    // cube of side 2 * size centered on the origin, so all coordinates are integers.
    fn wrap(&self, pos: Position) -> Position {
        let n = self.size as i32;
        let from = self
            .faces
            .iter()
            .find(|f| {
                (f.x..f.x + self.size).contains(&pos.x) && (f.y..f.y + self.size).contains(&pos.y)
            })
            .unwrap();
        let (a, b) = ((pos.x - from.x) as i32, (pos.y - from.y) as i32);
        let point = add(
            scale(from.normal, n),
            add(
                scale(from.right, 2 * b + 1 - n),
                scale(from.down, 2 * a + 1 - n),
            ),
        );

        let travel = from.directions()[pos.direction];
        let point = add(point, add(travel, neg(from.normal)));
        let to = self.faces.iter().find(|f| f.normal == travel).unwrap();

        Position {
            x: to.x + ((dot(point, to.down) + n - 1) / 2) as usize,
            y: to.y + ((dot(point, to.right) + n - 1) / 2) as usize,
            direction: to
                .directions()
                .iter()
                .position(|&d| d == neg(from.normal))
                .unwrap(),
        }
    }
}
