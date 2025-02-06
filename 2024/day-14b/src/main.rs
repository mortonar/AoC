use anyhow::{Context, Result};
use regex::Regex;
use std::cmp::max;
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

    let mut i = 0;
    loop {
        i += 1;
        robots.iter_mut().for_each(|r| r.advance(grid_size));
        if is_tree(&robots) {
            println!("{i} seconds:");
            print_robots(grid_size, &robots);
            break;
        }
    }

    Ok(())
}

// Instead of looking for an exact tree shape, look for tree height/trunk.
fn is_tree(robots: &[Robot]) -> bool {
    let mut y_coords: Vec<usize> = robots.iter().map(|r| r.coords.1).collect();
    y_coords.sort();
    y_coords.dedup();

    for y in y_coords {
        let mut x_at_y: Vec<usize> = robots
            .iter()
            .filter(|r| r.coords.1 == y)
            .map(|r| r.coords.0)
            .collect();
        x_at_y.sort();
        x_at_y.dedup();

        if largest_contiguous_seq(&x_at_y) >= 23 {
            return true;
        }
    }

    false
}

fn largest_contiguous_seq(x_coords: &[usize]) -> usize {
    let mut largest = 1;
    let mut current = 1;
    let mut p = x_coords.first().unwrap();
    for n in x_coords.iter().skip(1) {
        if *n != *p + 1 {
            current = 0;
        }
        current += 1;
        largest = max(largest, current);
        p = n;
    }

    largest
}

#[test]
fn test_largest_contiguous_seq() {
    assert_eq!(largest_contiguous_seq(&[1, 2, 3, 7]), 3);
    assert_eq!(
        largest_contiguous_seq(&[1, 2, 3, 7, 8, 9, 10, 11, 12, 13]),
        7
    );
    assert_eq!(largest_contiguous_seq(&[1, 3, 5, 7]), 1);
    assert_eq!(
        largest_contiguous_seq(&[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
        ]),
        23
    );
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
