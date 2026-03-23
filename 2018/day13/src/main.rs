use anyhow::Result;
use std::io::stdin;

fn main() -> Result<()> {
    let mut mine = parse_input()?;
    mine.run_until_one();
    Ok(())
}

fn parse_input() -> Result<Mine> {
    let mut tracks = Vec::new();
    let mut carts = Vec::new();

    for (y, line) in stdin().lines().enumerate() {
        let line = line?;
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' | 'v' | '>' | '<' => {
                    carts.push(Cart {
                        x,
                        y,
                        dir: c,
                        turn: Turn::Left,
                        dead: false,
                    });
                    row.push(match c {
                        '^' | 'v' => '|',
                        _ => '-',
                    });
                }
                _ => row.push(c),
            }
        }
        tracks.push(row);
    }

    Ok(Mine { tracks, carts })
}

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    dir: char,
    turn: Turn,
    dead: bool,
}

#[derive(Debug)]
struct Mine {
    carts: Vec<Cart>,
    tracks: Vec<Vec<char>>,
}

impl Mine {
    fn run_until_one(&mut self) {
        let mut first_crash = false;

        while self.carts.len() != 1 {
            self.carts.sort_by_key(|c| (c.y, c.x));

            for i in 0..self.carts.len() {
                if self.carts[i].dead {
                    continue;
                }

                match self.carts[i].dir {
                    '^' => self.carts[i].y -= 1,
                    'v' => self.carts[i].y += 1,
                    '<' => self.carts[i].x -= 1,
                    '>' => self.carts[i].x += 1,
                    _ => {}
                }

                let (nx, ny) = (self.carts[i].x, self.carts[i].y);

                for j in 0..self.carts.len() {
                    if i != j
                        && !self.carts[j].dead
                        && self.carts[j].x == nx
                        && self.carts[j].y == ny
                    {
                        self.carts[i].dead = true;
                        self.carts[j].dead = true;
                        if !first_crash {
                            println!("Part 1: {},{}", nx, ny);
                            first_crash = true;
                        }
                        break;
                    }
                }

                if self.carts[i].dead {
                    continue;
                }

                let track = self.tracks[ny][nx];
                let dir = self.carts[i].dir;

                match (track, dir) {
                    ('/', '^') => self.carts[i].dir = '>',
                    ('/', '<') => self.carts[i].dir = 'v',
                    ('/', 'v') => self.carts[i].dir = '<',
                    ('/', '>') => self.carts[i].dir = '^',
                    ('\\', '^') => self.carts[i].dir = '<',
                    ('\\', '>') => self.carts[i].dir = 'v',
                    ('\\', 'v') => self.carts[i].dir = '>',
                    ('\\', '<') => self.carts[i].dir = '^',
                    ('+', _) => {
                        self.carts[i].dir = self.carts[i].turn.as_dir(dir);
                        self.carts[i].turn = self.carts[i].turn.next();
                    }
                    _ => {}
                }
            }

            self.carts.retain(|c| !c.dead);
        }

        if let Some(last) = self.carts.first() {
            println!("Part 2: {},{}", last.x, last.y);
        } else {
            panic!("Expected one cart!")
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Left => Self::Straight,
            Turn::Straight => Self::Right,
            Turn::Right => Self::Left,
        }
    }

    fn as_dir(&self, dir: char) -> char {
        match (self, dir) {
            (Self::Left, '^') => '<',
            (Self::Left, '>') => '^',
            (Self::Left, 'v') => '>',
            (Self::Left, '<') => 'v',
            (Self::Right, '^') => '>',
            (Self::Right, '>') => 'v',
            (Self::Right, 'v') => '<',
            (Self::Right, '<') => '^',
            _ => dir,
        }
    }
}
