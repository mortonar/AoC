use anyhow::{Context, Result};
use regex::Regex;
use std::{env, io};

fn main() -> Result<()> {
    // Size is given in width(y) by height(X) but it makes more sense in my head flipped.
    let args: Vec<String> = env::args().collect();
    let grid_size: (usize, usize) = if args.len() >= 2 {
        (args[2].parse()?, args[1].parse()?)
    } else {
        (103, 101)
    };

    let parser = Parser::new()?;
    let mut robots = Vec::new();
    for line in io::stdin().lines() {
        robots.push(parser.parse(&line?)?);
    }

    for _ in 0..100 {
        robots.iter_mut().for_each(|r| r.advance(grid_size));
    }
    print_robots(grid_size, &robots);

    let bisects = (grid_size.0 / 2, grid_size.1 / 2);
    let mut quads = [0, 0, 0, 0];
    for r in robots.iter() {
        if r.coords.0 < bisects.0 {
            if r.coords.1 < bisects.1 {
                quads[0] += 1;
            } else if r.coords.1 > bisects.1 {
                quads[1] += 1;
            }
        } else if r.coords.0 > bisects.0 {
            if r.coords.1 < bisects.1 {
                quads[2] += 1;
            } else if r.coords.1 > bisects.1 {
                quads[3] += 1;
            }
        }
    }
    let safety_factor: u64 = quads.iter().product();
    println!("{safety_factor}");

    Ok(())
}

fn print_robots(grid_size: (usize, usize), robots: &[Robot]) {
    for i in 0..grid_size.0 {
        for j in 0..grid_size.1 {
            let mut sum = 0;
            robots
                .iter()
                .filter(|r| r.coords == (i, j))
                .for_each(|_| sum += 1);
            if sum >= 1 {
                print!("{sum} ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

struct Parser {
    regex: Regex,
}

impl Parser {
    fn new() -> Result<Self> {
        Ok(Parser {
            regex: Regex::new(r"^p=(\d{1,3}),(\d{1,3}) v=(-?\d{1,3}),(-?\d{1,3})$")?,
        })
    }

    fn parse(&self, line: &str) -> Result<Robot> {
        // Flip x,y here too
        let (_full, [y, x, vy, vx]) = self
            .regex
            .captures(line.as_ref())
            .context("Regex doesn't match")?
            .extract();
        Ok(Robot::new(
            (x.parse()?, y.parse()?),
            (vx.parse()?, vy.parse()?),
        ))
    }
}

#[derive(Debug)]
struct Robot {
    coords: (usize, usize),
    velocity: (isize, isize),
}

impl Robot {
    fn new(coords: (usize, usize), velocity: (isize, isize)) -> Self {
        Robot { coords, velocity }
    }

    fn advance(&mut self, grid_size: (usize, usize)) {
        self.coords.0 = Self::wrap_add(self.coords.0, self.velocity.0, grid_size.0);
        self.coords.1 = Self::wrap_add(self.coords.1, self.velocity.1, grid_size.1);
    }

    fn wrap_add(val: usize, vel: isize, grid_limit: usize) -> usize {
        let new_val = val as isize + vel;
        if new_val < 0 {
            grid_limit - new_val.abs() as usize
        } else if new_val > (grid_limit - 1) as isize {
            new_val as usize - (grid_limit - 1) - 1
        } else {
            new_val as usize
        }
    }
}
