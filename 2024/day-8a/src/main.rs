use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::stdin;
use std::ops::{Add, Sub};

fn main() -> Result<()> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (i, l) in stdin().lines().enumerate() {
        for (j, c) in l?.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push(Point::new(i as isize, j as isize))
            }
            max_x = i;
            max_y = j;
        }
    }
    let max_x = max_x as isize;
    let max_y = max_y as isize;

    let mut locations = HashSet::new();
    for positions in antennas.values() {
        for pair in positions.iter().combinations(2) {
            for a in pair[0].antinodes(pair[1]) {
                if a.in_bounds(max_x, max_y) {
                    locations.insert(a);
                }
            }
        }
    }
    println!("{}", locations.len());

    Ok(())
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, rhs: &'b Point) -> Self::Output {
        Point {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: &'b Point) -> Self::Output {
        Point {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn antinodes(&self, other: &Point) -> [Point; 2] {
        let diff = self - other;
        [self + &diff, other - &diff]
    }

    fn in_bounds(&self, max_x: isize, max_y: isize) -> bool {
        self.x >= 0 && self.x <= max_x && self.y >= 0 && self.y <= max_y
    }
}
